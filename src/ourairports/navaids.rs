use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum NavaidType {
    Dme,
    Ndb,
    NdbDme,
    Tacan,
    Vor,
    VorDme,
    Vortac
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum UsageType {
    Hi,
    Lo,
    Both,
    Term,
    Rnav
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum NavaidPower {
    High,
    Medium,
    Low,
    Unknown
}