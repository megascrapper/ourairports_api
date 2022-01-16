//! Contains the type representing a single airport as well as enums of possible airport types.
//!
//! Also contains a function to get airports data from OurAirports.
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
//!     assert_eq!(2434, heathrow_airport.id());
//!     assert_eq!("EGLL", heathrow_airport.ident());
//!     assert_eq!("LHR", heathrow_airport.iata_code());
//!     assert_eq!(&AirportType::LargeAirport, heathrow_airport.airport_type());
//!
//! #    Ok(())
//! # }
//! ```

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use log::debug;

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
    airport_type: AirportType,
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
    /// Internal OurAirports integer identifier for the airport. This will stay persistent, even if
    /// the airport code changes.
    pub fn id(&self) -> Id {
        self.id
    }
    /// The text identifier used in the OurAirports URL.
    ///
    /// This will be the ICAO code if available.
    /// Otherwise, it will be a local airport code (if no conflict), or if nothing else is
    /// available, an internally-generated code starting with the ISO2 country code, followed by a
    /// dash and a four-digit number.
    pub fn ident(&self) -> &str {
        &self.ident
    }
    /// The type of the airport. See [`AirportType`] for available values.
    pub fn airport_type(&self) -> &AirportType {
        &self.airport_type
    }
    /// The official airport name, including "Airport", "Airstrip", etc.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The airport latitude in decimal degrees (positive for north).
    pub fn latitude_deg(&self) -> f64 {
        self.latitude_deg
    }
    /// The airport longitude in decimal degrees (positive for east).
    pub fn longitude_deg(&self) -> f64 {
        self.longitude_deg
    }
    /// The airport elevation above MSL in feet (negative for altitude below MSL).
    pub fn elevation_ft(&self) -> Option<i32> {
        self.elevation_ft
    }
    /// The continent where the airport is located. See [`Continent`] for possible values.
    pub fn continent(&self) -> &Continent {
        &self.continent
    }
    /// The two-character [ISO 3166:1-alpha2 code](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
    /// for the country where the airport is (primarily) located. A handful of unofficial, non-ISO
    /// codes are also in use, such as "XK" for Kosovo. The values corresponds to [`Country.code`](../countries/struct.Country.html#method.code)
    pub fn iso_country(&self) -> &str {
        &self.iso_country
    }
    /// An alphanumeric code for the high-level administrative subdivision of a country where the
    /// airport is primarily located (e.g. province, governorate), prefixed by the ISO2 country code
    /// and a hyphen. [ISO 3166:2](https://en.wikipedia.org/wiki/ISO_3166-2) codes are used whenever
    /// possible, preferring higher administrative levels, but some custom codes are also present.
    pub fn iso_region(&self) -> &str {
        &self.iso_region
    }
    /// The primary municipality that the airport serves (when available).
    /// Note that this is not necessarily the municipality where the airport is physically located.
    pub fn municipality(&self) -> &str {
        &self.municipality
    }
    /// `true` if the airport currently has scheduled airline service; `false` otherwise.
    pub fn scheduled_service(&self) -> bool {
        self.scheduled_service
    }
    /// The code that an aviation GPS database (such as Jeppesen's or Garmin's) would normally use
    /// for the airport. This will always be the [ICAO code](https://en.wikipedia.org/wiki/ICAO_airport_code)
    /// if one exists.
    pub fn gps_code(&self) -> &str {
        &self.gps_code
    }
    /// The three-letter [IATA code](https://en.wikipedia.org/wiki/International_Air_Transport_Association_code)
    /// for the airport (if it has one).
    pub fn iata_code(&self) -> &str {
        &self.iata_code
    }
    /// The local country code for the airport, if different from the [`gps_code`](self.gps_code()) and
    /// [`iata_code`](self.iata_code()) fields (used mainly for US airports).
    pub fn local_code(&self) -> &str {
        &self.local_code
    }
    /// URL of the airport's official home page on the web, if one exists.
    pub fn home_link(&self) -> &str {
        &self.home_link
    }
    /// URL of the airport's page on Wikipedia, if one exists.
    pub fn wikipedia_link(&self) -> &str {
        &self.wikipedia_link
    }
    /// Extra keywords/phrases to assist with search. May include former names for the airport,
    /// alternate codes, names in other languages, nearby tourist destinations, etc. Each item
    /// represents one keyword.
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
    #[serde(alias = "closed")]
    ClosedAirport,
    #[serde(alias = "balloonport")]
    BalloonPort // undocumented type?
}

/// Returns a [`BTreeMap`] of all [`Airport`] in the latest OurAirports `airports.csv`
/// with its ID as the key, sorted according to its keys.
///
/// # Errors
/// Returns [`FetchError`] if the data cannot be fetched or there's something wrong
/// with the de serialization process.
pub fn get_airports_csv() -> Result<BTreeMap<Id, Airport>, FetchError> {
    // get data
    debug!("getting data");
    let content = crate::web_request_blocking(AIRPORTS_CSV_URL)?;
    // initialise csv reader & return value
    debug!("initialising CSV reader");
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    debug!("parsing and deserializing data");
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Airport = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
