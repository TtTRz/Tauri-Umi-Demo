use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SendMessagePayload {
    name: String,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    // your custom commands
    // multiple arguments are allowed
    // note that rename_all = "camelCase": you need to use "myCustomCommand" on JS
    SendMessage {
        payload: SendMessagePayload,
    },
    SendAsyncMessage {
        payload: SendMessagePayload,
        callback: String,
        error: String,
    },
}
