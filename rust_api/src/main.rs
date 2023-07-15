use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
struct Match {
    eventId: String,
    homeTeam: String,
    awayTeam: String,
    vsTeams: String,
    league: String,
    restTime: String,
    score: String,
    period: String,
    url: String,
    hasSubscribe: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        fetch_json().await?; // Fetch JSON request
        time::sleep(Duration::from_secs(60)).await; // Wait for 1 minute
    }
}

async fn fetch_json() -> Result<(), Error> {

    let url = "http://106.52.68.20/b365/soccer/test/allEv?lang=en"; // Updated API endpoint
    let response = reqwest::get(url).await?;

    let matches: serde_json::Value = response.json().await.unwrap();

    for soccer_match in matches.as_object().unwrap() {
        for match_value in soccer_match {
            println!("{:?}", match_value);
        }
    }

    Ok(())
}
