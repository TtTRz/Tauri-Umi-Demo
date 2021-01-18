#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use serde::{Deserialize, Serialize};

mod cmd;

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}
#[derive(Debug, Clone)]
struct CommandError<'a> {
    message: &'a str,
}

impl<'a> CommandError<'a> {
    fn new(message: &'a str) -> Self {
        Self { message }
    }
}

impl<'a> std::fmt::Display for CommandError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Tauri uses the `anyhow` lib so custom error types must implement std::error::Error
// and the function call should call `.into()` on it
impl<'a> std::error::Error for CommandError<'a> {}

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|_webview, arg| {
            use cmd::Cmd::*;
            match serde_json::from_str(arg) {
                Err(e) => Err(e.to_string()),
                Ok(command) => {
                    match command {
                        // definitions for your custom commands from Cmd here
                        SendMessage { payload } => {
                            println!("{:?}", payload)
                        }
                        SendAsyncMessage {
                            payload,
                            callback,
                            error,
                        } => tauri::execute_promise(
                            _webview,
                            move || {
                                let response = Response {
                                    message: "async success!",
                                };
                                Ok(response)
                            },
                            callback,
                            error,
                        ),
                    }
                    Ok(())
                }
            }
        })
        .build()
        .run();
}
