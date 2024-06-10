use shamirsecretsharing::*;
use std::io::{self, Write};

// Define the size of the data
const DATA_SIZE: usize = 64;

fn main() {
    // Custom secret string
    let secret_string = "Azhan is my creator.";
    
    // Convert the secret string to bytes
    let mut data = secret_string.as_bytes().to_vec();
    
    // Ensure the data is of the required length (padded with zeros if necessary)
    data.resize(DATA_SIZE, 0);
    
    // Number of shares to create
    let count = 5;
    
    // Minimum number of shares required to reconstruct the secret
    let threshold = 3;
    
    // Create shares
    let shares = create_shares(&data, count, threshold).unwrap();

    // Display the generated shares
    println!("Generated Shares (Copy and paste these as needed):");
    for (i, share) in shares.iter().enumerate() {
        print!("Share {}: ", i + 1);
        for byte in share {
            print!("{} ", byte);
        }
        println!();
    }

    // Collect user input for shares
    let mut selected_shares = Vec::new();
    for i in 1..=threshold {
        println!("Enter Share {}: (format x y1 y2 ... yn)", i);
        let mut share_input = String::new();
        io::stdin().read_line(&mut share_input).unwrap();
        let parts: Vec<u8> = share_input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        selected_shares.push(parts);
    }

    // Convert selected shares to the format required by combine_shares
    // Convert from Vec<&[u8]> to Vec<Vec<u8>>
    let selected_shares: Vec<Vec<u8>> = selected_shares.iter().map(|s| s.clone()).collect();

    // Reconstruct the secret with the provided shares
    match combine_shares(&selected_shares) {
        Ok(Some(restored)) => {
            let restored_string = String::from_utf8(restored).unwrap();
            println!("Restored secret: {}", restored_string);
        }
        _ => {
            println!("Failed to reconstruct the secret.");
        }
    }
}
