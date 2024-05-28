use crate::auth::Auth;
use crate::error::GetError;
use reqwest::{Request, StatusCode, Url};
use serde::de::DeserializeOwned;

const RAWG_BASE_URI: &str = "https://api.rawg.io/api/";

type Client = reqwest::Client;

pub struct Rawg {
    client: Client,
    auth: Auth,
}

impl Rawg {
    pub fn new() -> Self {
        Rawg::default()
    }

    pub fn key(mut self, key: String) -> Self {
        self.auth = Auth::Key { key };
        self
    }

    async fn url(&self, route: String) -> Result<Url, GetError> {
        let key = match &self.auth {
            Auth::Key { key } => key,
            _ => return Err(GetError::KeyNotProvided),
        };

        let mut url = Url::parse(&format!("{RAWG_BASE_URI}{route}"))?;

        url.query_pairs_mut().append_pair("key", key);

        Ok(url)
    }

    pub async fn get<T>(&self, route: String) -> Result<T, GetError>
    where
        T: DeserializeOwned,
    {
        let url = self.url(route).await?;

        let client = &self.client;

        let response = client.get(url).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            StatusCode::UNAUTHORIZED => Err(GetError::KeyNotProvided),
            _ => Err(GetError::SiteError),
        }
    }

    pub async fn get_with_query<T>(
        &self,
        route: String,
        query: Option<Vec<(String, String)>>,
    ) -> Result<T, GetError>
    where
        T: DeserializeOwned,
    {
        let mut url = self.url(route).await?;
        let client = &self.client;

        if let Some(queries) = query {
            // Convert &str to String
            for (key, value) in queries {
                url.query_pairs_mut().append_pair(&key, &value);
            }
        }
        let response = client.get(url).send().await?;
        let json = response.json::<T>().await?;

        Ok(json)
    }
}

impl Default for Rawg {
    fn default() -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            auth: Auth::None,
        }
    }
}
