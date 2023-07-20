/* use chrono::Local; */
use reqwest::{Error, Response, StatusCode};
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
        println!("\n{}", chrono::Local::now());
        fetch_json().await?; // Fetch JSON request
        time::sleep(Duration::from_secs(70)).await; // Wait for 1 minute
    }
}

async fn fetch_json() -> Result<(), Error> {
    let url: &str = "http://106.52.68.20/b365/soccer/test/allEv?lang=en"; // Updated API endpoint
    let response: Response = reqwest::get(url).await?;

    /* let current_time: chrono::DateTime<Local> = Local::now(); */

    if response.status() == StatusCode::OK {
        let matches: serde_json::Value = response.json().await.unwrap();

        for (_, soccer_match_value) in matches.as_object().unwrap() {
            /* let soccer_match: serde_json::Value = soccer_match_value.json(); */
            let score: Vec<&str> = soccer_match_value["score"]
                .as_str()
                .unwrap()
                .split('-')
                .map(|s| s.trim())
                .collect();
            let time: Vec<String> = soccer_match_value["restTime"]
                .as_str()
                .unwrap()
                .split(':')
                .map(|time| time.trim().to_owned())
                .collect();
            let minutes: i32 = time[0].parse().unwrap();

            let num_goals1: i32 = match score[0].to_string().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Error: Could not parse {} as integer", score[0]);
                    continue; // Skip to the next iteration of the loop
                }
            };

            let num_goals2: i32 = match score[1].to_string().parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Error: Could not parse {} as integer", score[1]);
                    continue; // Skip to the next iteration of the loop
                }
            };
            let goals_total: i32 = num_goals1 + num_goals2;
            let match_score = calculate_score(&goals_total, &minutes);
            println!(
                "result: {}-{}, total goals: {}, minutes: {}, score: {}",
                score[0], score[1], goals_total, minutes, match_score
            );
        }
    } else {
        println!("There was an error: {}", response.status());
    }

    Ok(())
}

fn calculate_score(goals_total: &i32, minutes: &i32) -> f32 {
    if (*minutes == 0) && (*goals_total == 0) {
        return 0.0;
    }
    ((*goals_total as f32) * 100.0) / (*minutes as f32)
}
