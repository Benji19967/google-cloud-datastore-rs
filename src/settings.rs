use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to setup settings");
}

#[derive(Debug, Clone, Deserialize)]
pub struct GCP {
    pub scope: String,
    pub domain_name: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub gcp: GCP,
    pub gcp_project_id: String, // from env var `BXDX_GCP_PROJECT_ID`
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let builder = Config::builder()
            .add_source(File::with_name("config/settings"))
            .add_source(config::Environment::with_prefix("BXDX"));
        // .add_source(File::with_name(&format!("config/{bxdx_env}")).required(false))
        // .add_source(File::with_name("config/local").required(false));

        builder
            .build()?
            // Deserialize (and thus freeze) the entire configuration.
            .try_deserialize()
    }
}
