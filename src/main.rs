use liveflights::handler::run;
use liveflights::configuration::get_configuration;
use mongodb::{bson::doc, options::ClientOptions, Client};


#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    
    let configuration = get_configuration().expect("Failed to retrieve config");
    
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse(&configuration.database.connection_string())
            .await?;

    // Manually set an option
    client_options.app_name = Some("Rust Connection".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}


