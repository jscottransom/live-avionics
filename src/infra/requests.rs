use reqwest::*;
use std::env;
use std::fs;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub settings: AviationSettings,
}

// All fields within the Setting struct need to be deserializable in order for the entire struct to be
// as well.
#[derive(serde::Deserialize)]
pub struct AviationSettings {
    pub key: String,
}

/// Collects the API access key and errors out with a message if the json config is invalid or missing.
///
/// # Arguments
///
/// * No arguments are passed to this function.
///
/// # Return String
///
/// The return value is either the AVIATION API Key or the Error Message.
///
pub fn get_key() -> String {
    let path = "aviation.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let json: AviationSettings = serde_json::from_str(&data).unwrap();

    json.key
}

/// Executes a GET request for real-time, flight information. Currently limited to 1 result.
///
/// # Arguments
///
/// * api_key String representation of the API Key provided by Aviation Stack.
///
/// # Return Result
///
///
pub async fn pull_data(api_key: String) -> Result<String> {
    let url = format!(
        "http://api.aviationstack.com/v1/flights?access_key={}&limit=3",
        api_key
    );

    let response = reqwest::get(url).await?.text().await?;
    println!("{}", response);

    Ok(response)
}
