pub mod configuration;
pub mod inference;
pub mod models;
pub mod template;
pub mod utils;

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
            // log received message and sender to console
            println!(
                "\nIncoming (user '{}'):\n{}\n",
                msg.author.name, msg.content
            );

            let result = openrouter(msg.to_owned()).await;

            match result {
                // successful response
                Ok(response) => {
                    println!("\nResponse:\n{}\n", &response);
                    if let Err(why) = msg.reply(&ctx.http, &response).await {
                        eprintln!("Err sending message: {:?}", why);
                    }
                }

                // unexpected response
                Err(why) => {
                    eprintln!("Err reading response: {:?}", why);
                }
            }
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!(
            "\n{} Connected:\n id => '{}', \n name => '{}'.",
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
        .expect("Err during client build");

    if let Err(why) = client.start().await {
        println!("Err starting client: {:?}", why);
    }
}
