use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PedidoReqBody {
    pub service_name: String,
    pub request_body: RequestBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub nota: Nota,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nota {
    pub cabecalho: HashMap::<String, HashMap<String, String>>,
    pub itens: Itens,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Itens {
    #[serde(rename = "INFORMARPRECO")]
    pub informarpreco: String,
    pub item: Vec<HashMap<String, HashMap<String, String>>>,
}
