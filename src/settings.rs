use config::{Config, ConfigError, File};
use rand::Rng;
use std::fs;

//TODO: Add minimum altitude on different directions

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, Clone)]
/// General option structure
///
/// * `lang`: language
/// * `mpc_auth_token`: legacy field (unused by What's Up; token is scraped automatically)
pub struct General {
    /// Language
    pub lang: String,
    /// Legacy MPC token field (unused by What's Up)
    #[serde(default = "default_mpc_auth_token")]
    pub mpc_auth_token: String,
}

/// Default for legacy `mpc_auth_token` field (empty; What's Up scrapes its own token).
pub(crate) fn default_mpc_auth_token() -> String {
    std::env::var("MPC_AUTH_TOKEN").unwrap_or_default()
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, Clone)]
/// Observatory option structure
///
/// * `place`: place name
/// * `latitude`: latitude
/// * `longitude`: longitude
/// * `altitude`: altitude
/// * `observatory_name`: observatory name
/// * `observer_name`: observer name
/// * `mpc_code`: mpc code
/// * `north_altitude`: north altitude to limit only visible objects
/// * `south_altitude`: south altitude to limit only visible objects
/// * `east_altitude`: east altitude to limit only visible objects
/// * `west_altitude`: west altitude to limit only visible objects
pub struct Observatory {
    /// Place name
    pub place: String,
    /// Latitude
    pub latitude: f32,
    /// Longitude
    pub longitude: f32,
    /// Altitude
    pub altitude: f32,
    /// Observatory name
    pub observatory_name: String,
    /// Observer name
    pub observer_name: String,
    /// MPC code
    pub mpc_code: String,
    /// North altitude to limit only visible objects
    pub north_altitude: i32,
    /// South altitude to limit only visible objects
    pub south_altitude: i32,
    /// East altitude to limit only visible objects
    pub east_altitude: i32,
    /// West altitude to limit only visible objects
    pub west_altitude: i32,
}
#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, Clone)]
/// Setting structure
///
/// * `general`: General settings structure
/// * `observatory`: Observatory settings structure
pub struct Settings {
    /// General settings structure
    pub general: General,
    /// Observatory settings structure
    pub observatory: Observatory,
}

