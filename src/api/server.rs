use actix_web::{web, App, HttpServer};

use crate::api::handlers;

pub async fn start(endpoint: String) -> std::io::Result<()> {
    println!("Started server at: {}", &endpoint);
    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::resource("/decode_channel").route(web::get().to(handlers::decode_channel)),
            )
    })
    .bind(endpoint)?
    .run()
    .await
}
