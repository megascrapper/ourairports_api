use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AirportType {
    SmallAirport,
    MediumAirport,
    LargeAirport,
    Heliport,
    SeaplaneBase,
    ClosedAirport,
}