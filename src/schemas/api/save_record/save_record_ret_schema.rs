use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveRecordResponse {
    pub service_name: Option<String>,
    pub status: Option<String>,
    pub pending_printing: Option<String>,
    pub transaction_id: Option<String>,
    pub response_body: Option<ResponseBody>,
    pub status_message: Option<String>,
    pub ts_error: Option<TsError>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TsError {
    pub ts_error_code: String,
    pub ts_error_level: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    pub entities: Option<Entities>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub total: String,
    #[serde(flatten)]
    pub entity: HashMap<String, Value>,
}

