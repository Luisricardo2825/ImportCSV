use futures::future::join_all;
use serde::{de::DeserializeOwned, Serialize};

pub struct Promisse {
    url: String,
}

impl Promisse {
    pub fn new(url: String) -> Promisse {
        Self { url }
    }
    pub async fn all<C: Serialize, T: DeserializeOwned>(
        &self,
        jsons: Vec<C>,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let mut requests = vec![];
        let client = reqwest::Client::new();

        for ele in jsons {
            let resp = client.post(&self.url).json(&ele).send();
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
        let mut responses: Vec<T> = vec![];
        for ele in jsons {
            responses.push(ele.unwrap());
        }

        Ok(responses)
    }
}
