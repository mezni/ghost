pub mod errors;
pub mod store;

use errors::AppError;

/// An asynchronous function that returns a shared message.
pub async fn shared_function() -> Result<String, AppError> {
    Ok("Hello from the common library!".into())
}
