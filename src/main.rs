use anyhow::{Context, Result};
use clap::Parser;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fmt::Error;
use toml::Table;

#[derive(Parser)]
struct Cli {
    /// Path to config toml
    #[arg(short = 'c', long = "config", required = false)]
    config: Option<String>,

    /// Use definition in config file
    #[arg(short = 'u', long = "user", required = false)]
    user: Option<String>,

    /// The bot users username
    #[arg(short = 'n', long = "name", required = false)]
    username: Option<String>,

    /// A URL to the bot users profile picture
    #[arg(short = 'p', long = "picture", required = false)]
    picture: Option<String>,

    /// Message
    #[arg(short = 'm', long = "message", required = false)]
    message: Option<String>,

    /// Discord webhook endpoint
    #[arg(short = 'e', long = "endpoint", required = false)]
    endpoint: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let name: String;
    let picture: String;
    let endpoint: String;

    if args.user.is_none() {
        name = args
            .username
            .ok_or(Error)
            .context("--name must be provided when not providing --user")?;

        picture = args
            .picture
            .ok_or(Error)
            .context("--picture must be provided when not providing --user")?;

        endpoint = args
            .endpoint
            .ok_or(Error)
            .context("--endpoint must be provided when not providing --user")?;
    } else {
        let config_file = std::path::Path::new(
            args.config
                .as_ref()
                .context("--config must be provided when using --user")?,
        );

        let file = std::fs::read_to_string(config_file)
            .map_err(|_e| format!("Cannot read config file {}", &args.config.as_ref().unwrap()))?;

        let config: Table = file.parse().unwrap();

        let requested_user = &config[args.user.as_ref().unwrap()];

        name = requested_user
            .get("name")
            .ok_or(Error)
            .map_err(|_e| {
                format!(
                    "Name value missing in \"{}\" config",
                    args.user.as_ref().unwrap()
                )
            })?
            .as_str()
            .unwrap()
            .to_string();

        picture = requested_user
            .get("picture")
            .ok_or(Error)
            .map_err(|_e| {
                format!(
                    "Picture value missing in \"{}\" config",
                    args.user.as_ref().unwrap()
                )
            })?
            .as_str()
            .unwrap()
            .to_string();

        endpoint = requested_user
            .get("endpoint")
            .ok_or(Error)
            .map_err(|_e| {
                format!(
                    "Endpoint value missing in \"{}\" config",
                    args.user.as_ref().unwrap()
                )
            })?
            .as_str()
            .unwrap()
            .to_string();
    }

    let message = args
        .message
        .ok_or(Error)
        .map_err(|_error| format!("--message must be provided"))?;

    let mut json = HashMap::new();
    json.insert("content", message);
    json.insert("username", name);
    json.insert("avatar_url", picture);

    println!("Sending: {:?}", json);

    let client = Client::new();
    let resp = client.post(endpoint).json(&json).send()?;

    if resp.status().is_success() {
        println!("Message sent!");
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }

    Ok(())
}
