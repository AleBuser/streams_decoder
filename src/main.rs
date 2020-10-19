use local::api::server;
use local::types::config::Config;

use std::fs::File;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    let endpoint = config.endpoint;
    let node = config.node;

    println!("Starting....");

    server::start(endpoint, node).await
}
