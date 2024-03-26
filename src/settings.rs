use config::{Config, ConfigError, Environment, File};
use dirs;
use serde_derive::Deserialize;

#[derive(serde_derive::Deserialize, Debug)]
struct General {
    lang: String,
}

#[derive(serde_derive::Deserialize, Debug)]
struct Observatory {
    place: String,
    latitude: f32,
    longitude: f32,
    altitude: f32,
    observatory_name: String,
    observer_name: String,
    mpc_code: String,
    nord_altitude: i32,
    south_altitude: i32,
    east_altitude: i32,
    west_altitude: i32,
}
#[derive(serde_derive::Deserialize, Debug)]
pub struct Settings {
    general: General,
    observatory: Observatory,
}

impl Settings {
    /// Constructor for Settings struct
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(
                dirs::config_local_dir()
                    .unwrap()
                    .join("asteroid_tui")
                    .join("config.toml")
                    .to_str()
                    .unwrap(),
            ))
            .build()?;
        s.try_deserialize()
    }

    /// Get lang value from settings
    pub fn get_lang(&self) -> String {
        self.general.lang
    }

    /// Get place value from settings
    pub fn get_place(&self) -> String {
        self.observatory.place
    }

    /// Get observatory name value from settings
    pub fn get_observatory_name(&self) -> String {
        self.observatory.observatory_name
    }

    /// Get observer name value from settings
    pub fn get_observer_name(&self) -> String {
        self.observatory.observer_name
    }

    /// Get mpc code value from settings
    pub fn get_mpc_code(&self) -> String {
        self.observatory.mpc_code
    }

    /// Get latitude value from settings
    pub fn get_latitude(&self) -> f32 {
        self.observatory.latitude
    }

    /// Get longitude value from settings
    pub fn get_longitude(&self) -> f32 {
        self.observatory.longitude
    }

    /// Get altitude value from settings
    pub fn get_altitude(&self) -> f32 {
        self.observatory.altitude
    }

    /// Get nord altitude value from settings
    pub fn get_nord_altitude(&self) -> i32 {
        self.observatory.nord_altitude
    }

    /// Get south altitude value from settings
    pub fn get_south_altitude(&self) -> i32 {
        self.observatory.south_altitude
    }

    /// Get east altitude value from settings
    pub fn get_east_altitude(&self) -> i32 {
        self.observatory.east_altitude
    }

    /// Get west altitude value from settings
    pub fn get_west_altitude(&self) -> i32 {
        self.observatory.west_altitude
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_settings() {
        let s = Settings::new().unwrap();
        assert!(s.observatory.place.is_ascii());
    }

    #[test]
    fn test_get_values() {
        let s = Settings::new().unwrap();
        assert!(s.get_lang().is_ascii());
        assert!(s.get_place().is_ascii());
        assert!(s.get_observatory_name().is_ascii());
        assert!(s.get_observer_name().is_ascii());
        assert!(s.get_mpc_code().is_ascii());
        assert!(s.get_latitude().is_finite());
        assert!(s.get_longitude().is_finite());
        assert!(s.get_altitude().is_finite());
        assert!(s.get_nord_altitude().is_positive());
        assert!(s.get_south_altitude().is_positive());
        assert!(s.get_east_altitude().is_positive());
        assert!(s.get_west_altitude().is_positive());
    }
}
