use std::fs::File;
use std::io::{self, BufReader, Read};
use uuid::Uuid;

fn main() -> io::Result<()> {
    // Specify the line terminator (can be \n or \r\n depending on your file's line endings)
    let terminator = "\n"; 

    // Path to the large file
    let file_path = "example.csv";

    // Open the file
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    // Define the chunk size (e.g., 8 KB)
    let chunk_size = 8192;

    // Process the file in chunks and handle leftover data between chunks
    let processed_lines = process_file_in_chunks(&mut reader, terminator, chunk_size)?;

    // Print the processed lines with their UUIDs
    for (id, line) in processed_lines {
        println!("ID: {}, Line: {}", id, line);
    }

    Ok(())
}

fn process_file_in_chunks<R: Read>(reader: &mut R, terminator: &str, chunk_size: usize) -> io::Result<Vec<(Uuid, String)>> {
    let mut buffer = vec![0; chunk_size];
    let mut leftover = String::new();
    let mut processed_lines = Vec::new();

    loop {
        // Read a chunk from the file
        let bytes_read = reader.read(&mut buffer)?;

        // Break the loop if we've reached the end of the file
        if bytes_read == 0 {
            break;
        }

        // Convert the chunk to a string and prepend any leftover data
        let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut data = leftover.clone(); // Clone leftover to keep previous state
        data.push_str(&chunk); // Append the new chunk to the data

        // Process the complete lines and collect them with UUIDs
        leftover = process_lines(&data, terminator, &mut processed_lines)?;
    }

    Ok(processed_lines)
}

/// Process the lines in the data chunk, returning the leftover (incomplete) line if any
fn process_lines(data: &str, terminator: &str, processed_lines: &mut Vec<(Uuid, String)>) -> io::Result<String> {
    let mut leftover = String::new();
    let mut lines = data.split(terminator).peekable();

    // Process all complete lines
    while let Some(line) = lines.next() {
        if lines.peek().is_none() {
            // Save the last incomplete line as leftover
            leftover = line.to_string();
        } else {
            // Create a UUID for each line and store it with the line content
            let id = Uuid::new_v4();
            processed_lines.push((id, line.to_string()));
        }
    }

    Ok(leftover)
}
