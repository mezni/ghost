use std::fs::File;
use std::io::{self, BufReader, Read};

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
    let line_count = process_file_in_chunks(&mut reader, terminator, chunk_size)?;

    println!("Total number of lines: {}", line_count);

    Ok(())
}

fn process_file_in_chunks<R: Read>(reader: &mut R, terminator: &str, chunk_size: usize) -> io::Result<usize> {
    let mut buffer = vec![0; chunk_size];
    let mut leftover = String::new();
    let mut line_count = 0;

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

        // Process the complete lines based on the specified terminator
        leftover = process_lines(&data, terminator, &mut line_count)?;
    }

    Ok(line_count)
}

/// Process the lines in the data chunk, returning the leftover (incomplete) line if any
fn process_lines(data: &str, terminator: &str, line_count: &mut usize) -> io::Result<String> {
    let mut leftover = String::new();
    let mut lines = data.split(terminator).peekable();

    // Process all complete lines
    while let Some(line) = lines.next() {
        if lines.peek().is_none() {
            // Save the last incomplete line as leftover
            leftover = line.to_string();
        } else {
            // Process the complete line (for now, just increment the line counter)
            *line_count += 1;
            // Uncomment this to print each line if needed:
            // println!("Line: {}", line);
        }
    }

    Ok(leftover)
}
