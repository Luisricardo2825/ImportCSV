use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spreedsheet {
    #[serde(flatten)]
    pub local_fields: HashMap<String, Value>,
}
