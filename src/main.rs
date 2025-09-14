use bcrypt::{hash, DEFAULT_COST};
use std::io::{self, Write};

fn main() {
    print!("🔐 Enter the string to hash:\n> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("❌ Error reading input: {}", e);
        return;
    }

    match hash(input.trim(), DEFAULT_COST) {
        Ok(hashed) => println!("\n✅ Bcrypt hash: {}", hashed),
        Err(e) => eprintln!("❌ Failed to hash string: {}", e),
    }
}
