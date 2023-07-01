use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
// use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveRecord {
    pub service_name: String,
    pub request_body: RequestBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub data_set: DataSet,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSet {
    pub root_entity: String,
    pub include_presentation_fields: String,
    pub data_row: DataRow,
    pub entity: Entity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRow {
    #[serde(flatten)]
    pub local_fields: HashMap<String, HashMap<String, HashMap<String, String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tippessoa {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nomeparc {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Codcid {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ativo {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cliente {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Classificms {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub fieldset: Fieldset,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fieldset {
    pub list: String,
}
