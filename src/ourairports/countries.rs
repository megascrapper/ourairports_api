//! Contains the type representing a single country.
//!
//! # Examples
//! ```
//! use ourairports_api::ourairports::countries::*;
//! use ourairports_api::ourairports::Continent;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let countries = get_countries_csv()?;
//!
//!     let sample_country = countries.get(&302791).unwrap();
//!     assert_eq!("Brazil", sample_country.name());
//!     assert_eq!("BR", sample_country.code());
//!     assert_eq!(&Continent::SouthAmerica, sample_country.continent());
//!
//! #    Ok(())
//! # }
//! ```

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use log::debug;

use crate::ourairports::{vec_string_from_string, Continent, FetchError, Id, ToJsonString};

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
    /// Internal OurAirports integer identifier for the country.
    /// This will stay persistent, even if the country name or code changes.
    pub fn id(&self) -> Id {
        self.id
    }
    /// The two-character [ISO 3166:1-alpha2 code](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
    /// for the country.
    ///
    /// A handful of unofficial, non-ISO codes are also in use, such as "XK" for Kosovo.
    pub fn code(&self) -> &str {
        &self.code
    }
    /// The common English-language name for the country.
    /// Other variations of the name may appear in [`self.keywords()`] to assist with search.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The continent where the country is (primarily) located
    ///
    /// See [`Continent`](super::Continent) for possible continent values.
    pub fn continent(&self) -> &Continent {
        &self.continent
    }
    /// Link to the Wikipedia article about the country.
    pub fn wikipedia_link(&self) -> &str {
        &self.wikipedia_link
    }
    /// A list of of search keywords/phrases related to the country. Each item represents one
    /// keyword.
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

/// Returns a [`BTreeMap`] of all [`Country`] in the latest OurAirports `countries.csv`
/// with its ID as the key, sorted according to its keys.
///
/// # Errors
/// Returns [`FetchError`] if the data cannot be fetched or there's something wrong
/// with the de serialization process.
pub fn get_countries_csv() -> Result<BTreeMap<Id, Country>, FetchError> {
    // get data
    debug!("getting data");
    let content = crate::web_request_blocking(COUNTRIES_CSV_URL)?;
    // initialise csv reader & return value
    debug!("initialising CSV reader");
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    debug!("parsing and deserializing data");
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Country = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
