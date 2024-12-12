use ipc::IpcError;
use ipc_macro::define_commands;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

define_commands! {
    FetchUser(String) -> User;
    GetUsers() -> Vec<User>;
    AddUser(User) -> ();
    RemoveUser(u32) -> ();
    VerifyPassword(u32, String) -> bool;
    HashPassword(String) -> String;
    SetPassword(u32, String, String) -> ();
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    NoSuchUser,
    WrongPassword,
    UserAlreadyExists,
    IpcError(IpcError),
}

impl From<IpcError> for Error {
    fn from(value: IpcError) -> Self {
        Self::IpcError(value)
    }
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
