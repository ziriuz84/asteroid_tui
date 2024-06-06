use config::{Config, ConfigError, Environment, File};
use dirs;
use rand::Rng;
use serde_derive::Deserialize;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use toml;

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
struct General {
    lang: String,
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
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
#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug)]
pub struct Settings {
    general: General,
    observatory: Observatory,
}

/// Finds if config file exists or create it
fn file_exists_or_create() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir: PathBuf = dirs::config_local_dir().ok_or("Failed to get config local dir")?;
    let asteroid_tui_path: PathBuf = config_dir.join("asteroid_tui");

    // Ensure the directory exists
    if asteroid_tui_path.exists() {
        fs::create_dir_all(&asteroid_tui_path)?;
    }

    // Create or update the config file
    let config_file_path = asteroid_tui_path.join("config.toml");
    if config_file_path.exists() {
        let default_settings: Settings = default_settings();
        let default: String = toml::to_string(&default_settings)?;
        fs::write(config_file_path.clone(), default)?;
    }

    // Open the file for external use

    Ok(())
}

/// Creates default settings for file creation
fn default_settings() -> Settings {
    let mut rng = rand::thread_rng();
    let default_general: General = General {
        lang: "en".to_string(),
    };
    let default_observatory: Observatory = Observatory {
        place: "default".to_string(),
        latitude: rng.gen_range(0.1..89.9) as f32,
        longitude: rng.gen_range(0.1..179.9) as f32,
        altitude: rng.gen_range(0.1..100.0) as f32,
        observatory_name: "default".to_string(),
        observer_name: "default".to_string(),
        mpc_code: "500".to_string(),
        nord_altitude: 1,
        east_altitude: 1,
        south_altitude: 1,
        west_altitude: 1,
    };
    Settings {
        general: default_general,
        observatory: default_observatory,
    }
}

/// Parses value as float
///
/// * `value`: The value to be parsed
fn parse_float64(value: &str) -> Result<f64, Box<dyn std::error::Error>> {
    match value.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Could not parse value as float".into()),
    }
}

/// Parse value as integer
///
/// * `value`: The value to be parsed
fn parse_integer64(value: &str) -> Result<i64, Box<dyn std::error::Error>> {
    match value.parse::<i64>() {
        Ok(value) => Ok(value),
        Err(_) => Err("Could not parse value as float".into()),
    }
}

