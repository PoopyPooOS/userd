#![deny(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

use ipc::Server;
use manager::Manager;
use prelude::logger;
use std::fs;
use userd::Command;

mod config;
mod manager;
mod password;

mod error {
    use prelude::logger::Log;
    use std::io;

    #[prelude::error_enum]
    pub enum Error {
        #[error("Failed to read config file: {0}")]
        NoConfig(io::Error),
        #[error("Invalid config: {0}")]
        InvalidConfig(Box<Log>),

        #[error("Failed to signal service daemon: {0}")]
        Serviced(#[from] serviced::Error),
        #[error("IPC Error: {0}")]
        IpcError(#[from] ipc::error::IpcError),
        #[error("I/O Error: {0}")]
        IO(#[from] io::Error),
    }
}

#[prelude::entry(error::Error)]
fn main() {
    logger::set_app_name!();
    logger::panic::set_panic_hook();

    let serviced_pid = serviced::get_pid()?;

    let config = config::read()?;

    let user_manager = Manager::new(config.users);
    let ipc = Server::new("/tmp/ipc/services/userd.sock")?;

    for user in &user_manager.users {
        if !user.home.exists() {
            fs::create_dir_all(&user.home)?;
        }
    }

    serviced::ready(serviced_pid)?;

    ipc.on_client(move |mut client| loop {
        let command = client.receive::<Command>()?;
        let result = match command {
            Command::FetchUser(username) => user_manager.fetch_user(&username),
            Command::AddUser(user) => user_manager.add_user(&user),
            Command::RemoveUser(uid) => user_manager.remove_user(uid),
            Command::SetPassword(uid, original_password, new_password) => {
                user_manager.set_password(uid, &original_password, &new_password)
            }
            Command::VerifyPassword(uid, password) => user_manager.verify_password(uid, &password),
            Command::HashPassword(password) => user_manager.hash_password(&password),
            Command::GetUsers => Ok(user_manager.get_users()),
        };

        client.send(result)?;
    })?;

    Ok(())
}
