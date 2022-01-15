//! Contains the type representing a single airport as well as enums of possible airport types. Also
//! contains a function to get airports data from OurAirports.
//!
//! # Examples
//! ```
//! use ourairports_api::ourairports::airports::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let airports = get_airports_csv()?;
//!
//!     // London Heathrow Airport (ICAO: EGLL, IATA: LHR)
//!     let heathrow_airport = airports.get(&2434).unwrap();
//!     assert_eq!("EGLL", heathrow_airport.ident());
//!     assert_eq!("LHR", heathrow_airport.iata_code());
//!     assert_eq!(AirportType::LargeAirport, heathrow_airport.airport_type());
//!
//! #    Ok(())
//! # }
//! ```

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::ourairports::{
    bool_from_str, vec_string_from_string, Continent, FetchError, Id, ToJsonString,
};

const AIRPORTS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/airports.csv";

/// Represents a single airport in the OurAirports data.
///
/// See the [OurAirports data dictionary](https://ourairports.com/help/data-dictionary.html#airports)
/// for more information of each field.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Airport {
    id: Id,
    ident: String,
    #[serde(rename = "type")]
    airport_type: String,
    name: String,
    latitude_deg: f64,
    longitude_deg: f64,
    elevation_ft: Option<i32>,
    continent: Continent,
    iso_country: String,
    iso_region: String,
    municipality: String,
    #[serde(deserialize_with = "bool_from_str")]
    scheduled_service: bool,
    gps_code: String,
    iata_code: String,
    local_code: String,
    home_link: String,
    wikipedia_link: String,
    #[serde(deserialize_with = "vec_string_from_string")]
    keywords: Vec<String>,
}

impl Airport {
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn ident(&self) -> &str {
        &self.ident
    }
    /// The type of the airport. See [`AirportType`] for available values.
    pub fn airport_type(&self) -> &str {
        &self.airport_type
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn latitude_deg(&self) -> f64 {
        self.latitude_deg
    }
    pub fn longitude_deg(&self) -> f64 {
        self.longitude_deg
    }
    pub fn elevation_ft(&self) -> Option<i32> {
        self.elevation_ft
    }
    pub fn continent(&self) -> &Continent {
        &self.continent
    }
    pub fn iso_country(&self) -> &str {
        &self.iso_country
    }
    pub fn iso_region(&self) -> &str {
        &self.iso_region
    }
    pub fn municipality(&self) -> &str {
        &self.municipality
    }
    pub fn scheduled_service(&self) -> bool {
        self.scheduled_service
    }
    pub fn gps_code(&self) -> &str {
        &self.gps_code
    }
    pub fn iata_code(&self) -> &str {
        &self.iata_code
    }
    pub fn local_code(&self) -> &str {
        &self.local_code
    }
    pub fn home_link(&self) -> &str {
        &self.home_link
    }
    pub fn wikipedia_link(&self) -> &str {
        &self.wikipedia_link
    }
    pub fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }
}

impl PartialEq for Airport {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Airport {}

impl Ord for Airport {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Airport {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Airport {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl ToJsonString for Airport {}

/// Possible types of airports.
///
/// See [OurAirports map legend](https://ourairports.com/help/data-dictionary.html#airports)
/// for more information of each variant.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AirportType {
    /// Small airport
    SmallAirport,
    /// Medium airport
    MediumAirport,
    /// Large airport
    LargeAirport,
    /// Heliport
    Heliport,
    /// Seaplane base
    SeaplaneBase,
    /// Closed airport
    ClosedAirport,
}

/// Returns a [`BTreeMap`] of all [`Airport`] in the latest OurAirports `airports.csv`
/// with its ID as the key, sorted according to its keys.
///
/// # Errors
/// Returns [`crate::ourairports::FetchError`] if the data cannot be fetched or there's something wrong
/// with the serialization process.
pub fn get_airports_csv() -> Result<BTreeMap<Id, Airport>, FetchError> {
    // get data
    let content = crate::web_request_blocking(AIRPORTS_CSV_URL)?;
    // initialise csv reader & return value
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Airport = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
