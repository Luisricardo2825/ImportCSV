use std::fmt::Debug;

use console::style;
use futures::{executor, future::join_all};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    api::auth::{
        login::{login, LoginRet},
        logout::logout,
    },
    schemas::{api::auth::login_req_schema::AccessData, config::builder_config::EnvConfig},
    ui::spinner::SpinnerBuilder,
};

pub struct PromisseSankhya {
    pub url: String,
    pub login_ret: LoginRet,
}

impl PromisseSankhya {
    pub async fn new(env: EnvConfig) -> PromisseSankhya {
        let login_ret = login(
            &env.url,
            AccessData {
                username: env.username,
                password: env.password,
            },
        )
        .await;
        Self {
            url: env.url,
            login_ret: login_ret.unwrap(),
        }
    }
    pub async fn save_all<C: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        jsons: Vec<C>,
    ) -> Result<Vec<(i32, T)>, Box<reqwest::Error>> {
        let pb = SpinnerBuilder::new("Inserting...");

        let mut requests = vec![];
        let LoginRet { client, root } = &self.login_ret;
        let jsession_token = String::from(&root.response_body.jsessionid.field); // Pega o jsession ID

        let last_char = &self.url.chars().last().unwrap();
        let endpoint = "mge/service.sbr?serviceName=CRUDServiceProvider.saveRecord&outputType=json&jsessionid=";

        let mut post_url = format!("{}/{}{}", &self.url, &endpoint, &jsession_token); // Formata a url para usar o token
        if last_char.eq(&'/') {
            post_url = format!("{}{}{}", &self.url, &endpoint, &jsession_token);
            // Formata a url para usar o token
        }
        let total_requests = jsons.len();
        for (idx, ele) in jsons.into_iter().enumerate() {
            pb.set_message(format!(
                "{}: {}/{total_requests}",
                style("Adding request").cyan().bold().dim().to_string(),
                idx + 1
            ));
            let resp = client.post(&post_url).json(&ele).send();
            requests.push(resp);
        }
        pb.set_message(format!(
            "{}",
            style("Calling SaveRecord API...")
                .cyan()
                .bold()
                .dim()
                .to_string()
        ));
        let responses = join_all(requests).await;
        pb.set_message(format!(
            "{}",
            style("Finished call").cyan().bold().dim().to_string()
        ));
        let mut bulk_responses = vec![];
        for ele in responses {
            if ele.is_ok() {
                pb.set_message(
                    style("Parsing responses...")
                        .cyan()
                        .bold()
                        .dim()
                        .to_string(),
                );
                let strval = ele.unwrap().text().await.unwrap();
                let json: T = serde_json::from_str(strval.as_str()).expect("Error Deserializing");
                bulk_responses.push(json);
            }
        }
        let jsons = bulk_responses;
        let mut responses: Vec<(i32, T)> = vec![];
        let mut count = 1;
        for ele in jsons {
            // if ele.is_err() {
            //     let a = Err::<Vec<(i32, T)>, Box<reqwest::Error>>(Box::new(ele.err().unwrap()));
            //     return a;
            // }
            responses.push((count, ele));
            count = count + 1;
        }
        let msg = format!(
            "{} {} {}",
            style("Imported").green().dim().bold().to_string(),
            count - 1,
            style("rows").green().dim().bold().to_string()
        );
        pb.finish(msg);
        Ok(responses)
    }

    pub async fn close(&self) {
        let LoginRet { client, root: _ } = &self.login_ret;

        let logout_res = logout(client, &self.url).await;
        if logout_res.is_err() {
            panic!("Error during logout");
        }
    }
}

impl Drop for PromisseSankhya {
    fn drop(&mut self) {
        let LoginRet { client, root: _ } = &self.login_ret;

        let v = executor::block_on(logout(client, &self.url));
        if v.is_ok() {
            println!("Connection clossed. Status:{}", v.unwrap().status)
        }
    }
}
