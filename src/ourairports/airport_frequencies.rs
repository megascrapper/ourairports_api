//! Contains the type representing a single radio frequency.
//!
//! # Examples
//! ```
//! use ourairports_api::ourairports::airport_frequencies::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let airport_frequencies = get_airport_frequencies_csv()?;
//!
//!     // Nairobi tower (id: 54836)
//!     let tower = airport_frequencies.get(&54836).unwrap();
//!     assert_eq!("HKJK", tower.airport_ident());
//!     assert_eq!("TWR", tower.frequency_type());
//!
//! #    Ok(())
//! # }
//! ```

use crate::ourairports::{FetchError, Id, ToJsonString};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use log::debug;

const AIRPORT_FREQUENCIES_CSV_URL: &str =
    "https://davidmegginson.github.io/ourairports-data/airport-frequencies.csv";

/// Represents a single airport radio frequency for voice communication.
///
/// See the [OurAirports data dictionary](https://ourairports.com/help/data-dictionary.html#airport-frequencies)
/// for more information of each field.
///
/// # See also
/// * [`Navaid`](super::navaids::Navaid): for frequencies used in navigational aids.
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
    /// Internal OurAirports integer identifier for the frequency. This will stay persistent, even
    /// if the radio frequency or description changes.
    pub fn id(&self) -> Id {
        self.id
    }
    /// Internal OurAirports integer identifier for the associated airport.
    ///
    /// See also: [`Airport.id()`](../airports/struct.Airport.html#method.id)
    pub fn airport_ref(&self) -> Id {
        self.airport_ref
    }
    /// The identifier of the associated airport for the frequency.
    ///
    /// See also [`Airport.ident()`](../airports/struct.Airport.html#method.ident)
    pub fn airport_ident(&self) -> &str {
        &self.airport_ident
    }
    /// The type of this frequency.
    ///
    /// Currently, `frequency_type` is stored as a `String` as the frequency type is not a
    /// controlled vocabulary. This may change in the future as the type codes are stadardised.
    pub fn frequency_type(&self) -> &str {
        &self.frequency_type
    }
    /// A description of the frequency, typically the way a pilot would open a call on it.
    pub fn description(&self) -> &str {
        &self.description
    }
    /// Radio voice frequency in megahertz.
    ///
    /// Note that the same frequency may appear multiple times for an airport, serving different functions.
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

/// Returns a [`BTreeMap`] of all [`AirportFrequency`] in the latest OurAirports `airport_frequencies.csv`
/// with its ID as the key, sorted according to its keys.
///
/// # Errors
/// Returns [`FetchError`] if the data cannot be fetched or there's something wrong
/// with the de serialization process.
pub fn get_airport_frequencies_csv() -> Result<BTreeMap<Id, AirportFrequency>, FetchError> {
    // get data
    debug!("getting data");
    let content = crate::web_request_blocking(AIRPORT_FREQUENCIES_CSV_URL)?;
    // initialise csv reader & return value
    debug!("initialising reader");
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    debug!("parsing and deserializing data");
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: AirportFrequency = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
