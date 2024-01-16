use serde::{Deserialize, Serialize};

/// Struct for json body request data
/// # Example request data structure
///```
///{
///     "model": "anthropic/claude-2",
///     // Vec<RequestMessage> allows for multiple messages
///     // from the POST request:
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
pub struct RequestBody {
    pub model: String,
    pub messages: Vec<RequestMessage>,
}

/// Struct for nested request message data. Serializes a single message
/// # Example request message structure
/// ```
/// // Parent wraps this content with a `Vec<...`
/// [
///     // which contains the `...RequestMessage>` struct:
///     {
///         "role": "user",
///         "content": "This is sent from the user."
///     }
/// ]
/// ```
/// # Overall request structure
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

impl RequestBody {
    /// Instantiate a new `RequestBody` object.
    pub fn new(model: &str, messages: Vec<RequestMessage>) -> Self {
        Self {
            model: model.to_string(),
            messages,
        }
    }
}

impl RequestMessage {
    /// Instantiate a new `RequestMessage` object.
    pub fn new(role: &str, content: String) -> Self {
        Self {
            role: role.to_string(),
            content,
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
}





