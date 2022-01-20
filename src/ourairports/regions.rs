//! Contains the type representing a single administrative subdivision.
//!
//! # Examples
//! ```
//! use ourairports_api::ourairports::regions::*;
//! use ourairports_api::ourairports::Continent;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let regions = get_regions_csv()?;
//!
//!     let sample_region = regions.get(&306408).unwrap();
//!     assert_eq!("GE", sample_region.iso_country());
//!     assert_eq!("KA", sample_region.local_code());
//!     assert_eq!(&format!("{}-{}", sample_region.iso_country(), sample_region.local_code()), sample_region.code());
//!
//! #    Ok(())
//! # }
//! ```

use log::debug;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::ourairports::{vec_string_from_string, Continent, FetchError, Id, ToJsonString};

const REGIONS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/regions.csv";

/// Represents a high-level administrative subdivision of a country.
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
    /// Internal OurAirports integer identifier for the region.
    /// This will stay persistent, even if the region code changes.
    pub fn id(&self) -> Id {
        self.id
    }
    /// [`self.local_code()`] prefixed with the country code to make a globally-unique identifier.
    pub fn code(&self) -> &str {
        &self.code
    }
    /// local_code prefixed with the country code to make a globally-unique identifier.
    ///
    /// Whenever possible, these are official [ISO 3166:2](https://en.wikipedia.org/wiki/ISO_3166-2),
    /// at the highest level available, but in some cases OurAirports has to use unofficial codes.
    /// There is also a pseudo code `U-A` for each country, which means that the airport has not yet
    /// been assigned to a region (or perhaps can't be, as in the case of a deep-sea oil platform).
    pub fn local_code(&self) -> &str {
        &self.local_code
    }
    /// The common English-language name for the administrative subdivision.
    ///
    /// In some cases, the name in local languages will appear in `self.keywords()`
    pub fn name(&self) -> &str {
        &self.name
    }
    /// A code for the continent to which the region belongs.
    ///
    /// See [`Continent`](super::Continent) for possible continent values.
    pub fn continent(&self) -> &Continent {
        &self.continent
    }
    /// The two-character [ISO 3166:1-alpha2 code](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
    /// for the country containing the administrative subdivision.
    ///
    /// A handful of unofficial, non-ISO codes are also in use, such as "XK" for Kosovo.
    pub fn iso_country(&self) -> &str {
        &self.iso_country
    }
    /// A link to the Wikipedia article describing the subdivision.
    pub fn wikipedia_link(&self) -> &str {
        &self.wikipedia_link
    }
    /// A list of keywords to assist with search. May include former names for the region, and/or
    /// the region name in other languages. Each item represents one keyword.
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

/// Returns a [`BTreeMap`] of all [`Region`] in the latest OurAirports `regions.csv`
/// with its ID as the key, sorted according to its keys.
///
/// # Errors
/// Returns [`FetchError`] if the data cannot be fetched or there's something wrong
/// with the de serialization process.
pub fn get_regions_csv() -> Result<BTreeMap<Id, Region>, FetchError> {
    // get data
    debug!("getting data");
    let content = crate::web_request_blocking(REGIONS_CSV_URL)?;
    // initialise csv reader & return value
    debug!("initialising CSV reader");
    let mut rdr = csv::Reader::from_reader(content.as_bytes());
    debug!("parsing and deserializing data");
    let mut map = BTreeMap::new();
    for result in rdr.deserialize() {
        let record: Region = result?;
        map.insert(record.id(), record);
    }
    Ok(map)
}
