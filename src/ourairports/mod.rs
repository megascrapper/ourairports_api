use std::fmt::{Display, Formatter, write};
use error_chain::error_chain;
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Unexpected};

pub mod airports;
pub mod airport_frequencies;
pub mod runways;
pub mod navaids;
pub mod countries;
pub mod regions;

pub type Id = String;

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
    SouthAmerica
}

pub trait ToJsonString {
    fn to_json_string(&self) -> serde_json::Result<String> where Self: serde::Serialize {
        serde_json::to_string(&self)
    }

    fn to_json_string_pretty(&self) -> serde_json::Result<String> where Self: serde::Serialize {
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
            &"Value must be yes or no",
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