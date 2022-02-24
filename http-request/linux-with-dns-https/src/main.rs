use reqwest;

#[tokio::main]
async fn main() {
    let res = reqwest::get("https://www.rust-lang.org").await.expect("error");
    let bytes = res.bytes().await.expect("error");

    println!("{:#?}", bytes);
}

