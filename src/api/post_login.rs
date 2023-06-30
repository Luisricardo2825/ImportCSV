use reqwest::Client;
use std::error::Error;

use crate::schemas::{
    login_req_schema::{Interno, LoginRequestBody, Nomusu},
    login_ret_schema::AccessData,
};

pub async fn post_login(
    url: &String,
    access_data: AccessData,
) -> Result<(String, Client), Box<dyn Error>> {
    // let resp = reqwest::get("https://httpbin.org/ip").await?.text().await?;
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .expect("Erro ao iniciar o client");
    let AccessData { username, password } = access_data;

    let json = LoginRequestBody {
        request_body: crate::schemas::login_req_schema::RequestBody {
            nomusu: Nomusu { field: username },
            interno: Interno { field: password },
            ..Default::default()
        },
        ..Default::default()
    };

    let post_url = format!(
        "{}/mge/service.sbr?serviceName=MobileLoginSP.login&outputType=json",
        url
    );

    let response = client.post(post_url).json(&json).send().await?;

    let body = response.text().await?;

    Ok((body, client))
}
