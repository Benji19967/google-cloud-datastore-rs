use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, fmt};

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to setup settings");
}
// lazy_static! {
//     pub static ref GOOGLE_APPLICATION_CREDENTIALS: String =
//         env::var("GOOGLE_APPLICATION_CREDENTIALS")
//             .expect("GOOGLE_APPLICATION_CREDENTIALS must be set");
// }

#[derive(Debug, Clone, Deserialize)]
pub struct GCP {
    pub scope: String,
    pub domain_name: String,
    pub endpoint: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    // pub environment: String,
    pub gcp: GCP,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // let bxdx_env = env::var("BXDX_ENV").unwrap_or_else(|_| "development".into());

        let mut builder = Config::builder().add_source(File::with_name("config/settings"));
        // .add_source(File::with_name(&format!("config/{bxdx_env}")).required(false))
        // .add_source(File::with_name("config/local").required(false));

        builder
            .build()?
            // Deserialize (and thus freeze) the entire configuration.
            .try_deserialize()
    }
}
