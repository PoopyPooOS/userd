use std::fs;

use ipc_userd::{Command, Error, Response, User};
use linux_ipc::IpcChannel;
use serde::Deserialize;
use user::UserManager;

use crate::{commands::Commands, password::Hasher};

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub user: Vec<User>,
}

mod commands;
mod config;
mod password;
mod user;

fn main() {
    let user_manager = UserManager::new();
    let mut ipc = IpcChannel::new("/tmp/init/services/userd.sock").expect("Failed to create IPC channel");
    let commands = Commands::new(Hasher::new(), &user_manager);

    for user in user_manager.get_users() {
        if !user.home.exists() {
            fs::create_dir_all(&user.home).expect("Failed to create user's home directory");
        }
    }

    loop {
        let (received, reply) = ipc
            .receive::<Command, Result<Response, Error>>()
            .expect("Failed to receive content from IPC channel");

        let result = match received {
            Command::FetchUser(username) => commands.fetch_user(username),
            Command::AddUser(user) => commands.add_user(user),
            Command::RemoveUser(uid) => commands.remove_user(uid),
            Command::SetPassword(uid, original_password, new_password) => commands.set_password(uid, original_password, new_password),
            Command::VerifyPassword(uid, password) => commands.verify_password(uid, password),
            Command::HashPassword(password) => commands.hash_password(password),
            Command::GetUsers() => commands.get_users(),
        };

        reply(result).unwrap_or_else(|err| eprintln!("Failed to reply to client: {:#?}", err));
    }
}
