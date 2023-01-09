use actix_web::{get, HttpResponse, Responder, Result};
use cached::proc_macro::cached;
use reqwest::Url;
use scraper::{Html, Selector};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChallengeApiType {
    date: String,
    distances: Vec<String>,
    title: String,
    website: String,
    image: String,
    country: String,
}

#[get("/challenge")]
pub async fn get_challenges() -> Result<impl Responder> {
    let query = get_challenge_datas().await;

    Ok(HttpResponse::Ok().json(query))
}

#[cached(time = 3600)]
async fn get_challenge_datas() -> Vec<ChallengeApiType> {
    let url = Url::parse("https://www.challenge-family.com/races/").unwrap();

    let test = reqwest::get(url).await.unwrap().text().await.unwrap();

    let document = Html::parse_document(&test);

    let mut values: Vec<ChallengeApiType> = Vec::new();

    let selector = Selector::parse("article").unwrap();

    let articles = document.select(&selector).collect::<Vec<_>>();

    for article in articles {
        let title_selector = Selector::parse("h3.the-title a").unwrap();
        let title = article.select(&title_selector).next().unwrap();

        let title_text = title.text().next().unwrap();
        let title_link = title.value().attr("href").unwrap();

        let subtitle_selector = Selector::parse(".portfolio-meta-wrapper .d-subtitle").unwrap();
        let date_and_country = article
            .select(&subtitle_selector)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap();
        let date_and_country = date_and_country.split(" - ").collect::<Vec<_>>();
        let date = date_and_country.get(0).unwrap().trim();
        let country = date_and_country.get(1).unwrap().trim();

        let image_selector = Selector::parse("img.portfolio-image").unwrap();
        let image = article.select(&image_selector).next().unwrap();
        let image_link = image.value().attr("src").unwrap();

        let categories_selector = Selector::parse(".portfolio-categories img").unwrap();

        let categories = article.select(&categories_selector);

        let mut distances: Vec<String> = Vec::new();
        for category in categories.collect::<Vec<_>>() {
            let category_alt = category.value().attr("alt").unwrap().to_lowercase();
            if category_alt.contains("rouvy") || category_alt.contains("remote") {
                continue;
            }

            distances.push(category_alt);
        }

        values.push(ChallengeApiType {
            date: date.to_string(),
            distances,
            title: format!("{}", title_text.to_string()),
            website: title_link.to_string(),
            image: image_link.to_string(),
            country: country.to_string(),
        });
    }

    return values;
}
