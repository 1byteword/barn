// deprecated

use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::str::FromStr;
use rand::Rng;
use std::io::{self, Write};
use hex;

// 12th Mersenne prime
pub const PRIME: &str = "170141183460469231731687303715884105727";

fn eval_at(poly: &[BigInt], x: &BigInt, prime: &BigInt) -> BigInt {
    let mut accum = BigInt::zero();
    for coeff in poly.iter().rev() {
        accum *= x;
        accum += coeff;
        accum %= prime;
    }
    accum
}

fn gen_bigint_below(rng: &mut rand::rngs::ThreadRng, upper: &BigInt) -> BigInt {
    let mut result = BigInt::zero();
    let bits = upper.bits() as usize;
    while result >= *upper || result.is_zero() {
        let bytes = (bits + 7) / 8;
        let mut v = vec![0u8; bytes];
        rng.fill(&mut v[..]);
        result = BigInt::from_bytes_le(num_bigint::Sign::Plus, &v);
    }
    result
}

pub fn make_random_shares(secret: &[u8], minimum: usize, shares: usize, prime: &BigInt) -> Vec<(BigInt, Vec<u8>)> {
    let mut rng = rand::thread_rng();
    let secret_bigint = BigInt::from_bytes_le(num_bigint::Sign::Plus, secret);
    let mut poly = vec![secret_bigint];

    for _ in 1..minimum {
        poly.push(gen_bigint_below(&mut rng, prime));
    }

    println!("Polynomial coefficients: {:?}", poly);

    let points: Vec<(BigInt, Vec<u8>)> = (1..=shares).map(|i| {
        let x = BigInt::from(i);
        let y = eval_at(&poly, &x, prime);
        println!("Evaluated at {}: {}", i, y);
        (x, y.to_bytes_le().1)
    }).collect();

    points
}

pub fn reconstruct_secret(shares: &[(BigInt, Vec<u8>)], prime: &BigInt) -> Vec<u8> {
    let (x_s, y_s): (Vec<_>, Vec<_>) = shares.iter().map(|(x, y)| (x.clone(), BigInt::from_bytes_le(num_bigint::Sign::Plus, y))).unzip();
    println!("Shares for interpolation: x_s = {:?}, y_s = {:?}", x_s, y_s);
    let secret_bigint = lagrange_interpolate(BigInt::zero(), &x_s, &y_s, prime);
    secret_bigint.to_bytes_le().1
}

fn lagrange_interpolate(x: BigInt, x_s: &[BigInt], y_s: &[BigInt], prime: &BigInt) -> BigInt {
    let k = x_s.len();
    let mut result = BigInt::zero();

    for i in 0..k {
        let mut terms = y_s[i].clone();
        for j in 0..k {
            if i == j {
                continue;
            }
            let num = &x - &x_s[j];
            let denom = &x_s[i] - &x_s[j];
            terms = terms * &num * mod_inv(&denom, prime) % prime;
        }
        result = (result + terms) % prime;
    }

    if result < BigInt::zero() {
        result += prime;
    }

    result
}

fn mod_inv(a: &BigInt, p: &BigInt) -> BigInt {
    let mut mn = (p.clone(), a.clone());
    let mut xy = (BigInt::zero(), BigInt::one());

    while mn.1 != BigInt::zero() {
        let q = &mn.0 / &mn.1;
        mn = (mn.1.clone(), &mn.0 - &q * &mn.1);
        xy = (xy.1.clone(), &xy.0 - &q * &xy.1);
    }

    while xy.0 < BigInt::zero() {
        xy.0 += p;
    }

    xy.0
}

fn main() {
    let prime = BigInt::from_str(PRIME).unwrap();
    // hex for "This is a secret message"
    let secret = "54686973206973206120736563726574206d657373616765";

    let secret_bytes = hex::decode(secret).unwrap();
    println!("Original secret (Hex): {}", secret);
    println!("Original secret (Decimal): {:?}", secret_bytes);

    let minimum = 3;
    let shares = 5;

    let shares = make_random_shares(&secret_bytes, minimum, shares, &prime);
    
    println!("Shares:");
    for (i, share) in shares.iter().enumerate() {
        print!("Share {}: {} ", i + 1, share.0);
        for byte in &share.1 {
            print!("{} ", byte);
        }
        println!();
    }

    // Prompt user to enter 3 shares
    let mut input_shares = Vec::new();
    for i in 1..=3 {
        print!("Enter Share {}: (format x y1 y2 ... yn): ", i);
        io::stdout().flush().unwrap();

        let mut share_input = String::new();
        io::stdin().read_line(&mut share_input).unwrap();
        let parts: Vec<&str> = share_input.trim().split_whitespace().collect();

        if parts.len() >= 2 {
            let x = BigInt::from_str(parts[0]).unwrap();
            let y: Vec<u8> = parts[1..].iter().map(|s| s.parse().unwrap()).collect();
            input_shares.push((x, y));
        } else {
            println!("Invalid input. Please enter the share in format x y1 y2 ... yn");
        }
    }

    let secret_reconstructed_bytes = reconstruct_secret(&input_shares, &prime);

    println!("Reconstructed Bytes (Hex): {:?}", hex::encode(&secret_reconstructed_bytes));
    println!("Reconstructed Bytes (Decimal): {:?}", secret_reconstructed_bytes);
    let secret_reconstructed = hex::encode(&secret_reconstructed_bytes);
    println!("Reconstructed secret (Hex): {}", secret_reconstructed);

    if secret == secret_reconstructed {
        println!("The secret was correctly reconstructed.");
    } else {
        println!("Failed to reconstruct the secret.");
    }
}
