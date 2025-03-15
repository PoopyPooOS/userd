use ipc::{IpcError, define_commands};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

define_commands! {
    FetchUser(String) -> User;
    GetUsers() -> Vec<User>;
    AddUser(User) -> ();
    RemoveUser(u32) -> ();
    VerifyPassword(u32, String) -> bool;
    HashPassword(String) -> String;
    SetPassword(u32, String, String) -> ();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    #[error("IPC Error: {0}")]
    IpcError(#[from] IpcError),
}
