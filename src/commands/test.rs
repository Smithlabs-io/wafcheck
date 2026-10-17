use crate::error;
use crate::api;

pub fn test_key() -> Result<String, error::AppError> {
    let cloudflare_client = api::CloudflareClient::new()?;
    let raw_response = cloudflare_client.make_request("/user/tokens/verify", "GET", None, None)?.text()?;
    let parsed_response: serde_json::Value = serde_json::from_str(&raw_response)?;
    
    if parsed_response["success"].as_bool() == Some(true) {
        println!("Cloudflare API token is valid.");
    } else {
        println!("Cloudflare API token is invalid.");
    }

    Ok("Success".into())
}