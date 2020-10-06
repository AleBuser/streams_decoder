use actix_web::{web, App, HttpServer};
use actix_cors::Cors;


use crate::api::handlers;

pub async fn start(endpoint: String) -> std::io::Result<()> {
    println!("Started server at: {}", &endpoint);
    HttpServer::new(move || {
        App::new()
            .data(web::JsonConfig::default().limit(4096))
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600)
                    .finish(),
            )
            .service(
                web::resource("/decode_channel/{channel_root}").route(web::get().to(handlers::decode_channel)),
            )
    })
    .bind(endpoint)?
    .run()
    .await
}
