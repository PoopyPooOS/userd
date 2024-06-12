use ipc_userd::User;
use std::fs;

use crate::{
    config::{self, Config},
    UserConfig,
};

#[derive(Debug)]
pub struct UserManager {
    config: Config,
}

impl UserManager {
    pub fn new() -> Self {
        let config = config::read_config();

        if !config.users_path.exists() {
            panic!("{} does not exist", config.users_path.display());
        }

        Self { config }
    }

    pub fn get_users(&self) -> Vec<User> {
        let unparsed = fs::read_to_string(&self.config.users_path).expect("Failed to read users file");

        toml::from_str::<UserConfig>(&unparsed).expect("Failed to parse users file").user
    }

    pub fn add_user(&self, user: User) {
        let as_toml = format!("\n\n[[user]]\n{}", toml::to_string_pretty(&user).expect("Failed to serialize user"));
        let users = fs::read_to_string(&self.config.users_path).expect("Failed to read users file");
        fs::write(&self.config.users_path, users.trim_end_matches('\n').to_owned() + &as_toml).expect("Failed to write to users file");
    }
}
