use tracing::subscriber::{Subscriber, set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use tracing_log::LogTracer;

/// Build a tracing subscriber, composed of multiple layers
/// 
/// impl return type suggests it can be any type, it simply must contain the specified traits.
/// Send + Sync are required to pass to the init_subscriber function
pub fn get_subscriber(
    name: String,
    env_filter: String
) -> impl Subscriber + Send + Sync {

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let formatting_layer = BunyanFormattingLayer::new("liveflights".into(), std::io::stdout); //

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)

}

/// Register a subscriber as global default to process span data.
/// 
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}



