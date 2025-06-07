mod domain;

use domain::countries::Country;

fn main() {
    let country = Country::builder("   france   ", "fr")
        .created_by("system")
        .build();

    println!("{:#?}", country);
}
