use crate::infra::{get_key, pull_data};

pub async fn run() -> Result<String, reqwest::Error> {
    let api_key = get_key();
    let data = pull_data(api_key).await;

    Ok(data.unwrap())
}
