use lazy_static::lazy_static;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{fs, process::exit};
use toml;

lazy_static! {
    static ref CONFIG: Config = read_config();
}

/// Struct for the Config object.
#[derive(Deserialize, Debug)]
struct Config {
    pub secrets: Secrets,
}

#[derive(Deserialize, Debug)]
struct Secrets {
    pub discord_token: String,
    pub openrouter_token: String,
}

/// JSON struct to facilitate the POST request's `body`
#[derive(Serialize, Deserialize, Debug)]
struct OrouterMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OrouterResponse {
    message: Vec<OrouterMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestBody {
    model: String,
    messages: Vec<OrouterMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseBody {
    id: String,
    model: String,
    created: i32,
    object: String,
    choices: Vec<OrouterResponse>,
}

fn read_config() -> Config {
    let filename = "config.toml";
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Error reading from file: {}", filename);
        exit(1);
    });

    let config: Config = toml::from_str(&contents).unwrap_or_else(|_| {
        eprintln!("Error loading secrets from file: {}", filename);
        exit(1);
    });

    config
}

async fn openrouter_offload(msg: Message) -> Result<String, Error> {
    let discord_msg = OrouterMessage {
        role: "user".to_string(),
        content: format_msg(msg.content.to_string()),
    };
    let body = RequestBody {
        model: "mistralai/mixtral-8x7b-instruct".to_string(),
        messages: vec![discord_msg],
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .json(&body)
        .header(
            "Authorization",
            format!("Bearer {}", &CONFIG.secrets.openrouter_token),
        )
        .send()
        .await?;

    let body: serde_json::Value = serde_json::from_str(&res.text().await?).unwrap();
    let message = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .trim();
    dbg!(&body, &message);
    Ok(message.to_string())
}
/// Removes the `@<bot_name>` tag from beginning of a message
fn format_msg(msg: String) -> String {
    let remove_id = msg.split("<@1175812965224681502>");
    let formatted = remove_id.collect::<Vec<_>>().join("").trim().to_string();
    return format!("{}", formatted);
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_own(&ctx.cache) {
            return;
        }
        if msg.mentions_me(&ctx.http).await.unwrap() {
            // log the message to console
            println!(
                "\n[*] INCOMING (from '{}'):\n  ```\n  {}\n  ```\n",
                msg.author.name, msg.content
            );

            let result = openrouter_offload(msg.clone()).await;

            match result {
                Ok(response) => {
                    println!("[*] RES OK => \n  ```\n  {}\n  ```\n", &response);
                    if let Err(why) = msg.reply(&ctx.http, &response).await {
                        println!("[!] Err while sending message => {:?}", why);
                    }
                }
                Err(why) => {
                    println!("[!] Err reading response => {:?}", why);
                }
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "\n[*] Ready: \nlogged in as '{}', id => '{}'.",
            ready.user.name,
            ready.user.id
        );
    }
}

#[tokio::main]
async fn main() {
    let token_discord = &CONFIG.secrets.discord_token;

    // intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token_discord, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client.");

    if let Err(why) = client.start().await {
        println!("Err starting client => {:?}", why);
    }
}
