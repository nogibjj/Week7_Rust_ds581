use scraper::{Html, Selector};
use std::error::Error;

pub async fn get_temperature(zip_code: &str) -> Result<String, Box<dyn Error>> {
    let url = format!("https://weather.com/weather/today/l/{}", zip_code);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse(".CurrentConditions--tempValue--3KcTQ").unwrap();
    let temperature = document.select(&selector).next().unwrap().text().collect::<String>();

    Ok(temperature)
}

