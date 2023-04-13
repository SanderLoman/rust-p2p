use chrono::{DateTime, Local};
use colored::*;
use eyre::Result;
use reqwest::Client as AsyncClient;
use serde::Serialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Clone)]
struct ResponseTime {
    index: usize,
    pubkey: String,
    elapsed: std::time::Duration,
}

#[derive(Debug)]
struct LogEntry {
    time: DateTime<Local>,
    level: LogLevel,
    message: String,
}

#[derive(Debug)]
#[allow(unused)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_str = format!("{}", self.time.format("%m-%d|%H:%M:%S%.3f"));
        let msg_str = self.message.as_str();

        let level_str = match self.level {
            LogLevel::Info => "INFO".green(),
            LogLevel::Warning => "WARN".yellow(),
            LogLevel::Error => "ERRO".red(),
            LogLevel::Critical => "CRIT".magenta(),
        };

        write!(f, "{} [{}] {}", level_str, time_str, msg_str)
    }
}

pub async fn beaconnode_finder() -> Result<Vec<String>> {
    let api_url = "http://127.0.0.1:5052/eth/v1/beacon/states/head/validators";
    let client = AsyncClient::new();
    let response = client.get(api_url).send().await?.json::<Value>().await?;

    let data = response.get("data").unwrap_or(&Value::Null);

    // let max_files = 30;
    let mut pubkeys = vec![];

    if let Value::Array(entries) = data {
        // let entries_per_file = (entries.len() + max_files - 1) / max_files;

        pubkeys = entries
            .iter()
            .filter_map(|entry| {
                entry
                    .get("validator")
                    .and_then(|validator| validator.get("pubkey"))
                    .and_then(|pubkey| pubkey.as_str().map(str::to_string))
            })
            .collect();

        // let output_folder = Path::new("output_files");
        // create_dir_all(&output_folder)?;

        // let num_files = (entries.len() + entries_per_file - 1) / entries_per_file;
        // for i in 0..num_files {
        //     let start = i * entries_per_file;
        //     let end = usize::min(start + entries_per_file, entries.len());
        //     let output_str = format!("{:?}", &entries[start..end]);
        //     let mut file = File::create(output_folder.join(format!("output_{}.txt", i + 1)))?;
        //     file.write_all(output_str.as_bytes())?;

        //     println!(
        //         "{}",
        //         LogEntry {
        //             time: Local::now(),
        //             level: LogLevel::Info,
        //             message: format!(
        //                 "Wrote entries {}-{} to output_{}.json",
        //                 start + 1,
        //                 end,
        //                 i + 1
        //             ),
        //         }
        //     );
        // }
    }

    Ok(pubkeys)
}

async fn test_validator_response_time(pubkeys: Vec<String>) -> Result<()> {
    let client = AsyncClient::new();
    let mut response_times = vec![];

    for (index, pubkey) in pubkeys.iter().enumerate() {
        let api_url = format!(
            "http://127.0.0.1:5052/eth/v1/beacon/states/head/validators/{}",
            pubkey
        );

        let start = std::time::Instant::now();
        let response = client.get(&api_url).send().await?;
        let elapsed = start.elapsed();

        if response.status().is_success() {
            response_times.push(ResponseTime {
                index,
                pubkey: pubkey.clone(),
                elapsed,
            });
            println!(
                "{}",
                LogEntry {
                    time: Local::now(),
                    level: LogLevel::Info,
                    message: format!(
                        "Validator with index {} and pubkey {} responded in {:?}",
                        index, pubkey, elapsed
                    ),
                }
            );
        } else {
            println!(
                "{}",
                LogEntry {
                    time: Local::now(),
                    level: LogLevel::Warning,
                    message: format!(
                        "Validator with pubkey {} failed to respond: {}",
                        pubkey,
                        response.status()
                    ),
                }
            );
        }
    }

    // Sort response_times by elapsed time in ascending order
    response_times.sort_by(|a, b| match a.elapsed.cmp(&b.elapsed) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.index.cmp(&b.index),
    });

    let top_50_response_times = response_times
        .iter()
        .take(50)
        .cloned()
        .collect::<Vec<ResponseTime>>();

    // Create the responses directory and write the top 50 response times to a JSON file
    let responses_folder = Path::new("responses");
    create_dir_all(&responses_folder)?;
    let output_file = responses_folder.join("top_50_response_times.json");
    let json_data = serde_json::to_string_pretty(&top_50_response_times)?;
    let mut file = File::create(output_file)?;
    file.write_all(json_data.as_bytes())?;

    println!(
        "{}",
        LogEntry {
            time: Local::now(),
            level: LogLevel::Info,
            message:
                "Top 50 fastest response times written to responses/top_50_response_times.json"
                    .to_string(),
        }
    );

    Ok(())
}

pub async fn main() -> Result<()> {
    // Call the beaconnode_finder function to get a list of public keys.
    let pubkeys = beaconnode_finder().await?;

    // Call the test_validator_response_time function with the public keys.
    test_validator_response_time(pubkeys).await?;

    Ok(())
}
