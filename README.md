# Bog

> On December 1, 2023, under orders from San Francisco county officials, Discord Inc. gave
> Clyde a lethal dose of pentobarbital. These orders came after he escaped his enclosure, where the
> ensuing hour-long rampage resulted in the mauling of several toddlers and an adult before he could
> be re-contained. Clyde will not be missed.

Bog is a Discord chatbot written in Rust using Serenity. He currently uses OpenRouter's API to generate 
responses to mentions using their range of LLMs ('range' being used pretty loosely as he currently 
is hardcoded to use Mistral's `Mixtral-8x7B` model).

In his current form, Bog is a very basic prototype and mostly just serves as a conduit to learn Rust.

## running bog

Bog uses a configuration file (`config.toml`), which should contain Discord & OpenRouter auth tokens:

```toml
[secrets]
discord_token="YOUR_DISCORD_TOKEN"
openrouter_token="YOUR_OPENROUTER_TOKEN"
```

If running Bog via `cargo run`, this should be the package's `src/` directory.
