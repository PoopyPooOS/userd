use ipc::{IpcError, Server};
use ipc_userd::Command;
use manager::Manager;
use std::fs;

mod config;
mod manager;
mod password;

fn main() -> Result<(), IpcError> {
    logger::set_app_name!();
    logger::panic::set_panic_hook();

    let serviced_pid = ipc_serviced::get_pid();

    let config = config::read().expect("Failed to read config file: {err:#?}");

    let user_manager = Manager::new(config.users);
    let ipc = Server::new("/tmp/ipc/services/userd.sock")?;

    for user in &user_manager.users {
        if !user.home.exists() {
            fs::create_dir_all(&user.home).expect("Failed to create user's home directory");
        }
    }

    ipc_serviced::ready(serviced_pid);

    ipc.on_client(move |mut client| {
        loop {
            let command = client.receive::<Command>()?;
            let result = match command {
                Command::FetchUser(username) => user_manager.fetch_user(&username),
                Command::AddUser(user) => user_manager.add_user(&user),
                Command::RemoveUser(uid) => user_manager.remove_user(uid),
                Command::SetPassword(uid, original_password, new_password) => {
                    user_manager.set_password(uid, &original_password, &new_password)
                }
                Command::VerifyPassword(uid, password) => {
                    user_manager.verify_password(uid, &password)
                }
                Command::HashPassword(password) => Ok(user_manager.hash_password(&password)),
                Command::GetUsers() => Ok(user_manager.get_users()),
            };

            client.send(result)?;
        }
    })
}
