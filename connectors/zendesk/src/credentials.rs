use std::fmt;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ZendeskCredentials {
    pub email: String,
    pub api_token: String,
}

impl fmt::Debug for ZendeskCredentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ZendeskCredentials")
            .field("email", &self.email)
            .field("api_token", &"<redacted>")
            .finish()
    }
}
