use std::env::args;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = args();
    args.next();
    let url = args.next().unwrap();
    let file = args.next().unwrap();

    let html = reqwest::get(&url).await?.text().await?;

    fs::write(&file, html2md::parse_html(&html).as_bytes()).await?;

    Ok(())
}
