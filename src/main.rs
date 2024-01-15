use reqwest::Error;
use serde::{Serialize, Deserialize};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{fs, process::exit};
use toml;

/// Deserialize and parse secrets from TOML in `[config.toml]`
#[derive(Deserialize, Debug)]
struct Config {
    secrets: Secrets,
}

#[derive(Deserialize, Debug)]
struct Secrets {
    discord_token: String,
    openrouter_token: String,
}

/// JSONify POST request body content for OpenRouter
#[derive(Serialize, Deserialize)]
struct OrouterMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct RequestBody {
    model: String,
    messages: Vec<OrouterMessage>,
}

#[derive(Serialize,Deserialize)]
struct JsonContent {
    message: OrouterMessage,
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

async fn openrouter_offload(_user: String, msg: String) -> Result<String, Error> {
    let message = OrouterMessage {
        role: "user".to_string(),
        content: msg.to_string(),
    };

    let body = RequestBody {
        model: "cognitivecomputations/dolphin-mixtral-8x7b".to_string(),
        messages: vec![message],
    };

    let client = reqwest::Client::new();
    let res = client.post("https://openrouter.ai/api/v1/chat/completions")
        .json(&body)
        .header("Authorization", format!("Bearer {}", read_config().secrets.openrouter_token))
        .send()
        .await?;

    let res_body = res.text().await?;

    Ok(res_body)
}

async fn parse_json(content: JsonContent) -> String {
    let bot_response = content.message.content;
    bot_response
}

/// Removes the bot's ID from messages to the bot
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
            // test message => tags the message author with `<@msg.author.id>`.
            // alternatively, use `msg.author.name` to return the user's plaintext name.
            let result = openrouter_offload(msg.author.id.to_string(), format_msg(msg.content).to_string()).await;
            match result {
                Ok(response) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                        println!("Error sending message: {:?}", why);
                    }
                },
                Err(why) => {
                    println!("Err during OpenRouter endpoint fn call => {:?}", why);
                }
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} => connection ready.", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // consume token reader func
    let tokens = read_config();
    let token_discord = &tokens.secrets.discord_token;
    let _token_openrouter = &tokens.secrets.openrouter_token;

    // setup discord API bot intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token_discord, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client!");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
