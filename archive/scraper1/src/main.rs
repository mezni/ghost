use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

#[derive(Debug)]
struct OperatorInfo {
    country: String,
    rank: String,
    operator: String,
    technology: String,
    subscribers: String,
    ownership: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://en.wikipedia.org/wiki/List_of_mobile_network_operators_in_Asia_and_Oceania";
    let body = get(url)?.text()?;
    let document = Html::parse_document(&body);

    let heading_selector = Selector::parse("h2, h3").unwrap();
    let table_selector = Selector::parse("table.wikitable").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut results = Vec::new();
    let mut current_country = String::new();

    // Iterate all nodes manually to keep track of heading->table
    for node in document.tree.root().children() {
        if let Some(el) = scraper::ElementRef::wrap(node) {
            let tag_name = el.value().name();

            if (tag_name == "h2" || tag_name == "h3") && el.text().next().is_some() {
                // Update current country heading
                current_country = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                current_country = current_country.trim_end_matches("[edit]").trim().to_string();
            }

            if tag_name == "table" && el.value().has_class("wikitable", scraper::CaseSensitivity::AsciiCaseInsensitive) {
                for row in el.select(&row_selector).skip(1) {
                    let cells: Vec<_> = row.select(&cell_selector).collect();
                    if cells.len() >= 5 {
                        let get_text = |i: usize| {
                            cells.get(i)
                                .map(|c| c.text().collect::<Vec<_>>().join(" ").trim().to_string())
                                .unwrap_or_default()
                        };

                        results.push(OperatorInfo {
                            country: current_country.clone(),
                            rank: get_text(0),
                            operator: get_text(1),
                            technology: get_text(2),
                            subscribers: get_text(3),
                            ownership: get_text(4),
                        });
                    }
                }
            }
        }
    }

    // Show results
    for info in &results {
        println!("{:?}", info);
    }

    Ok(())
}
