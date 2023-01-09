use actix_web::{get, web::Json, HttpResponse, Responder, Result};
use diesel::prelude::*;
use triathlon_api::{establish_connection, models::Post, schema::posts::dsl::*};

#[get("/")]
pub async fn all_triathlons() -> Result<impl Responder> {
    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }

    Ok(HttpResponse::Ok().json(Json({})))
}
