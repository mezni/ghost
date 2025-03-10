// src/main.rs
use pest::Parser;
use pest_derive::Parser;

// Define the parser struct
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MgsvpParser;

fn main() {
    let pairs = MgsvpParser::parse(Rule::file, "<mgsvp;Hello, World!END")
        .unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        println!("{:?}", pair);
    }
}