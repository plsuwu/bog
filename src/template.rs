use crate::models::{RequestBody, RequestMessage};
use crate::utils::format_msg;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use serenity::model::channel::Message;
use std::{fs, process::exit};
use toml;

lazy_static! {
    // this should eventually be non-static & globally accessible (fine for now though).
    // also refactor into models or configuration module maybe??
    pub static ref PROMPT: Prompt = Prompt::read_from_file("src/prompt_templates/bog.toml");
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prompt {
    pub bot: BotProps,
    pub system: SystemProps,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BotProps {
    pub name: String,
    pub description: String,
    pub scenario: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemProps {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    pub presence_penalty: f32,
    pub frequency_penalty: f32,
    pub top_p: f32,
    pub top_k: i32,
    pub transforms: Vec<String>,
    pub min_p: f32,
    pub top_a: f32,
}

impl Prompt {
    pub fn read_from_file(filename: &str) -> Self {
        let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Err reading from file: {}", filename);
            exit(1)
        });

        toml::from_str(&contents).unwrap_or_else(|err| {
            eprintln!("Error loading data from file: {},\n{}", filename, err);
            exit(1);
        })
    }
}

fn format_prompt(input: &str, user: &str, char: &str) -> String {
    let mut formatted_input = input.replace("{{user}}", &user);
    formatted_input = formatted_input.replace("{{char}}", &char);
    formatted_input
}

// disgusting
pub fn construct_body(msg: Message) -> RequestBody {
    let system_model = &PROMPT.system.model;
    let system_temperature = &PROMPT.system.temperature;
    let system_max_tokens = &PROMPT.system.max_tokens;
    let system_stream = &PROMPT.system.stream;
    let system_presence_penalty = &PROMPT.system.presence_penalty;
    let system_frequency_penalty = &PROMPT.system.frequency_penalty;
    let system_top_p = &PROMPT.system.top_p;
    let system_top_k = &PROMPT.system.top_k;
    let system_transforms = &PROMPT.system.transforms;
    let system_min_p = &PROMPT.system.min_p;
    let system_top_a = &PROMPT.system.top_a;

    let msg_author = &msg.author.name;
    let msg_content = format_msg(msg.content);

    let bot_name = &PROMPT.bot.name;
    let bot_description = format_prompt(&PROMPT.bot.description, msg_author, bot_name);
    let bot_scenario = format_prompt(&PROMPT.bot.scenario, msg_author, bot_name);

    let scenario_prompt = RequestMessage::new("system", bot_scenario);
    let personality_prompt = RequestMessage::new("system", bot_description);
    let user_message = RequestMessage::new("user", msg_content);

    let request_body = RequestBody::new(
        system_model,
        *system_temperature,
        *system_max_tokens,
        *system_stream,
        *system_presence_penalty,
        *system_frequency_penalty,
        *system_top_p,
        *system_top_k,
        system_transforms.to_vec(),
        *system_min_p,
        *system_top_a,
        vec![scenario_prompt, personality_prompt, user_message],
    );

    request_body
}
