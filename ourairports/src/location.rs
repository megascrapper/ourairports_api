//! Contains types and traits related to containing location information.
//!
//! # Examples
//! Extracting location from an `Airport` instance
//!
//! ```
//! use ourairports::airports::*;
//! use ourairports::location::{Location, ContainsLocation};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let airports = get_airports_csv()?;
//!
//!     // Extracting the location of an airport
//!     let heathrow_airport = airports.get(&2434).unwrap();
//!     let location = heathrow_airport.extract_location();
//!     assert_eq!(heathrow_airport.latitude_deg(), location.latitude_deg());
//!
//!     // Using `Location::from()` to convert an `Airport` to `Location`
//!     let location_from = Location::from(heathrow_airport.clone());
//!     assert_eq!(location.latitude_deg(), location_from.latitude_deg());
//!
//! #    Ok(())
//! # }
//! ```

/// Type alias for latitude values.
pub type Latitude = Option<f64>;
/// Type alias for longitude values.
pub type Longitude = Option<f64>;
/// Type alias for elevation values.
pub type Elevation = Option<i32>;

/// Trait to indicate that the type contains location information.
pub trait ContainsLocation {
    /// The latitude of `self` in decimal degrees (positive for north).
    fn latitude_deg(&self) -> Latitude;
    /// The longitude `self` in decimal degrees (positive for north).
    fn longitude_deg(&self) -> Longitude;
    /// The elevation of `self` above MSL in feet (negative for altitude below MSL).
    fn elevation_ft(&self) -> Elevation;
    /// Extracts location information from `self`.
    fn extract_location(&self) -> Location {
        Location {
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.latitude_deg(),
            elevation_ft: self.elevation_ft()
        }
    }
}

// TODO: add associated id with the location
/// Contains the latitude, longitude and elevation for the location.
///
/// The value is `None` if it is not available or unknown.
pub struct Location {
    latitude_deg: Latitude,
    longitude_deg: Longitude,
    elevation_ft: Elevation
}

impl Location {
    pub fn latitude_deg(&self) -> Latitude {
        self.latitude_deg
    }
    pub fn longitude_deg(&self) -> Longitude {
        self.longitude_deg
    }
    pub fn elevation_ft(&self) -> Elevation {
        self.elevation_ft
    }
}

impl<T: ContainsLocation> From<T> for Location {
    fn from(item: T) -> Self {
        Location {
            latitude_deg: item.latitude_deg(),
            longitude_deg: item.latitude_deg(),
            elevation_ft: item.elevation_ft()
        }
    }
}