#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    username: String,
    password: String,
    port: u16,
    host: String,
    database_name: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    //initialize configuration reader
    let mut settings = config::Config::default();

    //Add configuration settings from file named `configuration`
    //It will look for any top level file with an extension
    //that `config` knows how to parse: yaml, json, etc..
    settings.merge(config::File::with_name("configuration"))?;

    //Try to convert values read into `Settings` type
    settings.try_into()
}