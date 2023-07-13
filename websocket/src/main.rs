use reqwest::Error;
use serde::Deserialize;
use std::time::Duration;
use tokio::time;

#[derive(Debug, Deserialize)]
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
    let matches: serde_json::Value = response.json().await?;

    if let Some(matches_json) = matches.as_object() {
        let mut total_goals = 0;

        for (_eventId, match_json) in matches_json {
            let match_data: Match = serde_json::from_value(match_json.clone())?;

            // Extract the integers from the score string
            let goals: Vec<u32> = match_data
                .score
                .split('-')
                .filter_map(|s| s.parse().ok())
                .collect();

            // Sum up the goals
            let match_goals: u32 = goals.iter().sum();
            total_goals += match_goals;
        }

        println!("Total Goals: {}", total_goals);
    }

    Ok(())
}
