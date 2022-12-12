use crate::infra::{get_key, pull_data, Request};
use serde::{Serialize, Deserialize};
use uuid::Uuid;




pub async fn run() -> Result<Request, mongodb::error::Error> {
    let api_key = get_key();
    let res_data = pull_data(api_key).await;

    let request_id = Uuid::new_v4();

    let full_request = Request {
        data: res_data.unwrap()
    };

    Ok(full_request)
}


