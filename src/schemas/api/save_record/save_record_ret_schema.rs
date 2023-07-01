use serde::Deserialize;
use serde::Serialize;

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
    pub entity: Entity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    #[serde(rename = "TIPPESSOA")]
    pub tippessoa: Tippessoa,
    #[serde(rename = "ATIVO")]
    pub ativo: Ativo,
    #[serde(rename = "NOMEPARC")]
    pub nomeparc: Nomeparc,
    #[serde(rename = "Cidade_AD_UF")]
    pub cidade_ad_uf: CidadeAdUf,
    #[serde(rename = "CLASSIFICMS")]
    pub classificms: Classificms,
    #[serde(rename = "CODPARC")]
    pub codparc: Codparc,
    #[serde(rename = "CODCID")]
    pub codcid: Codcid,
    #[serde(rename = "CLIENTE")]
    pub cliente: Cliente,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tippessoa {
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
pub struct Nomeparc {
    #[serde(rename = "$")]
    pub field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CidadeAdUf {
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
pub struct Codparc {
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
pub struct Cliente {
    #[serde(rename = "$")]
    pub field: String,
}
