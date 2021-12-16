use std::env::current_dir;
use std::path::PathBuf;
use std::collections::HashMap;

use ourairports_api::ourairports::countries::get_countries_csv;

// Some(PathBuf::from(&argv[1]))

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let countries_data = get_countries_csv(None);
    for (_, country) in countries_data.iter() {
        println!("{}", country.name());
    }
}