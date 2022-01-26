pub use ourairports_derive::ContainsLocation;

pub type Latitude = f64;
pub type Longitude = f64;
pub type Elevation = Option<i32>;

pub trait ContainsLocation {
    fn latitude_deg(&self) -> Latitude;
    fn longitude_deg(&self) -> Longitude;
    fn elevation_ft(&self) -> Elevation;
}

pub trait ToLocation {
    /// Extracts location information for this item.
    fn to_location(&self) -> Location where Self: ContainsLocation {
        Location {
            latitude_deg: self.latitude_deg(),
            longitude_deg: self.latitude_deg(),
            elevation_ft: self.elevation_ft()
        }
    }
}

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