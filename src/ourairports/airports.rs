use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AirportType {
    ClosedAirport,
    Heliport,
    SmallAirport,
    MediumAirport,
    LargeAirport,
    SeaplaneBase,
}