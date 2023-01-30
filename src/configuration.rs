#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
}

// All fields within the Setting struct need to be deserializable in order for the entire struct to be
// as well.
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub database: String,
    pub options: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "mongodb+srv://{}:{}@{}/{}",
            self.username, self.password, self.database, self.options
        )
    }
}
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize the config reader
    let mut settings = config::Config::default();

    // Add the config valuesfrom a file named `configuration`.
    settings.merge(config::File::with_name("configuration"))?;
    settings.try_into()
}
