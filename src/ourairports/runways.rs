use crate::ourairports::{bool_from_str, FetchError, Id, ToJsonString};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

const RUNWAYS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/runways.csv";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Runway {
    id: Id,
    airport_ref: Id,
    airport_ident: String,
    length_ft: Option<i32>,
    width_ft: Option<i32>,
    surface: String,
    #[serde(deserialize_with = "bool_from_str")]
    lighted: bool,
    #[serde(deserialize_with = "bool_from_str")]
    closed: bool,
    le_ident: String,
    le_latitude_deg: Option<f64>,
    le_longitude_deg: Option<f64>,
    le_elevation_ft: Option<i32>,
    #[serde(rename = "le_heading_degT")]
    le_heading_deg_true: Option<f64>,
    le_displaced_threshold_ft: Option<i32>,
    he_ident: String,
    he_latitude_deg: Option<f64>,
    he_longitude_deg: Option<f64>,
    he_elevation_ft: Option<i32>,
    #[serde(rename = "he_heading_degT")]
    he_heading_deg_true: Option<f64>,
    he_displaced_threshold_ft: Option<i32>,
}

impl Runway {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn airport_ref(&self) -> Id {
        self.airport_ref
    }
    pub fn airport_ident(&self) -> &str {
        &self.airport_ident
    }
    pub fn length_ft(&self) -> Option<i32> {
        self.length_ft
    }
    pub fn width_ft(&self) -> Option<i32> {
        self.width_ft
    }
    pub fn surface(&self) -> &str {
        &self.surface
    }
    pub fn lighted(&self) -> bool {
        self.lighted
    }
    pub fn closed(&self) -> bool {
        self.closed
    }
    pub fn le_ident(&self) -> &str {
        &self.le_ident
    }
    pub fn le_latitude_deg(&self) -> Option<f64> {
        self.le_latitude_deg
    }
    pub fn le_longitude_deg(&self) -> Option<f64> {
        self.le_longitude_deg
    }
    pub fn le_elevation_ft(&self) -> Option<i32> {
        self.le_elevation_ft
    }
    pub fn le_heading_deg_true(&self) -> Option<f64> {
        self.le_heading_deg_true
    }
    pub fn le_displaced_threshold_ft(&self) -> Option<i32> {
        self.le_displaced_threshold_ft
    }
    pub fn he_ident(&self) -> &str {
        &self.he_ident
    }
    pub fn he_latitude_deg(&self) -> Option<f64> {
        self.he_latitude_deg
    }
    pub fn he_longitude_deg(&self) -> Option<f64> {
        self.he_longitude_deg
    }
    pub fn he_elevation_ft(&self) -> Option<i32> {
        self.he_elevation_ft
    }
    pub fn he_heading_deg_true(&self) -> Option<f64> {
        self.he_heading_deg_true
    }
    pub fn he_displaced_threshold_ft(&self) -> Option<i32> {
        self.he_displaced_threshold_ft
    }
}

impl PartialEq for Runway {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Runway {}

impl Ord for Runway {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Runway {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Runway {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ToJsonString for Runway {}

pub fn get_runways_csv() -> Result<BTreeMap<Id, Runway>, FetchError> {
    // get data
    let content = crate::web_request_blocking(RUNWAYS_CSV_URL)?;
    // initialise csv reader & return value
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Runway = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
