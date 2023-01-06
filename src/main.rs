pub mod services;

use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server started at 127.0.0.1::6969");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(services::ironman::get_ironmans)
            .service(services::challenge::get_challenges)
    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await
}
