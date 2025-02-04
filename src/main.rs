mod file_util;

use std::env;
use reqwest;
use scraper::{Html, Selector};
use tokio;


fn main()  {
    let args: Vec<String> = env::args().collect();

    let url = if args.len() > 1 {
        &args[1]
    } else {
        "https://news.ycombinator.com/"
    };

    let file_path = if args.len() > 2 {
        &args[2]
    } else {
        "headlines.txt"
    };

    get_web_content(url, file_path).unwrap();
}

#[tokio::main]
async fn get_web_content(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse(".titleline > a").unwrap();

    let mut content = String::new();

    for element in document.select(&selector) {
        let title = element.text().collect::<Vec<_>>().join("");
        let link = element.value().attr("href").unwrap_or("No link found");

        content.push_str(&format!("- {} ({})\n", title, link));
    }
    file_util::save_to_file(file_path, content.as_str());

    Ok(())
}