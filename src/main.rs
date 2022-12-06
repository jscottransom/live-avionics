
use reqwest::*;
use std::env;

// Collect API Key
fn get_key() -> String {
    let v = env::var("AVIATION_KEY").expect("$AVIATION_KEY is not set");
    v
}


#[tokio::main]
async fn main() -> Result<()> {
    let api_key = get_key();
    let url = format!(
        "http://api.aviationstack.com/v1/flights?access_key={}&limit=1",
        api_key
    );

    let res = reqwest::get(url).await?.text().await?;
    println!("{}", res);
    Ok(())
}


