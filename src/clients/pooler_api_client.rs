use std::env;
use dotenv::dotenv;
use reqwest;
use serde::{de::DeserializeOwned, Serialize};

pub struct PoolerApiClient {
    base_url: String,
    client: reqwest::Client,
    authorization_token: String,

}

impl PoolerApiClient {
    pub fn new() -> Self {
        dotenv().ok();
        let base_url = env::var("POOLER_BASE_URL").expect("Error loading env variable POOLER_BASE_URL");
        let secret_token = env::var("POOLER_SERCRET_TOKEN").expect("Error loading env variable POOLER_SERCRET_TOKEN");

        PoolerApiClient {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
            authorization_token: secret_token.to_string(),
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, reqwest::Error>
        where
            T: DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.get(&url).send().await?;
        response.json().await
    }

    pub async fn post<T, U>(&self, endpoint: &str, body: &U) -> Result<T, reqwest::Error>
        where
            T: DeserializeOwned,
            U: Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut request = self.client.post(&url).json(body);

        if let token = &self.authorization_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await?;
        response.json().await
    }
}