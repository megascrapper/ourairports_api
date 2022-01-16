//! Contains the type representing a single radio frequency.
//!
//! # Examples
//! ```
//! use ourairports_api::ourairports::runways::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let runways = get_runways_csv()?;
//!
//!     // Runway 08L/26R at Vancouver International Airport
//!     // (ICAO: CYVR, IATA: YVR)
//!     let example_runway = runways.get(&234512).unwrap();
//!     assert_eq!("08L", example_runway.le_ident());
//!     assert_eq!("26R", example_runway.he_ident());
//!     assert_eq!("CYVR", example_runway.airport_ident());
//! #    Ok(())
//! # }
//! ```

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use log::debug;
use serde::{Deserialize, Serialize};

use crate::ourairports::{bool_from_str, FetchError, Id, ToJsonString};

const RUNWAYS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/runways.csv";

/// Represents a single airport landing surface (runway, helipad, or waterway).
///
/// See the [OurAirports data dictionary](https://ourairports.com/help/data-dictionary.html#runways)
/// for more information of each field.
///
/// # Information on runway ends
/// Methods beginning with `le_` apply only to the low-numbered end of the runway (e.g. 09), while
/// methods beginning with `he_` apply only to the high-numbered end of the runway (e.g. 27).
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
    /// Internal OurAirports integer identifier for the runway. This will stay persistent, even if
    /// the runway numbering changes.
    pub fn id(&self) -> Id {
        self.id
    }
    /// Internal OurAirports integer identifier for the associated airport.
    ///
    /// See [`Airport.id()`](../airports/struct.Airport.html#method.id) for more information about
    /// airport ID.
    pub fn airport_ref(&self) -> Id {
        self.airport_ref
    }
    /// The identifier of the associated airport for the runway.
    ///
    /// See [`Airport.ident()`](../airports/struct.Airport.html#method.ident) for more information
    /// about airport identifiers.
    pub fn airport_ident(&self) -> &str {
        &self.airport_ident
    }
    /// Length of the full runway surface (including displaced thresholds, overrun areas, etc) in
    /// feet.
    pub fn length_ft(&self) -> Option<i32> {
        self.length_ft
    }
    /// Width of the runway surface in feet.
    pub fn width_ft(&self) -> Option<i32> {
        self.width_ft
    }
    /// Code for the runway surface type.
    ///
    /// Currently, `surface` is stored as a `String` as the surface type is not a
    /// controlled vocabulary. This may change in the future as the type codes are stadardised.
    pub fn surface(&self) -> &str {
        &self.surface
    }
    /// `true` if the surface is lighted at night, `false` otherwise.
    pub fn lighted(&self) -> bool {
        self.lighted
    }
    /// `true` if the runway surface is currently closed, `false` otherwise.
    pub fn closed(&self) -> bool {
        self.closed
    }
    /// Identifier for the low-numbered end of the runway.
    pub fn le_ident(&self) -> &str {
        &self.le_ident
    }
    /// Latitude of the centre of the low-numbered end of the runway, in decimal degrees
    /// (positive is north). Returns `None` if not available.
    pub fn le_latitude_deg(&self) -> Option<f64> {
        self.le_latitude_deg
    }
    /// Longitude of the centre of the low-numbered end of the runway, in decimal degrees
    /// (positive is east). Returns `None` if not available.
    pub fn le_longitude_deg(&self) -> Option<f64> {
        self.le_longitude_deg
    }
    /// Elevation above MSL of the low-numbered end of the runway in feet.
    /// Returns `None` if not available.
    pub fn le_elevation_ft(&self) -> Option<i32> {
        self.le_elevation_ft
    }
    /// Heading of the low-numbered end of the runway in degrees true.
    /// Returns `None` if not available.
    pub fn le_heading_deg_true(&self) -> Option<f64> {
        self.le_heading_deg_true
    }
    /// Length of the displaced threshold for the low-numbered end of the runway, in feet.
    /// Returns `None` if not available.
    pub fn le_displaced_threshold_ft(&self) -> Option<i32> {
        self.le_displaced_threshold_ft
    }
    /// Identifier for the high-numbered end of the runway.
    pub fn he_ident(&self) -> &str {
        &self.he_ident
    }
    /// Latitude of the centre of the high-numbered end of the runway, in decimal degrees
    /// (positive is north). Returns `None` if not available.
    pub fn he_latitude_deg(&self) -> Option<f64> {
        self.he_latitude_deg
    }
    /// Longitude of the centre of the high-numbered end of the runway, in decimal degrees
    /// (positive is east). Returns `None` if not available.
    pub fn he_longitude_deg(&self) -> Option<f64> {
        self.he_longitude_deg
    }
    /// Elevation above MSL of the high-numbered end of the runway in feet.
    /// Returns `None` if not available.
    pub fn he_elevation_ft(&self) -> Option<i32> {
        self.he_elevation_ft
    }
    /// Heading of the high-numbered end of the runway in degrees true.
    /// Returns `None` if not available.
    pub fn he_heading_deg_true(&self) -> Option<f64> {
        self.he_heading_deg_true
    }
    /// Length of the displaced threshold for the high-numbered end of the runway, in feet.
    /// Returns `None` if not available.
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

/// Returns a [`BTreeMap`] of all [`Runway`] in the latest OurAirports `runways.csv`
/// with its ID as the key, sorted according to its keys.
///
/// # Errors
/// Returns [`FetchError`] if the data cannot be fetched or there's something wrong
/// with the de serialization process.
pub fn get_runways_csv() -> Result<BTreeMap<Id, Runway>, FetchError> {
    // get data
    debug!("getting data");
    let content = crate::web_request_blocking(RUNWAYS_CSV_URL)?;
    // initialise csv reader & return value
    debug!("initialising CSV reader");
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    debug!("parsing and deserializing data");
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Runway = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
