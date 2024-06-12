use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    /// username -> User
    FetchUser(String),
    /// None -> Vec<User>
    GetUsers,
    /// user struct -> None
    AddUser(User),
    /// uid -> None
    RemoveUser(u32),
    /// uid, password -> bool
    VerifyPassword(u32, String),
    /// uid, original password, new password -> None
    SetPassword(u32, String, String),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    NoSuchUser,
    WrongPassword,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Response {
    User(User),
    Ok,
    Vec(Vec<Response>),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub uid: u32,
    pub username: String,
    pub display_name: Option<String>,
    pub password: Option<String>,
    #[serde(default = "default_shell")]
    /// The default shell is /sbin/shell
    pub shell: PathBuf,
    pub home: PathBuf,
}

fn default_shell() -> PathBuf {
    PathBuf::from("/sbin/shell")
}
