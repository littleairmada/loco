use std::process::exit;
mod constants;
mod env_vars;
pub mod generate;
pub mod git;
pub mod prompt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    Dialog(#[from] dialoguer::Error),
}
#[derive(Debug)]
pub struct CmdExit {
    pub code: i32,
    pub message: Option<String>,
}

impl CmdExit {
    #[must_use]
    pub fn error_with_message(message: &str) -> Self {
        Self {
            code: 1,
            message: Some(format!("🙀 {message}")),
        }
    }
    #[must_use]
    pub fn ok_with_message(message: &str) -> Self {
        Self {
            code: 0,
            message: Some(message.to_string()),
        }
    }

    pub fn exit(&self) {
        if let Some(message) = &self.message {
            eprintln!("{message}");
        };

        exit(self.code);
    }
}
