#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use reqwest;
use scraper::{ Html, Selector };
use std::collections::HashMap;
extern crate dotenv;

const SEARCH_TEXT:&str = "品切れ";
const SELECTOR:&str = ".item-cart-add-area__add-button";
// リングフィットアドベンチャー
const URL:&str = "https://store.nintendo.co.jp/item/HAC_Q_AL3PA.html";
// テスト：動物の森→在庫あり
// const URL:&str = "https://store.nintendo.co.jp/item/HAC_J_ACBAA_32.html";

#[get("/")]
fn index() -> &'static str {
    let html = fetch_html(URL).expect("CAN'T FETCH!!!!!");
    let is_sold_out = is_contain(html, SELECTOR.to_owned(), SEARCH_TEXT.to_owned());

    if !is_sold_out {
        send_slack(format!(":tada: 品切れではなさそう {}", URL));
        "品切れではなさそう"
    } else {
        send_slack(format!(":tada: 品切れです {}", URL));
        "品切れです"
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

/**
 * HTMLを取得
 */
fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let mut response = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.122 Safari/537.36")
        .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("accept-encoding", "gzip, deflate, br")
        .send()?;
    response.text()
}

/**
 * 指定の文字がhtmlに含まれているか
 */
fn is_contain(html: String, selector: String, search_text: String) -> bool {
    let document = Html::parse_document(&html);
    let selector = Selector::parse(&selector).unwrap();

    let mut selected = document.select(&selector);


    let is_contain = selected.any(|item| {
        item.html().contains(&search_text)
    });

    is_contain
}

/**
 * slackのwebhookURLにPOST
 */
fn send_slack(content: String) {
    dotenv::dotenv().expect("Failed to read .env file");

    let url = std::env::var("SLACK_WEBHOOK_URL").expect("CAN'T GET `SLACK_WEBHOOK_URL` ENV");

    let mut map = HashMap::new();
    map.insert("text", content);

    let mut response = reqwest::Client::new()
        .post(&url)
        .json(&map)
        .send();
}


/**
 * 自分のブログでテスト
 */
#[test]
fn test_with_myblog() {
    let html = fetch_html("http://blog.morifuji-is.ninja/").expect("CAN'T FETCH!!!!!");
    let res = is_contain(html, ".post-title".to_string(), "Rust".to_string());

    assert_eq!(res, true);
}

/**
 * slackに送信テスト
 */
#[test]
fn test_slack() {
    send_slack("TEST".to_string());
}
