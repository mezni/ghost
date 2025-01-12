use chrono::{DateTime, Utc};
use rand::Rng;

// Define a struct to hold CDR data
#[derive(Debug)]
struct Cdr {
    caller_number: String,
    callee_number: String,
    call_duration: u64, // in seconds
    timestamp: DateTime<Utc>,
}

// Implement a function to generate a random CDR
fn generate_cdr() -> Cdr {
    let mut rng = rand::thread_rng();

    // Generate random phone numbers
    let caller_number: String = (0..10).map(|_| rng.gen_range('0'..='9')).collect();
    let callee_number: String = (0..10).map(|_| rng.gen_range('0'..='9')).collect();

    // Generate random call duration (between 1 second and 1 hour)
    let call_duration: u64 = rng.gen_range(1..3600);

    // Generate random timestamp (within the last 24 hours)
    let timestamp: DateTime<Utc> = Utc::now() - chrono::Duration::seconds(rng.gen_range(0..86400));

    Cdr {
        caller_number,
        callee_number,
        call_duration,
        timestamp,
    }
}

fn main() {
    // Generate and print 100 random CDRs
    for i in 1..=100 {
        let cdr = generate_cdr();
        println!("CDR #{}: {:?}", i, cdr);
    }
}