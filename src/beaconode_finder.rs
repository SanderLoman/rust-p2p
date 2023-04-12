// work on getting *ONLY* the id of the beacon node aka the validator id 

use chrono::{DateTime, Local};
use colored::*;
use eyre::Result;
use reqwest::Client as AsyncClient;
use serde_json::Value;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

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

pub async fn beaconnode_finder() -> Result<()> {
    // Define the URL of the API endpoint for your Lighthouse beacon node.
    let api_url = "http://127.0.0.1:5052/eth/v1/beacon/states/head/validators";

    // Send the request to the Lighthouse beacon node.
    let client = AsyncClient::new();
    let response = client
        .get(api_url)
        .send()
        .await?
        .json::<Value>()
        .await?;

    // Extract the "data" field from the response.
    let data = response.get("data").unwrap_or(&Value::Null);

    // Define the number of entries per file.
    let max_files = 26;

    if let Value::Array(entries) = data {
        let entries_per_file = (entries.len() + max_files - 1) / max_files;

        // Create the output folder if it doesn't exist.
        let output_folder = Path::new("output_files");
        create_dir_all(&output_folder)?;

        let num_files = (entries.len() + entries_per_file - 1) / entries_per_file;
        for i in 0..num_files {
            let start = i * entries_per_file;
            let end = usize::min(start + entries_per_file, entries.len());

            // Create the output string for the current chunk of data.
            let output_str = format!("{:?}", &entries[start..end]);

            // Create the output file for the current chunk of data.
            let mut file = File::create(output_folder.join(format!("output_{}.txt", i + 1)))?;

            // Write the output string to the output file.
            file.write_all(output_str.as_bytes())?;

            println!(
                "{}",
                LogEntry {
                    time: Local::now(),
                    level: LogLevel::Info,
                    message: format!("Wrote entries {}-{} to output_{}.json", start + 1, end, i + 1),
                }
            );
        }
    }

    Ok(())
}