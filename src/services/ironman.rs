use actix_web::{get, HttpResponse, Responder, Result};
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct IronmanApiValue {
    values: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct IronmanApiType {
    date: String,
    distance: String,
    title: String,
    website: String,
    image: String,
    city: String,
    country: String,
    region: String,
}

#[get("/ironman")]
pub async fn get_ironmans() -> Result<impl Responder> {
    let url = Url::parse("https://sheets.googleapis.com/v4/spreadsheets/1yLtxUETnuF3UZLmypYkAK6Vj4PE9Fo_BT-WsA4oE_YU/values/Race-Catalog?key=AIzaSyC9s2sNhwUZOUXJfnyt-cD4k4nUyY-3HBs").unwrap();

    let query = reqwest::get(url)
        .await
        .unwrap()
        .json::<IronmanApiValue>()
        .await
        .unwrap();

    let mut values: Vec<IronmanApiType> = Vec::new();

    for (index, value) in query.values.iter().enumerate() {
        if index == 0 {
            continue;
        }
        let date_day = value.get(0).unwrap().clone();
        let date_month = value.get(1).unwrap().clone();
        let date_year = value.get(2).unwrap().clone();

        let distance = value.get(3).unwrap().clone();
        let title = value.get(4).unwrap().clone();

        let region = value.get(5).unwrap().clone();
        let country = value.get(6).unwrap().clone();
        let city = value.get(7).unwrap().clone();
        let website = value.get(17).unwrap().clone();
        let image = value.get(18).unwrap().clone();

        values.push(IronmanApiType {
            date: format!("{} {} {}", date_day, date_month, date_year),
            distance,
            title,
            city,
            website,
            image,
            country,
            region,
        })
    }

    Ok(HttpResponse::Ok().json(values))
}
