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
    pub fn get_lang() -> String {
        let s = Settings::new().unwrap();
        s.general.lang
    }

    /// Get place value from settings
    pub fn get_place() -> String {
        let s = Settings::new().unwrap();
        s.observatory.place
    }

    /// Get observatory name value from settings
    pub fn get_observatory_name() -> String {
        let s = Settings::new().unwrap();
        s.observatory.observatory_name
    }

    /// Get observer name value from settings
    pub fn get_observer_name() -> String {
        let s = Settings::new().unwrap();
        s.observatory.observer_name
    }

    /// Get mpc code value from settings
    pub fn get_mpc_code() -> String {
        let s = Settings::new().unwrap();
        s.observatory.mpc_code
    }

    /// Get latitude value from settings
    pub fn get_latitude() -> f32 {
        let s = Settings::new().unwrap();
        s.observatory.latitude
    }

    /// Get longitude value from settings
    pub fn get_longitude() -> f32 {
        let s = Settings::new().unwrap();
        s.observatory.longitude
    }

    /// Get altitude value from settings
    pub fn get_altitude() -> f32 {
        let s = Settings::new().unwrap();
        s.observatory.altitude
    }

    /// Get nord altitude value from settings
    pub fn get_nord_altitude() -> i32 {
        let s = Settings::new().unwrap();
        s.observatory.nord_altitude
    }

    /// Get south altitude value from settings
    pub fn get_south_altitude() -> i32 {
        let s = Settings::new().unwrap();
        s.observatory.south_altitude
    }

    /// Get east altitude value from settings
    pub fn get_east_altitude() -> i32 {
        let s = Settings::new().unwrap();
        s.observatory.east_altitude
    }

    /// Get west altitude value from settings
    pub fn get_west_altitude() -> i32 {
        let s = Settings::new().unwrap();
        s.observatory.west_altitude
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
        assert!(Settings::get_lang().is_ascii());
        assert!(Settings::get_place().is_ascii());
        assert!(Settings::get_observatory_name().is_ascii());
        assert!(Settings::get_observer_name().is_ascii());
        assert!(Settings::get_mpc_code().is_ascii());
        assert!(Settings::get_latitude().is_finite());
        assert!(Settings::get_longitude().is_finite());
        assert!(Settings::get_altitude().is_finite());
        assert!(Settings::get_nord_altitude().is_positive());
        assert!(Settings::get_south_altitude().is_positive());
        assert!(Settings::get_east_altitude().is_positive());
        assert!(Settings::get_west_altitude().is_positive());
    }
}
