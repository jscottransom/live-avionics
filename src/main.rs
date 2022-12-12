use liveflights::handler::run;
use liveflights::infra::{load_document};
use liveflights::configuration::get_configuration;
use liveflights::telemetry::{get_subscriber, init_subscriber};
use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Serialize, Deserialize};
use tracing::Instrument;
use uuid::Uuid;
use bson::{Bson, Document};



#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let subscriber = get_subscriber("liveflights".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to retrieve config");
    
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!("Executing Flight Data Extraction Request",%request_id);
    // let request_span_guard= request_span.enter();
    let request = run().await?;

    tracing::info!("Loading document");
    load_document(request).await?;


    Ok(())
}


