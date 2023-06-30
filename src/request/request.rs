use std::fmt::Debug;

use futures::{executor, future::join_all};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    auth::{
        login::{login, LoginRet},
        logout::logout,
    },
    schemas::login_ret_schema::AccessData,
};

pub struct PromisseSankhya {
    pub url: String,
    pub login_ret: LoginRet,
}

impl PromisseSankhya {
    pub async fn new(url: String, access_data: AccessData) -> PromisseSankhya {
        let login_ret = login(&url, access_data).await;
        Self {
            url,
            login_ret: login_ret.unwrap(),
        }
    }
    pub async fn save_all<C: Serialize + Debug, T: DeserializeOwned + Debug>(
        &self,
        jsons: Vec<C>,
    ) -> Result<Vec<(i32, T)>, Box<reqwest::Error>> {
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
        for ele in jsons {
            let resp = client.post(&post_url).json(&ele).send();
            requests.push(resp);
        }

        let responses = join_all(requests).await;
        let mut bulk_responses = vec![];
        for ele in responses {
            if ele.is_ok() {
                bulk_responses.push(ele.unwrap().json::<T>());
            }
        }
        let jsons = join_all(bulk_responses).await;
        let mut responses: Vec<(i32, T)> = vec![];
        let mut count = 1;
        for ele in jsons {
            if ele.is_err() {
                let a = Err::<Vec<(i32, T)>, Box<reqwest::Error>>(Box::new(ele.err().unwrap()));
                return a;
            }
            responses.push((count, ele.unwrap()));
            count = count + 1;
        }

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
