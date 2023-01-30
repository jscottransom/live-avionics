use crate::configuration::get_configuration;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Request {
    pub data: String,
}

// Execute Load operation into MongoDB database
pub async fn load_document(request: Request) -> Result<(), mongodb::error::Error> {
    let configuration = get_configuration().expect("Failed to retrieve config");

    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse(&configuration.database.connection_string()).await?;

    // Manually set an option
    client_options.app_name = Some("Rust Connection".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("avionics")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    tracing::info!("Connected successfully.");

    let uuid = Uuid::new_v4();
    let doc_id = format!("_{}", uuid);
    let doc = doc! {doc_id: request.data };

    let db = client.database("avionics");
    let collection = db.collection::<Document>("flight_data");
    collection.insert_one(doc, None).await?;

    Ok(())
}
