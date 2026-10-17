use crate::error;
use std::env;
use reqwest;

fn get_cf_token() -> Result<String, error::AppError> {
    let cloudflare_api_token = match env::var("CLOUDFLARE_API_TOKEN") {
        Ok(token) => Ok(token),
        Err(_) => {Err(error::AppError::MissingEnvVar("Cloudflare API Token not found.\nSet it via `export CLOUDFLARE_API_TOKEN='<YOUR_TOKEN_HERE>'".to_string()))},
    };
    return cloudflare_api_token;
}


pub struct CloudflareClient {
    client: reqwest::blocking::Client,
    headers: reqwest::header::HeaderMap,
    base_url: String,
}

impl CloudflareClient {
    pub fn new() -> Result<CloudflareClient, error::AppError> {
        let client = reqwest::blocking::Client::builder().build()?;

        let mut header_map = reqwest::header::HeaderMap::new();
        let cloudflare_token = get_cf_token()?;
        header_map.insert("Authorization", format!("Bearer {}", cloudflare_token).parse()?);

        Ok(Self {
            client,
            headers: header_map,
            base_url: "https://api.cloudflare.com/client/v4".to_string(),
        })
    }

    pub fn make_request(&self, path: &str, method: &str, _body: Option<&str>, headers: Option<reqwest::header::HeaderMap>) -> Result<reqwest::blocking::Response, error::AppError> {
        let method = match method {
            "GET" => reqwest::Method::GET,
            "POST" => reqwest::Method::POST,
            "PUT" => reqwest::Method::PUT,
            "DELETE" => reqwest::Method::DELETE,
            _ => return Err(error::AppError::CloudflareApiError(format!("Invalid method: {}", method)))
        };

        let mut header_map = self.headers.clone();
        if let Some(custom_headers) = headers {
            header_map.extend(custom_headers);
        }

        let request = self.client.request(method, format!("{}{}", self.base_url, path))
        .headers(header_map);

        let response = match request.send() {
            Ok(text) => Ok(text),
            Err(e) => Err(error::AppError::CloudflareApiError(e.to_string()))
        };

        return response;
    }
}