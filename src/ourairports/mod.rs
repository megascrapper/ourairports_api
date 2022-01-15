//! Contains all of the OurAirports data types and its associated enums and functions.

use error_chain::error_chain;
use serde::de::{self, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};

pub mod airport_frequencies;
pub mod airports;
pub mod countries;
pub mod navaids;
pub mod regions;
pub mod runways;

/// Type of all ID fields.
pub type Id = u64;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Csv(csv::Error);
    }
}

/// List of allowed continent values.
#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Continent {
    #[serde(rename = "AF")]
    Africa,
    #[serde(rename = "AN")]
    Antarctica,
    #[serde(rename = "AS")]
    Asia,
    #[serde(rename = "EU")]
    Europe,
    #[serde(rename = "NA")]
    NorthAmerica,
    #[serde(rename = "OC")]
    Oceania,
    #[serde(rename = "SA")]
    SouthAmerica,
}

/// Trait for converting OurAirports data into JSON.
pub trait ToJsonString {
    /// Serialize an OurAirports data to string of JSON.
    fn to_json_string(&self) -> serde_json::Result<String>
    where
        Self: serde::Serialize,
    {
        serde_json::to_string(&self)
    }

    /// Serialize an OurAirports data to string of JSON with pretty printing.
    fn to_json_string_pretty(&self) -> serde_json::Result<String>
    where
        Self: serde::Serialize,
    {
        serde_json::to_string_pretty(&self)
    }
}

/// Converts a string to a boolean based on "yes" and "no"
fn bool_from_str<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.to_lowercase().as_str() {
        "yes" | "1" => Ok(true),
        "no" | "0" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"Value must be yes and no or 1 and 0",
        )),
    }
}

/// Transforms a comma-separated string to a vector.
fn vec_string_from_string<'de, D>(deserializer: D) -> std::result::Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let keywords = String::deserialize(deserializer)?;
    match keywords.len() {
        0 => Ok(vec![]),
        _ => Ok(keywords.split(',').map(|s| s.trim().to_string()).collect()),
    }
}