/// Creates default settings for file creation
fn default_settings() -> Settings {
    let test = false; // true if you want to test
    let mut rng = rand::rng();
    let default_general: General = General {
        lang: "en".to_string(),
        mpc_auth_token: default_mpc_auth_token(),
    };
    let default_observatory: Observatory = match test {
        false => Observatory {
            place: "default".to_string(),
            latitude: rng.random_range(0.1..89.9) as f32,
            longitude: rng.random_range(0.1..179.9) as f32,
            altitude: rng.random_range(0.1..100.0) as f32,
            observatory_name: "default".to_string(),
            observer_name: "default".to_string(),
            mpc_code: "500".to_string(),
            north_altitude: 30,
            east_altitude: 45,
            south_altitude: 20,
            west_altitude: 70,
        },
        true => Observatory {
            place: "default".to_string(),
            latitude: 0.0,  // rng.random_range(0.1..89.9) as f32,
            longitude: 0.0, // rng.random_range(0.1..179.9) as f32,
            altitude: 0.0,  // rng.random_range(0.1..100.0) as f32,
            observatory_name: "default".to_string(),
            observer_name: "default".to_string(),
            mpc_code: "500".to_string(),
            north_altitude: 30,
            east_altitude: 45,
            south_altitude: 20,
            west_altitude: 70,
        },
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

/// Modifies field in config.toml file
///
/// * `key`: The key to be modified
/// * `value`: The value to be set
pub fn modify_field_in_file(key: String, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Get config directory
    let config_dir = dirs::config_local_dir()
        .ok_or("Failed to get config directory")?;
    let config_path = config_dir
        .join("asteroid_tui")
        .join("config.toml");
    let config_path_str = config_path
        .to_str()
        .ok_or("Failed to convert config path to string")?;

    // Read the file
    let contents = fs::read_to_string(config_path_str)?;

    // Parse the TOML contents into a HashMap
    let mut settings: toml::Value = toml::from_str(&contents)?;

    // Modify the field
    match key.as_str() {
        "lang" => settings["general"]["lang"] = toml::Value::String(value.to_string()),
        "place" => settings["observatory"]["place"] = toml::Value::String(value.to_string()),
        "latitude" => {
            settings["observatory"]["latitude"] = toml::Value::Float(
                parse_float64(value)
                    .map_err(|e| format!("Failed to parse latitude: {}", e))?
            )
        }
        "longitude" => {
            settings["observatory"]["longitude"] = toml::Value::Float(
                parse_float64(value)
                    .map_err(|e| format!("Failed to parse longitude: {}", e))?
            )
        }
        "altitude" => {
            settings["observatory"]["altitude"] = toml::Value::Float(
                parse_float64(value)
                    .map_err(|e| format!("Failed to parse altitude: {}", e))?
            )
        }
        "observatory_name" => {
            settings["observatory"]["observatory_name"] = toml::Value::String(value.to_string())
        }
        "observer_name" => {
            settings["observatory"]["observer_name"] = toml::Value::String(value.to_string())
        }
        "mpc_code" => settings["observatory"]["mpc_code"] = toml::Value::String(value.to_string()),
        "north_altitude" => {
            settings["observatory"]["north_altitude"] = toml::Value::Integer(
                parse_integer64(value)
                    .map_err(|e| format!("Failed to parse north_altitude: {}", e))?
            )
        }
        "south_altitude" => {
            settings["observatory"]["south_altitude"] = toml::Value::Integer(
                parse_integer64(value)
                    .map_err(|e| format!("Failed to parse south_altitude: {}", e))?
            )
        }
        "east_altitude" => {
            settings["observatory"]["east_altitude"] = toml::Value::Integer(
                parse_integer64(value)
                    .map_err(|e| format!("Failed to parse east_altitude: {}", e))?
            )
        }
        "west_altitude" => {
            settings["observatory"]["west_altitude"] = toml::Value::Integer(
                parse_integer64(value)
                    .map_err(|e| format!("Failed to parse west_altitude: {}", e))?
            )
        }
        _ => {}
    }

    // Serialize the updated settings back into a string
    let updated_contents = toml::to_string(&settings)?;

    // Write the updated contents back to the file
    fs::write(config_path_str, updated_contents)?;

    Ok(())
}
impl Settings {
    /// Loads settings from `~/.config/asteroid_tui/config.toml`, creating defaults on first run.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] if the config directory or file cannot be read or parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use asteroid_tui::settings::Settings;
    ///
    /// let settings = Settings::new()?;
    /// println!("Language: {}", settings.get_lang());
    /// # Ok::<(), config::ConfigError>(())
    /// ```
    pub fn new() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_local_dir()
            .ok_or_else(|| ConfigError::Message("Failed to get config directory".to_string()))?;
        let asteroid_tui_dir = config_dir.join("asteroid_tui");
        let config_file = asteroid_tui_dir.join("config.toml");
        
        // Check if directory exists, create if not
        if fs::metadata(&asteroid_tui_dir).is_err() {
            fs::create_dir_all(&asteroid_tui_dir)
                .map_err(|e| ConfigError::Message(format!("Failed to create config directory: {}", e)))?;
        }
        
        // Check if config file exists, create with defaults if not
        if fs::metadata(&config_file).is_err() {
            let default_settings = default_settings();
            let default_toml = toml::to_string(&default_settings)
                .map_err(|e| ConfigError::Message(format!("Failed to serialize default settings: {}", e)))?;
            
            let config_file_str = config_file
                .to_str()
                .ok_or_else(|| ConfigError::Message("Failed to convert config file path to string".to_string()))?;
            
            fs::write(config_file_str, default_toml)
                .map_err(|e| ConfigError::Message(format!("Failed to write default config file: {}", e)))?;
        }
        
        let config_file_str = config_file
            .to_str()
            .ok_or_else(|| ConfigError::Message("Failed to convert config file path to string".to_string()))?;
        
        let s = Config::builder()
            .add_source(File::with_name(config_file_str))
            .build()?;
        s.try_deserialize()
    }

    /// Get lang value from settings
    pub fn get_lang(&self) -> &String {
        &self.general.lang
    }

    /// Sets language value in config.toml
    ///
    /// * `lang`: lang to be set
    pub fn set_lang(&mut self, lang: String) -> Result<(), Box<dyn std::error::Error>> {
        //modify_field_in_file("lang".to_string(), &lang).expect("Error in setting lang, value");
        modify_field_in_file("lang".to_string(), &lang)?;
        Ok(())
    }

    /// Get MPC auth token from settings or environment variable
    pub fn get_mpc_auth_token(&self) -> String {
        std::env::var("MPC_AUTH_TOKEN")
            .unwrap_or_else(|_| self.general.mpc_auth_token.clone())
    }

    /// Sets settings in config.toml
    ///
    /// * `settings`: settings data to be set
    pub fn set_settings(&mut self, settings: Settings) -> Result<(), Box<dyn std::error::Error>> {
        // Update all fields directly
        self.observatory = settings.observatory;

        // Write to config file using serde directly
        let config_path = dirs::config_local_dir()
            .ok_or("Failed to get config dir")?
            .join("asteroid_tui")
            .join("config.toml");

        let toml = toml::to_string(&self)?;
        std::fs::write(config_path, toml)?;

        Ok(())
    }

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

    /// Get north altitude value from settings
    pub fn get_north_altitude(&self) -> &i32 {
        &self.observatory.north_altitude
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

    /// Gets all settings in one
    pub fn get_all_settings(&self) -> Settings {
        self.clone()
    }

    /// Sets latitude value in config.toml
    ///
    /// * `latitude`: latitude to be set
    pub fn set_latitude(&mut self, latitude: f32) {
        self.observatory.latitude = latitude;
    }

    /// Sets longitude value in config.toml
    ///
    /// * `longitude`: longitude to be set
    pub fn set_longitude(&mut self, longitude: f32) {
        self.observatory.longitude = longitude;
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
        assert!(s.get_north_altitude().is_positive());
        assert!(s.get_south_altitude().is_positive());
        assert!(s.get_east_altitude().is_positive());
        assert!(s.get_west_altitude().is_positive());
    }
}
