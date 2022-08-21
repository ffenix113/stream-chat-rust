use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::{self, header};
use serde::{de::DeserializeOwned, ser::Serialize};

use crate::models::ApiError;

const LIB_USER_AGENT: &str = "rust/0.1.0";

#[allow(dead_code)]
pub struct StreamChat {
    pub client: reqwest::Client,
    base_url: String,
}

impl StreamChat {
    #[allow(unused)]
    pub fn new(api_key: String, api_secret: String) -> Option<Self> {
        let mut headers = header::HeaderMap::new();

        let token = genearte_token(api_secret);

        // Consider marking security-sensitive headers with `set_sensitive`.
        let mut auth_value = header::HeaderValue::from_str(&token).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        headers.insert(
            "x-stream-apikey",
            header::HeaderValue::from_str(&api_key).unwrap(),
        );
        headers.insert("Stream-Auth-Type", header::HeaderValue::from_static("jwt"));

        let client = reqwest::ClientBuilder::new()
            .user_agent(LIB_USER_AGENT)
            .default_headers(headers);

        let base_url = match std::env::var("STREAM_CHAT_URL") {
            Ok(val) => val,
            Err(_) => String::from("http://localhost:3030"),
        };

        Some(StreamChat {
            client: client.build().expect("failed to build a client"),
            base_url: base_url,
        })
    }

    #[allow(unused)]
    pub fn set_client(&mut self, client: reqwest::Client) {
        self.client = client;
    }
    #[allow(unused)]
    pub async fn get<Params, Resp: DeserializeOwned>(
        &self,
        url: &str,
        params: Option<&Params>,
    ) -> Result<Resp, crate::models::ApiError>
    where
        Params: ?Sized + Serialize,
    {
        let url = self.get_url(url);

        let req = self
            .client
            .request(reqwest::Method::GET, url)
            .query(&params);

        let resp = self.execute::<Resp>(req).await?;

        Ok(resp)
    }
    #[allow(unused)]
    pub async fn post<Req, Resp>(&self, url: &str, body: &Req) -> Result<Resp, ApiError>
    where
        Req: ?Sized + Serialize,
        Resp: DeserializeOwned,
    {
        let url = self.get_url(url);
        let body = serde_json::to_string(&body).unwrap();

        let req = self.client.request(reqwest::Method::POST, url).body(body);

        Ok(self.execute::<Resp>(req).await?)
    }
    #[allow(unused)]
    pub async fn delete<Params, Resp: DeserializeOwned>(
        &self,
        url: &str,
        params: Option<&Params>,
    ) -> Result<Resp, ApiError>
    where
        Params: ?Sized + Serialize,
    {
        let url = self.get_url(url);

        let req = self
            .client
            .request(reqwest::Method::DELETE, url)
            .query(&params);

        let resp = self.execute::<Resp>(req).await?;

        Ok(resp)
    }
    #[allow(unused)]
    pub async fn patch<Req, Resp>(&self, url: &str, body: &Req) -> Result<Resp, ApiError>
    where
        Req: ?Sized + Serialize,
        Resp: DeserializeOwned,
    {
        let url = self.get_url(url);
        let body = serde_json::to_string(&body).unwrap();

        let req = self.client.request(reqwest::Method::PATCH, url).body(body);

        Ok(self.execute::<Resp>(req).await?)
    }

    async fn execute<Resp: DeserializeOwned>(
        &self,
        req_builder: reqwest::RequestBuilder,
    ) -> Result<Resp, ApiError> {
        Ok(req_builder.send().await?.json::<Resp>().await?)
    }

    fn get_url(&self, path: &str) -> String {
        let trimmed_path = match path.strip_prefix('/') {
            Some(val) => val.to_string(),
            None => String::from(path),
        };

        format!("{}/{}", &self.base_url, trimmed_path)
    }
}

#[derive(serde_derive::Serialize)]
struct Claims {
    server: bool,
}

pub fn genearte_token(api_secret: String) -> String {
    let my_claims = Claims { server: true };

    let header = Header {
        alg: Algorithm::HS256,
        ..Default::default()
    };

    encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(api_secret.as_bytes()),
    )
    .expect("failed to create a JWT")
}
