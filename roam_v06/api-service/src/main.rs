// src/main.rs
mod config; // Makes the config.rs module available
mod errors; // Makes the errors.rs module available

fn main() -> Result<(), errors::AppError> {
    // Load the server configuration
    let server_config = config::load_config()?;

    // Print out the loaded configuration for verification
    println!("--- Service Configuration ---");
    println!("Host: {}", server_config.service.host);
    println!("Port: {}", server_config.service.port);
    println!("CORS Allowed Origins: {:?}", server_config.service.cors);
    println!("\n--- Database Configuration ---");
    println!("DB Host: {}", server_config.database.host);
    println!("DB Port: {}", server_config.database.port);
    println!("DB User: {}", server_config.database.user);
    println!("DB Name: {}", server_config.database.name);
    // You typically wouldn't print the password in a real app,
    // but useful for initial debugging:
    // println!("DB Password: {}", server_config.database.password);

    // --- Your application's main logic would go here ---
    // For example, starting a web server or connecting to the database:
    // let db_pool = my_db_crate::connect(&server_config.database)?;
    // my_web_server_crate::start(server_config.service.host, server_config.service.port, db_pool)?;

    println!("\nApplication started successfully with loaded configuration!");

    Ok(())
}