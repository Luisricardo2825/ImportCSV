use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvConfig {
    pub url: String,
    pub username: String,
    pub password: String,
    pub entity: String,
}
