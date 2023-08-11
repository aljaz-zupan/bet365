/* use chrono::Local; */
mod discord_bot;
use crate::discord_bot::GatewayIntents;
use discord_bot::{Client, Handler};
use dotenv::dotenv;
use reqwest::{Error, Response, StatusCode};
use serde::{Deserialize, Serialize};
/* use serenity::futures::future::ok; */
use serenity::framework::StandardFramework;
use serenity::http::Http;
use serenity::model::prelude::{Channel, ChannelId};
use std::sync::Arc;

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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = std::env::var("DISCORD_BOT_API_TOKEN").expect("DISCORD_BOT_API_TOKEN must be set.");
    /* let match_score: f32 = std::env::var("MATCH_SCORE").unwrap()
    .parse() {
        Ok(num) => num,
        Err(_) => 9.0
    }  */

    let framework = StandardFramework::new().configure(|c| c.prefix("~")); // set the bot's prefix to "~"

    let mut client = Client::builder(&token, GatewayIntents::DIRECT_MESSAGES)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client");

    // The http client is passed to the fetch task so it can make requests.
    let http = client.cache_and_http.http.clone();

    // Launch a new asynchronous task for fetching JSON.
    let fetch_task = tokio::spawn(async move {
        loop {
            println!("\n{}", chrono::Local::now());
            match fetch_json(&http).await {
                Ok(Some(msg)) => {
                    /* let user_id = UserId(282566557710680065); // Modify this to the  user ID you want to send the message to */
                    let channel_id = ChannelId(1067562408622694492);

                    if let Ok(Channel::Guild(channel)) = channel_id.to_channel(&http).await {
                        /* let _ = channel.say(&http, msg).await; */
                        if let Err(e) = channel.say(&http, msg).await {
                            eprintln!("Error sending message: {:?}", e);
                        }
                    }
                }
                Ok(None) => {}
                Err(e) => eprintln!("Error fetching JSON: {}", e),
            }
            time::sleep(Duration::from_secs(120)).await; // Wait for 1 minute
        }
    });

    // Start the client (this blocks the current thread until the client is shut down).
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    // We must wait on our fetch task before ending the program, otherwise it may be cancelled prematurely.
    fetch_task.await?;

    Ok(())
}

async fn fetch_json(_http: &Arc<Http>) -> Result<Option<String>, Error> {
    let url: &str = "http://106.52.68.20/b365/soccer/test/allEv?lang=en";

    let response_result: Result<Response, Error> = reqwest::get(url).await;

    if let Ok(response) = response_result {
        if response.status() == StatusCode::OK {
            let matches: serde_json::Value = response.json().await?;
            let mut messages = Vec::new();

            for (_, soccer_match_value) in matches.as_object().unwrap() {
                let match_league = soccer_match_value["league"].as_str().unwrap();
                let match_period = soccer_match_value["period"].as_str().unwrap();

                if !match_league.to_lowercase().contains("esoccer") {
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
                            continue;
                        }
                    };

                    let num_goals2: i32 = match score[1].to_string().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            eprintln!("Error: Could not parse {} as integer", score[1]);
                            continue;
                        }
                    };
                    let goals_total: i32 = num_goals1 + num_goals2;
                    let match_score = calculate_score(&goals_total, &minutes);

                    if match_score >= 9.0 && match_period == "SecondHalf" {
                        let msg = format!(
                            "result: {}-{}, total goals: {}, minutes: {}, score: **{:5}**, match: __{}__ , league: {}",
                            score[0], score[1], goals_total, minutes.to_string(), match_score.to_string(), soccer_match_value["vsTeams"].as_str().unwrap(), match_league
                        );
                        messages.push(msg);
                    }
                }
            }

            if !messages.is_empty() {
                let msg = format!("@here\n{}", messages.join("\n"));
                return Ok(Some(msg));
            }
        } else {
            println!("There was an oppsie: {}", response.status());
        }
    } else if let Err(err) = response_result {
        println!("Err: {}", err);
    }

    Ok(None)
}

fn calculate_score(goals_total: &i32, minutes: &i32) -> f32 {
    if (*minutes == 0) && (*goals_total == 0) {
        return 0.0;
    }
    ((*goals_total as f32) * 100.0) / (*minutes as f32)
}
