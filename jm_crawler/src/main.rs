use reqwest::Result;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<()> {
    let url: &str = "https://jm365.work/mJ8rWd";
    let user_agent: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0";

    let client: reqwest::Client = reqwest::Client::new();

    let resp = client
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await?
        .text()
        .await?;

    //print!("{:#?}", resp);

    let windown: Html = Html::parse_document(&resp);

    let range_selector = Selector::parse("div.word").unwrap();

    //print!("{:#?}", windown.select(&range_selector));

    let p_selector = Selector::parse("p").unwrap();
    let br_selector = Selector::parse("br").unwrap();

    for element in windown.select(&range_selector) {
        let urls_element = element.select(&p_selector).next().expect("Could no select p");
        let urls_element2 = urls_element.select(&br_selector).next().expect("Could no select br");
        //let urls = urls_element.text();
        println!("{:#?}", urls_element2.text());
    }

    Ok(())
}