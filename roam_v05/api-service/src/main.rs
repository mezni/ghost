// src/main.rs
mod config; // This line makes the config.rs module available
mod errors; // This line makes the errors.rs module available

fn main() -> Result<(), errors::AppError> { // Now uses errors::AppError as the return type



    let server_config = config::load_config()?; // The `?` operator will now convert config::ConfigError to errors::AppError
    println!("Server Config: {:?}", server_config.service);
    println!("Database Config: {:?}", server_config.database);

    // Your application logic here, using server_config.service.port, etc.

    Ok(())
}
