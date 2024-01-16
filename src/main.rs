pub mod configuration;
pub mod inference;
pub mod utils;
pub mod models;

use configuration::CONFIG;
use inference::openrouter;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

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

            let result = openrouter(msg.to_owned()).await;

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
            "\n[*] Ready: \nlogged in as '{}', id => '{}', using name '{}'.",
            ready.user.name, ready.user.id, &CONFIG.bot.name
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
