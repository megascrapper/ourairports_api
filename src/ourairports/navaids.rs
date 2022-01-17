//! Contains the type representing a single radio navigation as well as enums of possible radio
//! navigation types, power and usage type.
//!
//! # Examples
//! ```
//! use ourairports_api::ourairports::navaids::*;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let navaids = get_navaids_csv()?;
//!
//!     let sample_navaid = navaids.get(&86738).unwrap();
//!     assert_eq!("NZCH", sample_navaid.associated_airport());
//!     assert_eq!(&UsageType::Both, sample_navaid.usage_type().unwrap());
//!     assert_eq!(&NavaidPower::High, sample_navaid.power().unwrap());
//!     assert_eq!(&NavaidType::VorDme, sample_navaid.navaid_type());
//! #    Ok(())
//! # }
//! ```

use crate::ourairports::{FetchError, Id, ToJsonString};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use log::debug;

const NAVAIDS_CSV_URL: &str = "https://davidmegginson.github.io/ourairports-data/navaids.csv";

/// Represents a single radio navigation.
///
/// See the [OurAirports data dictionary](https://ourairports.com/help/data-dictionary.html#navaids)
/// for more information of each field.
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
    /// Internal OurAirports integer identifier for the navaid.
    /// This will stay persistent, even if the navaid identifier or frequency changes.
    pub fn id(&self) -> Id {
        self.id
    }
    /// A unique string identifier constructed from the navaid name and country, and used in the
    /// OurAirports URL.
    pub fn filename(&self) -> &str {
        &self.filename
    }
    /// The 1-3 character identifer that the navaid transmits.
    pub fn ident(&self) -> &str {
        &self.ident
    }
    /// The name of the navaid, excluding its type.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The name of the navaid, excluding its type.
    ///
    /// See [`NavaidType`] for a list of possible values.
    pub fn navaid_type(&self) -> &NavaidType {
        &self.navaid_type
    }
    /// The frequency of the navaid in *kilohertz*.
    ///
    /// If the Navaid operates on the VHF band (VOR, VOR-DME) or operates on the UHF band with a
    /// paired VHF frequency (DME, TACAN, VORTAC), then you need to divide this number by 1,000 to
    /// get the frequency in megahertz (115.3 MHz in this example).
    /// For an NDB or NDB-DME, you can use this frequency directly.
    pub fn frequency_khz(&self) -> &str {
        &self.frequency_khz
    }
    /// The latitude of the navaid in decimal degrees (negative for south). Returns `None` if not available.
    pub fn latitude_deg(&self) -> Option<f64> {
        self.latitude_deg
    }
    /// The longitude of the navaid in decimal degrees (negative for west). Returns `None` if not available.
    pub fn longitude_deg(&self) -> Option<f64> {
        self.longitude_deg
    }
    /// The navaid's elevation MSL in feet. Returns `None` if not available.
    pub fn elevation_ft(&self) -> Option<i32> {
        self.elevation_ft
    }
    /// The two-character [ISO 3166:1-alpha2 code](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
    /// for the country that operates the navaid.
    ///
    /// A handful of unofficial, non-ISO codes are also in use, such as "XK" for Kosovo.
    /// The values corresponds to [`Country.code`](../countries/struct.Country.html#method.code)
    pub fn iso_country(&self) -> &str {
        &self.iso_country
    }
    /// The paired VHF frequency for the DME (or TACAN) in kilohertz.
    ///
    /// Divide by 1,000 to get the paired VHF frequency in megahertz (e.g. 115.3 MHz).
    pub fn dme_frequency_khz(&self) -> &str {
        &self.dme_frequency_khz
    }
    /// The DME channel (an alternative way of tuning distance-measuring equipment).
    pub fn dme_channel(&self) -> &str {
        &self.dme_channel
    }
    /// The latitude of the associated DME in decimal degrees (negative for south).
    ///
    /// If `None`, assume the value is the same as [`self.latitude_deg()`].
    pub fn dme_latitude_deg(&self) -> Option<f64> { self.dme_latitude_deg }
    /// The longitude of the associated DME in decimal degrees (negative for west).
    ///
    /// If `None`, assume the value is the same as [`self.longitude_deg()`].
    pub fn dme_longitude_deg(&self) -> Option<f64> { self.dme_longitude_deg }
    /// The associated DME transmitters elevation MSL in feet.
    ///
    /// If `None`, assume the value is the same as [`self.elevation_ft()`].
    pub fn dme_elevation_ft(&self) -> Option<i32> {
        self.dme_elevation_ft
    }
    /// The magnetic variation adjustment built into a VOR's, VOR-DME's, or TACAN's radials.
    /// Positive means east (added to the true direction), and negative means west (subtracted from the true direction).
    ///
    /// This will not usually be the same as [`self.slaved_variation_deg()`] because the magnetic pole is constantly in motion.
    pub fn slaved_variation_deg(&self) -> Option<f64> {
        self.slaved_variation_deg
    }
    /// The actual magnetic variation at the navaid's location.
    /// Positive means east (added to the true direction), and negative means west (subtracted from the true direction).
    pub fn magnetic_variation_deg(&self) -> Option<f64> {
        self.magnetic_variation_deg
    }
    /// The primary function of the navaid in the airspace system.
    ///
    /// See [`UsageType`] for a list of possible values.
    pub fn usage_type(&self) -> Option<&UsageType> {
        self.usage_type.as_ref()
    }
    /// The power-output level of the navaid.
    ///
    /// See [`NavaidPower`] for a list of possible values.
    pub fn power(&self) -> Option<&NavaidPower> {
        self.power.as_ref()
    }
    /// The identifier of the associated airport for the navaid.
    ///
    /// See [`Airport.ident()`](../airports/struct.Airport.html#method.ident) for more information
    /// about airport identifiers.
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

/// Possible types of navaids.
///
/// See [OurAirports map legend](https://ourairports.com/help/#navaids)
/// for more information of each variant.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum NavaidType {
    /// DME
    Dme,
    /// NDB
    Ndb,
    /// NDB-DME
    NdbDme,
    /// TACAN
    Tacan,
    /// VOR
    Vor,
    /// VOR-DME
    VorDme,
    /// VORTAC
    Vortac,
}

/// possible usage types of navaids.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum UsageType {
    /// High-altitude airways, at or above flight level 180
    Hi,
    /// Low-altitude airways
    Lo,
    /// High- and low-altitude airways
    Both,
    /// Terminal-area navigation only
    #[serde(alias = "TERMINAL")]
    Term,
    /// Non-GPS area navigation
    Rnav,
}

/// Possible power levels of navaids.
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
