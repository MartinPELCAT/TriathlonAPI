pub mod services;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("server started at 127.0.0.1::6969");
    
    HttpServer::new(|| App::new().service(services::ironman::get_ironmans))
        .bind(("127.0.0.1", 6969))?
        .run()
        .await
}
