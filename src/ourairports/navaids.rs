use crate::ourairports::{FetchError, Id, ToJsonString};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use log::debug;

const NAVAIDS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/navaids.csv";

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Navaid {
    id: Id,
    filename: String,
    ident: String,
    name: String,
    #[serde(rename = "type")]
    navaid_type: NavaidType,
    frequency_khz: String,
    latitude_deg: Option<f64>,
    longitude_deg: Option<f64>,
    elevation_ft: Option<i32>,
    iso_country: String,
    dme_frequency_khz: String,
    dme_channel: String,
    dme_latitude_deg: Option<f64>,
    dme_longitude_deg: Option<f64>,
    dme_elevation_ft: Option<i32>,
    slaved_variation_deg: Option<f64>,
    magnetic_variation_deg: Option<f64>,
    #[serde(rename = "usageType")]
    usage_type: Option<UsageType>,
    power: Option<NavaidPower>,
    associated_airport: String,
}

impl Navaid {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn filename(&self) -> &str {
        &self.filename
    }
    pub fn ident(&self) -> &str {
        &self.ident
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn navaid_type(&self) -> &NavaidType {
        &self.navaid_type
    }
    pub fn frequency_khz(&self) -> &str {
        &self.frequency_khz
    }
    pub fn latitude_deg(&self) -> Option<f64> {
        self.latitude_deg
    }
    pub fn longitude_deg(&self) -> Option<f64> {
        self.longitude_deg
    }
    pub fn elevation_ft(&self) -> Option<i32> {
        self.elevation_ft
    }
    pub fn iso_country(&self) -> &str {
        &self.iso_country
    }
    pub fn dme_frequency_khz(&self) -> &str {
        &self.dme_frequency_khz
    }
    pub fn dme_channel(&self) -> &str {
        &self.dme_channel
    }
    pub fn dme_latitude_deg(&self) -> Option<f64> {
        self.dme_latitude_deg
    }
    pub fn dme_longitude_deg(&self) -> Option<f64> {
        self.dme_longitude_deg
    }
    pub fn dme_elevation_ft(&self) -> Option<i32> {
        self.dme_elevation_ft
    }
    pub fn slaved_variation_deg(&self) -> Option<f64> {
        self.slaved_variation_deg
    }
    pub fn magnetic_variation_deg(&self) -> Option<f64> {
        self.magnetic_variation_deg
    }
    pub fn usage_type(&self) -> Option<&UsageType> {
        self.usage_type.as_ref()
    }
    pub fn power(&self) -> Option<&NavaidPower> {
        self.power.as_ref()
    }
    pub fn associated_airport(&self) -> &str {
        &self.associated_airport
    }
}

impl PartialEq for Navaid {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Navaid {}

impl Ord for Navaid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Navaid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Navaid {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ToJsonString for Navaid {}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum NavaidType {
    Dme,
    Ndb,
    NdbDme,
    Tacan,
    Vor,
    VorDme,
    Vortac,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum UsageType {
    Hi,
    Lo,
    Both,
    #[serde(alias = "TERMINAL")]
    Term,
    Rnav,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum NavaidPower {
    Low,
    Medium,
    High,
    Unknown,
}

pub fn get_navaids_csv() -> Result<BTreeMap<Id, Navaid>, FetchError> {
    // get data
    debug!("getting data");
    let content = crate::web_request_blocking(NAVAIDS_CSV_URL)?;
    // initialise csv reader & return value
    debug!("initialising CSV reader");
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    debug!("parsing and deserializing data");
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Navaid = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
