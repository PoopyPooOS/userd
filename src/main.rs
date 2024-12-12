use ipc::{IpcError, Server};
use ipc_userd::Command;
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

fn main() -> Result<(), IpcError> {
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
    let ipc = Server::new("/tmp/ipc/services/userd.sock")?;

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

    ipc.on_client(move |mut client| loop {
        let command = client.receive::<Command>()?;
        let result = match command {
            Command::FetchUser(username) => user_manager.fetch_user(&username),
            Command::AddUser(user) => user_manager.add_user(&user),
            Command::RemoveUser(uid) => user_manager.remove_user(uid),
            Command::SetPassword(uid, original_password, new_password) => user_manager.set_password(uid, &original_password, &new_password),
            Command::VerifyPassword(uid, password) => user_manager.verify_password(uid, &password),
            Command::HashPassword(password) => Ok(user_manager.hash_password(&password)),
            Command::GetUsers() => Ok(user_manager.get_users()),
        };

        client.send(result)?;
    })
}
