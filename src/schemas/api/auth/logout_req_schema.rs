use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequestBody {
    pub service_name: String,
    pub status: String,
    pub pending_printing: String,
}

impl LogoutRequestBody {
    pub fn new() -> Self {
        Self {
            service_name: "MobileLoginSP.logout".to_string(),
            status: "1".to_string(),
            pending_printing: "false".to_string(),
        }
    }
}
