use crate::ourairports::{Id, ToJsonString};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

const AIRPORT_FREQUENCIES_CSV_URL: &str =
    "https://davidmegginson.github.io/ourairports-data/airport-frequencies.csv";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct AirportFrequency {
    id: Id,
    airport_ref: Id,
    airport_ident: String,
    #[serde(rename = "type")]
    frequency_type: String, // TODO: make this an enum once the vocab is controlled
    description: String,
    frequency_mhz: String,
}

impl AirportFrequency {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn airport_ref(&self) -> Id {
        self.airport_ref
    }
    pub fn airport_ident(&self) -> &str {
        &self.airport_ident
    }
    pub fn frequency_type(&self) -> &str {
        &self.frequency_type
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn frequency_mhz(&self) -> &str {
        &self.frequency_mhz
    }
}

impl PartialEq for AirportFrequency {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for AirportFrequency {}

impl Ord for AirportFrequency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for AirportFrequency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for AirportFrequency {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ToJsonString for AirportFrequency {}

pub fn get_airport_frequencies_csv() -> crate::ourairports::Result<BTreeMap<Id, AirportFrequency>> {
    // get data
    let content = crate::web_request_blocking(AIRPORT_FREQUENCIES_CSV_URL)?;
    // initialise csv reader & return value
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: AirportFrequency = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
