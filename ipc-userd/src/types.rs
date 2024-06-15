use ipc_macro::define_commands;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// #[derive(Debug, Deserialize, Serialize)]
// pub enum Command {
//     /// username -> User
//     FetchUser(String),
//     /// None -> Vec<User>
//     GetUsers,
//     /// user struct -> None
//     AddUser(User),
//     /// uid -> None
//     RemoveUser(u32),
//     /// uid, password -> bool
//     VerifyPassword(u32, String),
//     /// password -> String
//     HashPassword(String),
//     /// uid, original password, new password -> None
//     SetPassword(u32, String, String),
// }

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
}

// #[derive(Debug, Deserialize, Serialize)]
// pub enum Response {
//     User(User),
//     Ok,
//     Vec(Vec<Response>),
//     String(String),
// }

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub uid: u32,
    pub username: String,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub shell: PathBuf,
    pub home: PathBuf,
}
