use num_bigint::{BigInt, ToBigInt};
use num_traits::{One, Zero};
use std::str::FromStr;
use rand::Rng;

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

pub fn make_random_shares(secret: i64, minimum: usize, shares: usize, prime: &BigInt) -> Vec<(BigInt, BigInt)> {
    let mut rng = rand::thread_rng();
    let secret_bigint = BigInt::from(secret);
    let mut poly = vec![secret_bigint];

    for _ in 1..minimum {
        poly.push(gen_bigint_below(&mut rng, prime));
    }

    let points: Vec<(BigInt, BigInt)> = (1..=shares).map(|i| {
        let x = BigInt::from(i);
        let y = eval_at(&poly, &x, prime);
        (x, y)
    }).collect();

    points
}

pub fn reconstruct_secret(shares: &[(BigInt, BigInt)], prime: &BigInt) -> BigInt {
    let (x_s, y_s): (Vec<_>, Vec<_>) = shares.iter().cloned().unzip();
    lagrange_interpolate(BigInt::zero(), &x_s, &y_s, prime)
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
