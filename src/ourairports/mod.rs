use error_chain::error_chain;
use serde::{Deserialize, Deserializer, Serializer};
use serde::de::{self, Unexpected};

pub mod countries;

pub type Id = String;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Csv(csv::Error);
    }
}

/// List of allowed continent values.
pub enum Continent {
    Africa,
    Antarctica,
    Asia,
    Europe,
    NorthAmerica,
    Oceania,
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

fn continents_enum_from_string<'de, D>(deserializer: D) -> std::result::Result<Continent, D::Error>
    where
        D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_str() {
        "AF" => Ok(Continent::Africa),
        "AN" => Ok(Continent::Antarctica),
        "AS" => Ok(Continent::Asia),
        "EU" => Ok(Continent::Europe),
        "NA" => Ok(Continent::NorthAmerica),
        "OC" => Ok(Continent::Oceania),
        "SA" => Ok(Continent::SouthAmerica),
        _ => panic!("invalid continent value")
    }
}

fn continents_enum_to_string<S>(value: &Continent) -> std::result::Result<String, S::Error>
    where S: Serializer
{
    match value {
        Continent::Africa => Ok(String::from("AF")),
        Continent::Antarctica => Ok(String::from("AN")),
        Continent::Asia => Ok(String::from("AS")),
        Continent::Europe => Ok(String::from("EU")),
        Continent::NorthAmerica => Ok(String::from("NA")),
        Continent::Oceania => Ok(String::from("OC")),
        Continent::SouthAmerica => Ok(String::from("SA"))
    }
}