fn modify_field_in_file(key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file
    let contents = fs::read_to_string(
        dirs::config_local_dir()
            .unwrap()
            .join("asteroid_tui")
            .join("config.toml")
            .to_str()
            .unwrap(),
    )?;

    // Parse the TOML contents into a HashMap
    let mut settings: toml::Value = toml::from_str(&contents)?;

    // Modify the field
    /*
    if let Some(table) = settings.get_mut(key) {
        table.insert("new_value".to_string(), value.to_string());
    } else {
        settings.insert(key.to_string(), toml::Value::String(value.to_string()));
    }
    */

    match key {
        "lang" => settings["general"]["lang"] = toml::Value::String(value.to_string()),
        "place" => settings["observatory"]["place"] = toml::Value::String(value.to_string()),
        "latitude" => {
            settings["observatory"]["latitude"] = toml::Value::Float(parse_float64(value).unwrap())
        }
        "longitude" => {
            settings["observatory"]["longitude"] = toml::Value::Float(parse_float64(value).unwrap())
        }
        "altitude" => {
            settings["observatory"]["altitude"] = toml::Value::Float(parse_float64(value).unwrap())
        }
        "observatory_name" => {
            settings["observatory"]["observatory_name"] = toml::Value::String(value.to_string())
        }
        "observer_name" => {
            settings["observatory"]["observer_name"] = toml::Value::String(value.to_string())
        }
        "mpc_code" => settings["observatory"]["mpc_code"] = toml::Value::String(value.to_string()),
        "nord_altitude" => {
            settings["observatory"]["nord_altitude"] =
                toml::Value::Integer(parse_integer64(value).unwrap())
        }
        "south_altitude" => {
            settings["observatory"]["south_altitude"] =
                toml::Value::Integer(parse_integer64(value).unwrap())
        }
        "east_altitude" => {
            settings["observatory"]["east_altitude"] =
                toml::Value::Integer(parse_integer64(value).unwrap())
        }
        "west_altitude" => {
            settings["observatory"]["west_altitude"] =
                toml::Value::Integer(parse_integer64(value).unwrap())
        }
        _ => {}
    }

    // Serialize the updated settings back into a string
    let updated_contents = toml::to_string(&settings)?;

    // Write the updated contents back to the file
    fs::write(
        dirs::config_local_dir()
            .unwrap()
            .join("asteroid_tui")
            .join("config.toml")
            .to_str()
            .unwrap(),
        updated_contents,
    )?;

    Ok(())
}
impl Settings {
    /// Constructor for Settings struct
    pub fn new() -> Result<Self, ConfigError> {
        if fs::metadata(
            dirs::config_local_dir()
                .unwrap()
                .join("asteroid_tui")
                .to_str()
                .unwrap(),
        )
        .is_err()
        {
            if let Err(err) = fs::create_dir(
                dirs::config_local_dir()
                    .unwrap()
                    .join("asteroid_tui")
                    .to_str()
                    .unwrap(),
            ) {
                println!("Error in creating directory: {}", err);
            } else {
                let mut file = fs::File::create(
                    dirs::config_local_dir()
                        .unwrap()
                        .join("asteroid_tui")
                        .join("config.toml")
                        .to_str()
                        .unwrap(),
                )
                .unwrap();
                let default_settings: Settings = default_settings();
                let default = toml::to_string(&default_settings).unwrap();
                file.write_all(default.as_bytes())
                    .expect("Error in writing to config file");
            }
        } else {
            let mut file = fs::File::create(
                dirs::config_local_dir()
                    .unwrap()
                    .join("asteroid_tui")
                    .join("config.toml")
                    .to_str()
                    .unwrap(),
            )
            .unwrap();
            let default_settings: Settings = default_settings();
            let default = toml::to_string(&default_settings).unwrap();
            file.write_all(default.as_bytes())
                .expect("Error in writing to config file");
        }
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
    pub fn get_lang(&self) -> &String {
        &self.general.lang
    }

    pub fn set_lang(&mut self, lang: String) {}

    /// Get place value from settings
    pub fn get_place(&self) -> &String {
        &self.observatory.place
    }

    /// Get observatory name value from settings
    pub fn get_observatory_name(&self) -> &String {
        &self.observatory.observatory_name
    }

    /// Get observer name value from settings
    pub fn get_observer_name(&self) -> &String {
        &self.observatory.observer_name
    }

    /// Get mpc code value from settings
    pub fn get_mpc_code(&self) -> &String {
        &self.observatory.mpc_code
    }

    /// Get latitude value from settings
    pub fn get_latitude(&self) -> &f32 {
        &self.observatory.latitude
    }

    /// Get longitude value from settings
    pub fn get_longitude(&self) -> &f32 {
        &self.observatory.longitude
    }

    /// Get altitude value from settings
    pub fn get_altitude(&self) -> &f32 {
        &self.observatory.altitude
    }

    /// Get nord altitude value from settings
    pub fn get_nord_altitude(&self) -> &i32 {
        &self.observatory.nord_altitude
    }

    /// Get south altitude value from settings
    pub fn get_south_altitude(&self) -> &i32 {
        &self.observatory.south_altitude
    }

    /// Get east altitude value from settings
    pub fn get_east_altitude(&self) -> &i32 {
        &self.observatory.east_altitude
    }

    /// Get west altitude value from settings
    pub fn get_west_altitude(&self) -> &i32 {
        &self.observatory.west_altitude
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
