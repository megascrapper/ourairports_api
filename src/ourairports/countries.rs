use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::ourairports::{Id, vec_string_from_string, ToJsonString};

const COUNTRIES_CSV: &str = "countries.csv";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Country {
    id: Id,
    code: String,
    name: String,
    continent: String,
    wikipedia_link: String,
    #[serde(deserialize_with = "vec_string_from_string")]
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

impl ToJsonString for Country {}

/// Returns a `HashMap` of countries from `countries.csv` with the country id as key and `Country`
/// struct as value
pub fn get_countries_csv(data_path: &PathBuf) -> crate::ourairports::Result<HashMap<String, Country>> {
    let mut file_path = PathBuf::from(&data_path);
    file_path.push(COUNTRIES_CSV);
    // open file
    let content = fs::read_to_string(file_path)?;
    // initialise csv reader & return value
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut country_map = HashMap::new();
    for result in rdr.deserialize() {
        let record: Country = result?;
        country_map.insert(record.id().to_owned(), record);
    }
    Ok(country_map)
}