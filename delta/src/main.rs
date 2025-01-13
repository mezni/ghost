use deltalake::{open_table, DeltaTableError};

#[tokio::main]
async fn main() -> Result<(), DeltaTableError> {
    // Open the Delta table
    let table = open_table("./data/delta").await?;

    // Collect all active file URIs in the table
    let files: Vec<_> = table.get_file_uris().collect();
    println!("Active files: {:?}", files);

    Ok(())
}
