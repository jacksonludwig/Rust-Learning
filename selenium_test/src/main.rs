use fantoccini::Client;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut c = Client::new("http://localhost:4444").await.expect("Failed to connect to webdriver");
    c.goto("https://smitesource.com/gods/1699").await?;

    c.find(fantoccini::Locator::Css(".build-card-list")).await?;

    let page_data = c.source().await?;
    println!("{}", page_data);

    c.close().await
}
