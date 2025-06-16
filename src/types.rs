use ipc::{define_commands, IpcError};
use prelude::thiserror::{self, Error};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

define_commands! {
    FetchUser(String) -> User;
    GetUsers -> Vec<User>;
    AddUser(User);
    RemoveUser(u32);
    VerifyPassword(u32, String) -> bool;
    HashPassword(String) -> String;
    SetPassword(u32, String, String);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub uid: u32,
    #[serde(skip)]
    pub username: String,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub shell: PathBuf,
    pub home: PathBuf,
}

#[derive(Debug, Error, Deserialize, Serialize)]
pub enum Error {
    #[error("No such user")]
    NoSuchUser,
    #[error("Wrong password")]
    WrongPassword,
    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Failed to hash password")]
    PasswordHashError,
    #[error("IPC Error: {0}")]
    IpcError(#[from] IpcError),
}

impl From<argon2::password_hash::Error> for Error {
    fn from(_: argon2::password_hash::Error) -> Self {
        Self::PasswordHashError
    }
}
