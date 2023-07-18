use chrono::Local;
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
        time::sleep(Duration::from_secs(70)).await; // Wait for 1 minute
    }
}

#[allow(unused)]
async fn fetch_json() -> Result<(), Error> {
    let url = "http://106.52.68.20/b365/soccer/test/allEv?lang=en"; // Updated API endpoint
    let response = reqwest::get(url).await?;
    let time2 = Local::now();

    /* match (response.status()) {
        reqwest::StatusCode => println!("Sve oke :D"),
        _ => println!("Nešto nije ok :D, error: {}", response.status()),
    } */

    println!("time: {:?}, {}", time2, response.status());

    /* let matches: serde_json::Value = response.json().await.unwrap();

    for (_, soccer_match_value) in matches.as_object().unwrap() {
        /* let soccer_match: serde_json::Value = soccer_match_value.json(); */
        let score: Vec<String> = soccer_match_value["score"]
            .to_string()
            .split('-')
            .map(|s| s.trim().to_owned())
            .collect();
        let time: Vec<String> = soccer_match_value["restTime"]
            .to_string()
            .split(':')
            .map(|time| time.trim().to_owned())
            .collect();
        let minutes = time[0].clone();
        println!(
            "result -> {}-{}       time in minutes -> {} ",
            score[0], score[1], minutes
        );
    } */
    Ok(())
}
