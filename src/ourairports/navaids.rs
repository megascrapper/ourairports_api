use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum UsageType {
    Hi,
    Lo,
    Both,
    Term,
    Rnav
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum NavaidPower {
    Low,
    Medium,
    High,
    Unknown
}