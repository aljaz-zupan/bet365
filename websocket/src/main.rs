use reqwest::Error;
use std::time::Duration;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        fetch_json().await?; // Fetch JSON request
        time::sleep(Duration::from_secs(60)).await; // Wait for 1 minute
    }
}

async fn fetch_json() -> Result<(), Error> {
    let url: &str = "http://106.52.68.20/b365/soccer/test/oneHd2allEv/C1-G15?lang=en"; // Replace with your API endpoint

    let response: reqwest::Response = reqwest::get(url).await?;
    let json = response.text().await?;

    println!("Fetched JSON: {:?}", json);
    println!("--------------------------");
    Ok(())
}
