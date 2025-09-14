use bcrypt::{hash, DEFAULT_COST};
use std::io::{self, Write};

fn main() {
    println!("🔐 Enter the string to hash:");

    // Prompt user for input
    print!("> ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("❌ Error reading input: {}", e);
        return;
    }

    let input = input.trim(); // Remove newline

    // Hash the input using bcrypt
    match hash(input, DEFAULT_COST) {
        Ok(hashed) => println!("\n✅ Bcrypt hash: {}", hashed),
        Err(e) => eprintln!("❌ Failed to hash string: {}", e),
    }
}

