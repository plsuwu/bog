# Bog

> On December 1, 2023, under orders from San Francisco county officials, Discord Inc. gave
> Clyde a lethal dose of pentobarbital. These orders came after he escaped his enclosure, where the
> ensuing hour-long rampage resulted in the mauling of several toddlers and an adult before he could
> be re-contained. Clyde will not be missed.

Bog is a Discord chatbot written in Rust using Serenity. He currently uses OpenRouter's API to generate 
responses to mentions using an LLM (Bog is currently hardcoded to run inference with the `Mixtral-8x7B-Instruct` model).

In his current form, Bog is a very basic prototype and mostly just serves as a conduit to learn Rust.

## Usage

Bog authorizes with the Discord and OpenRouter APIs via a configuration file, `config.toml`, which must contain both a Discord client secret 
and an OpenRouter API key:

```toml
[secrets]
discord_token="YOUR_DISCORD_TOKEN"
openrouter_token="YOUR_OPENROUTER_TOKEN"
```
> See [discord/oauth2](https://discord.com/developers/docs/topics/oauth2) or [openrouter/api-keys](https://openrouter.ai/docs#api-keys) for more info.

If running Bog via `cargo run`, Bog expects the config to be found in the package's root directory.
