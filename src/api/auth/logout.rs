use reqwest::Client;

use crate::schemas::api::auth::{
    logout_req_schema::LogoutRequestBody, logout_ret_schema::LogoutSchema,
};

pub async fn logout(client: &Client, url: &String) -> Result<LogoutSchema, String> {
    let json = LogoutRequestBody::new();
    let endpoint = "mge/service.sbr?serviceName=MobileLoginSP.logout&outputType=json";
    let mut post_url = format!("{}/{}", &url, &endpoint); // Formata a url para usar o token

    let last_char = url.chars().last().unwrap(); // Valida se a url contem "/" no final
    if last_char.eq(&'/') {
        post_url = format!("{}{}", &url, &endpoint); // Formata a url para usar o "/"
    }
    let response = client
        .post(post_url)
        .json(&json)
        .send()
        .await
        .expect("{\"message\":\"Erro sending request\"}"); // Faz a requisição http

    let resp: String = response
        .text_with_charset("utf-8")
        .await
        .expect("{\"message\":\"Erro during conversion\"}"); // tenta converter o arquivo para json

    let parsed_json: LogoutSchema = serde_json::from_str(&resp).unwrap();
    return Ok(parsed_json);
}
