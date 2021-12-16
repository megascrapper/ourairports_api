use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::csv_web_request_blocking;

const AIRPORTS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/countries.csv";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Country {
    id: String,
    code: String,
    name: String,
    continent: String,
    wikipedia_link: String,
    keywords: Vec<String>,
}

impl Country {
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn continent(&self) -> &str {
        &self.continent
    }
    pub fn wikipedia_link(&self) -> &str {
        &self.wikipedia_link
    }
    pub fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Country {}

impl Ord for Country {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Country {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Country {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Returns a `HashMap` of countries  from `countries.csv` with the country id as key and `Country`
/// struct as value
pub fn get_countries_csv(data_path: Option<std::path::PathBuf>) -> HashMap<String, Country> {
    let content = if let Some(data_path) = data_path {
        // open file
        fs::read_to_string(data_path).expect("could not open file")
    } else {
        csv_web_request_blocking(AIRPORTS_CSV_URL).expect("web request error")
    };
    // initialise csv reader & return value;
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut country_map = HashMap::new();
    for result in rdr.deserialize() {
        let record: Country = result.expect("error in deserializing");
        country_map.insert(record.id().clone().to_owned(), record);
    }
    country_map
}