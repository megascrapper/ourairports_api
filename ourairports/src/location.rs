pub use ourairports_derive::ContainsLocation;

pub type Latitude = Option<f64>;
pub type Longitude = Option<f64>;
pub type Elevation = Option<i32>;

pub trait ContainsLocation {
    fn latitude_deg(&self) -> Latitude;
    fn longitude_deg(&self) -> Longitude;
    fn elevation_ft(&self) -> Elevation;
}

pub trait ExtractLocation {
    /// Extracts location information for this item.
    fn location(&self) -> Location where Self: ContainsLocation {
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

impl<T: ContainsLocation> From<T> for Location {
    fn from(item: T) -> Self {
        Location {
            latitude_deg: item.latitude_deg(),
            longitude_deg: item.latitude_deg(),
            elevation_ft: item.elevation_ft()
        }
    }
}