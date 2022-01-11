use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::ourairports::{vec_string_from_string, Continent, Id, ToJsonString};

const REGIONS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/regions.csv";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Region {
    id: Id,
    code: String,
    local_code: String,
    name: String,
    continent: Continent,
    iso_country: String,
    wikipedia_link: String,
    #[serde(deserialize_with = "vec_string_from_string")]
    keywords: Vec<String>,
}

impl Region {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn code(&self) -> &str {
        &self.code
    }
    pub fn local_code(&self) -> &str {
        &self.local_code
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn continent(&self) -> &Continent {
        &self.continent
    }
    pub fn iso_country(&self) -> &str {
        &self.iso_country
    }
    pub fn wikipedia_link(&self) -> &str {
        &self.wikipedia_link
    }
    pub fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }
}

impl PartialEq for Region {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Region {}

impl Ord for Region {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Region {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Region {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ToJsonString for Region {}

pub fn get_regions_csv() -> crate::ourairports::Result<BTreeMap<Id, Region>> {
    // get data
    let content = crate::web_request_blocking(REGIONS_CSV_URL)?;
    // initialise csv reader & return value
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Region = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
