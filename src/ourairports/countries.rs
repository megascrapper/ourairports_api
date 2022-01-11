use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::ourairports::{vec_string_from_string, Continent, Id, ToJsonString};

const COUNTRIES_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/countries.csv";

/// Represents a country or country-like entity (e.g. Hong Kong).
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Country {
    id: Id,
    code: String,
    name: String,
    continent: Continent,
    wikipedia_link: String,
    #[serde(deserialize_with = "vec_string_from_string")]
    keywords: Vec<String>,
}

impl Country {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn continent(&self) -> &Continent {
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
pub fn get_countries_csv() -> crate::ourairports::Result<BTreeMap<Id, Country>> {
    // get data
    let content = crate::web_request_blocking(COUNTRIES_CSV_URL)?;
    // initialise csv reader & return value
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Country = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
