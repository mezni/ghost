// generate_hashes.rs
use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let passwords = vec![
        "superadmin123",
        "admin123", 
        "operator123",
        "viewer123"
    ];

    for password in passwords {
        match hash(password, DEFAULT_COST) {
            Ok(hashed) => println!("Password: {} -> Hash: {}", password, hashed),
            Err(e) => println!("Error hashing {}: {}", password, e),
        }
    }
}