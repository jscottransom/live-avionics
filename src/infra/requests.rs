use reqwest::*;
use std::env;


/// Collects the AVIATION_KEY environment variable and errors out with a message if it's not set.
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
    let v = env::var("AVIATION_KEY").expect("$AVIATION_KEY is not set");
    v
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
        "http://api.aviationstack.com/v1/flights?access_key={}&limit=1",
        api_key
    );

    let response = reqwest::get(url).await?.text().await?;

    Ok(response)
}