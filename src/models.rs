use serde::{Deserialize, Serialize};

/// Struct for settings in json request.
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
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


impl RequestBody {
    /// Instantiates a new [`RequestBody`] object.
    pub fn new(
        // i do not believe this is how you do this
        model: &str, temp: f32, max_t: i32, stream: bool,
        p_pen: f32, f_pen: f32, top_p: f32, top_k: i32,
        transforms: Vec<String>, min_p: f32, top_a: f32,
        messages: Vec<RequestMessage>
        ) -> Self {
        Self {
            messages,
            settings: Settings {
                model: model.to_string(),
                temperature: temp,
                max_tokens: max_t,
                stream,
                presence_penalty: p_pen,
                frequency_penalty: f_pen,
                top_p,
                top_k,
                transforms,
                min_p,
                top_a
            },
        }
    }
}

/// Struct for json body request data
/// # Example request data structure
///```
///{
///     "model": "anthropic/claude-2",
///     // Wrapping `<RequestMessage>` in a vec is mandatory, but also
///     // allows us to append multiple messages to a POST request.
///     "messages": [
///         {
///            "role": "user",
///            "content": "This is sent from the user.",
///         },
///         {
///             "role": "system",
///             "content": "Respond to the user's message as a helpful assistant."
///         }
///     ]
///}
///```
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessageBody {
    pub model: String,
    pub messages: Vec<RequestMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestBody {
    pub messages: Vec<RequestMessage>,
    pub settings: Settings,
}

/// Struct for nested request message data. Serializes a single message
/// # Example request message structure
/// ```
/// // Parent wraps this content with a `Vec<...
/// [
///     // which contains the ...RequestMessage>` struct:
///     {
///         "role": "user",
///         "content": "This is sent from the user."
///     }
/// ]
/// ```
/// ## Overall request structure
/// ```
///{
///     "model": "anthropic/claude-2",
///     "messages": [
///         {
///            "role": "user",
///            "content": "Hello, LLM!"
///         }
///    ]
///}
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage {
    pub role: String,
    pub content: String,
}

impl RequestMessage {
    /// Instantiate a new `RequestMessage` object.
    pub fn new(role: &str, content: String) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

/// Struct for json body of response
/// # Example response data structure
/// ```
/// {   // Vec<ResponseMessage>:
///     "choices": [{
///         "message": {
///             "content": "This is a response from the API."
///         }
///     }],
///     "id": "gen-0YV8H...",
///     "model": "anthropic/claude-2",
///     "created": 1621982371,
///     "object": "chat.completion",
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    pub choices: Vec<ResponseMessage>,
    pub id: String,
    pub model: String,
    pub created: i32,
    pub object: String,
}

/// Struct for nested response message data. Serializes a single message
/// # Example response message structure
/// ```
/// {   // `choices` holds a `Vec<..`
///    "choices": [
///     // which is filled with `..ResponseMessage>` objects that contain
///     // `message` K,V pairs:
///     { "message":
///         // ...which contains the `content`:`<ResponseContent>`
///         // struct (this is the target when parsing a response).
///         { "content": "This is a response from the API." }
///     }
///     // ...
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage {
    pub message: ResponseContent,
}

/// Struct for nested response message content. Serializes a single message key and its string
/// value.
/// # Example structure:
/// ```
/// { "choices": [
///         { "message" :
///             { "content": "This is a response from the API." }
///         }
///     ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseContent {
    pub content: String,
    pub role: String,
}





