use parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut parser = Parser::parse(Rule::HEADER, &input)?;

    let header = parser.next().unwrap();

    parser = Parser::parse(Rule::BODY, &input[header.as_str().len()..])?;
    let mut records = Vec::new();
    while let Some(record) = parser.next() {
        let mut inner = record.into_inner();
        let hlraddr = inner.next().unwrap().as_str();
        let nsub = inner.next().unwrap().as_str().parse::<i32>()?;
        let nsuba = inner.next().unwrap().as_str().parse::<i32>()?;
        records.push((hlraddr, nsub, nsuba));
    }

    parser = Parser::parse(Rule::FOOTER, &input[header.as_str().len() + records.len() * 3..])?;
    let footer = parser.next().unwrap();

    println!("Header: {}", header.as_str());
    println!("Records:");
    for (hlraddr, nsub, nsuba) in records {
        println!("  {}: {} {}", hlraddr, nsub, nsuba);
    }
    println!("Footer: {}", footer.as_str());

    Ok(())
}