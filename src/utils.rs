/// Helper func to remove the `@<bot_id>` mention from beginning of a message
pub fn format_msg(msg: String) -> String {
    let remove_id = msg.split("<@1175812965224681502>");
    let formatted = remove_id.collect::<Vec<_>>().join("").trim().to_string();
    return format!("{}", formatted);
}
