use gcp_auth::AuthenticationManager;
use gcp_auth::Token;

use crate::settings::SETTINGS;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct AuthManager {
    pub gcp_auth_manager: AuthenticationManager,
}

impl AuthManager {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            gcp_auth_manager: AuthenticationManager::new().await?,
        })
    }

    pub async fn get_token(self) -> Result<Token> {
        Ok(self
            .gcp_auth_manager
            .get_token(&[SETTINGS.gcp.scope.as_str()])
            .await?)
    }
}
