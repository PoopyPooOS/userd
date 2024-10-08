use crate::{commands::Commands, password::Hasher};
use ipc_userd::{Command, Error, Response, User};
use linux_ipc::IpcChannel;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    pub user: Vec<User>,
}

mod commands;
mod config;
mod password;
mod user;

fn main() {
    let user_manager = user::Manager::new();
    let mut ipc = IpcChannel::new("/tmp/ipc/serviced/userd.sock").expect("Failed to create IPC channel");
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

        // send ready message to init here because the service is now listening for messages

        let result = match received {
            Command::FetchUser(username) => commands.fetch_user(&username),
            Command::AddUser(user) => commands.add_user(&user),
            Command::RemoveUser(uid) => commands.remove_user(uid),
            Command::SetPassword(uid, original_password, new_password) => commands.set_password(uid, &original_password, &new_password),
            Command::VerifyPassword(uid, password) => commands.verify_password(uid, &password),
            Command::HashPassword(password) => Ok(commands.hash_password(&password)),
            Command::GetUsers() => Ok(commands.get_users()),
        };

        reply(result).unwrap_or_else(|err| eprintln!("Failed to reply to client: {:#?}", err));
    }
}
