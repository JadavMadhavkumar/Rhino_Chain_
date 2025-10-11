rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnumod block;
mod blockchain;

use blockchain::Blockchain;
use std::io::{self, Write};

fn main() {
    println!("===============================================");
    println!("   BIOMETRIC BLOCKCHAIN SYSTEM");
    println!("   Using Rust + Blockchain Security");
    println!("===============================================");

    // Try to load existing blockchain or create a new one
    let mut blockchain = match Blockchain::load_from_file("biometric_chain.json") {
        Ok(chain) => {
            println!("Loaded existing blockchain from file.");
            chain
        },
        Err(_) => {
            println!("Creating new blockchain with difficulty level 4.");
            Blockchain::new(4)
        }
    };

    loop {
        show_menu();
        let choice = get_user_input();

        match choice.trim() {
            "1" => add_biometric_block(&mut blockchain),
            "2" => view_blockchain(&blockchain),
            "3" => validate_blockchain(&blockchain),
            "4" => view_latest_block(&blockchain),
            "5" => {
                match blockchain.save_to_file("biometric_chain.json") {
                    Ok(_) => println!("Blockchain saved successfully!"),
                    Err(e) => println!("Error saving blockchain: {}", e),
                }
            },
            "6" => {
                match blockchain.save_to_file("biometric_chain.json") {
                    Ok(_) => println!("Blockchain saved. Exiting..."),
                    Err(e) => println!("Error saving blockchain: {}", e),
                }
                break;
            },
            _ => println!("Invalid choice. Please try again."),
        }

        println!("\nPress Enter to continue...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}

fn show_menu() {
    println!("\n+--------------------------------------------+");
    println!("|         Biometric Blockchain Menu          |");
    println!("+--------------------------------------------+");
    println!("| 1. Add Biometric Block                     |");
    println!("| 2. View Blockchain                         |");
    println!("| 3. Validate Blockchain                     |");
    println!("| 4. View Latest Block                       |");
    println!("| 5. Save Blockchain                         |");
    println!("| 6. Save and Exit                           |");
    println!("+--------------------------------------------+");
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn add_biometric_block(blockchain: &mut Blockchain) {
    println!("\n--- Adding New Biometric Block ---");
    println!("Enter biometric data (e.g., fingerprint hash, iris scan, etc.):");
    let biometric_data = get_user_input();
    
    println!("Mining new block (this may take a moment)...");
    let timer = std::time::Instant::now();
    
    blockchain.add_block(biometric_data.trim());
    
    let elapsed = timer.elapsed();
    println!("Block mined successfully in {:.2?}", elapsed);
    println!("Block hash: {}", blockchain.get_latest_block().hash);
}

fn view_blockchain(blockchain: &Blockchain) {
    println!("\n--- Blockchain ---");
    println!("Chain length: {} blocks", blockchain.chain.len());
    println!("Mining difficulty: {}", blockchain.difficulty);
    
    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("\nBlock #{}:", i);
        println!("  ID: {}", block.id);
        println!("  Timestamp: {}", block.timestamp);
        println!("  Biometric Data Hash: {}", block.biometric_data_hash);
        println!("  Previous Hash: {}", block.previous_hash);
        println!("  Hash: {}", block.hash);
        println!("  Nonce: {}", block.nonce);
    }
}

fn validate_blockchain(blockchain: &Blockchain) {
    println!("\n--- Blockchain Validation ---");
    let timer = std::time::Instant::now();
    
    let is_valid = blockchain.is_valid();
    
    let elapsed = timer.elapsed();
    if is_valid {
        println!("✓ Blockchain is valid! (Validation took {:.2?})", elapsed);
    } else {
        println!("✗ Blockchain is invalid! (Validation took {:.2?})", elapsed);
    }
}

fn view_latest_block(blockchain: &Blockchain) {
    println!("\n--- Latest Block ---");
    let block = blockchain.get_latest_block();
    println!("Block #{}:", block.index);
    println!("  ID: {}", block.id);
    println!("  Timestamp: {}", block.timestamp);
    println!("  Biometric Data Hash: {}", block.biometric_data_hash);
    println!("  Previous Hash: {}", block.previous_hash);
    println!("  Hash: {}", block.hash);
    println!("  Nonce: {}", block.nonce);
}