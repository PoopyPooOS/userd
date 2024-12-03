use ipc_userd::{Command, Error, Response};
use linux_ipc::IpcChannel;
use logger::{fatal, warn};
use manager::Manager;
use nix::{
    sys::signal::{kill, Signal::SIGUSR1},
    unistd::Pid,
};
use std::{env, fs, process};

mod config;
mod manager;
mod password;

fn main() {
    logger::set_app_name!();
    let serviced_pid = env::var("SERVICED_PID")
        .unwrap_or_else(|_| {
            fatal!("SERVICED_PID environment variable not set, was this launched manually?");
            process::exit(1);
        })
        .parse::<i32>()
        .unwrap_or_else(|_| {
            fatal!("SERVICED_PID environment variable is not an integer");
            process::exit(1);
        });

    let config = match config::read() {
        Ok(config) => config,
        Err(err) => {
            fatal!(format!("Failed to read config file: {err:#?}"));
            process::exit(1);
        }
    };

    let user_manager = Manager::new(config.users);
    let mut ipc = IpcChannel::new("/tmp/ipc/services/userd.sock").expect("Failed to create IPC channel");

    for user in &user_manager.users {
        if !user.home.exists() {
            fs::create_dir_all(&user.home).expect("Failed to create user's home directory");
        }
    }

    match kill(Pid::from_raw(serviced_pid), SIGUSR1) {
        Ok(()) => (),
        Err(err) => {
            warn!(format!("Failed to send ready signal to serviced: {err:#?}"));
            process::exit(1);
        }
    }

    loop {
        let (received, reply) = ipc
            .receive::<Command, Result<Response, Error>>()
            .expect("Failed to receive content from IPC channel");

        let result = match received {
            Command::FetchUser(username) => user_manager.fetch_user(&username),
            Command::AddUser(user) => user_manager.add_user(&user),
            Command::RemoveUser(uid) => user_manager.remove_user(uid),
            Command::SetPassword(uid, original_password, new_password) => user_manager.set_password(uid, &original_password, &new_password),
            Command::VerifyPassword(uid, password) => user_manager.verify_password(uid, &password),
            Command::HashPassword(password) => Ok(user_manager.hash_password(&password)),
            Command::GetUsers() => Ok(user_manager.get_users()),
        };

        reply(result).unwrap_or_else(|err| eprintln!("Failed to reply to client: {err:#?}"));
    }
}
