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

    for (_, soccer_match_value) in matches.as_object().unwrap() {
        /* let soccer_match: serde_json::Value = soccer_match_value.json(); */
        let score = soccer_match_value["score"].to_string().split("-");

        println!("{:?}\n", score);
    }

    Ok(())
}
