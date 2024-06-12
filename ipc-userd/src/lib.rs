pub mod types;

use linux_ipc::IpcChannel;
pub use types::*;

pub struct Userd {
    ipc: IpcChannel,
}

impl Userd {
    pub fn new(socket_path: &str) -> Self {
        let ipc = IpcChannel::connect(socket_path).unwrap();

        Self { ipc }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), Error> {
        self.ipc
            .send::<_, Result<Response, Error>>(Command::AddUser(user))
            .expect("Failed to send command to userd")
            .expect("Failed to add user with userd")
            .map(|_| ())
    }

    pub fn remove_user(&mut self, uid: u32) -> Result<(), Error> {
        self.ipc
            .send::<_, Result<Response, Error>>(Command::RemoveUser(uid))
            .expect("Failed to send command to userd")
            .expect("Failed to remove user with userd")
            .map(|_| ())
    }

    pub fn set_password(&mut self, uid: u32, original_password: String, new_password: String) -> Result<(), Error> {
        self.ipc
            .send::<_, Result<Response, Error>>(Command::SetPassword(uid, original_password, new_password))
            .expect("Failed to send command to userd")
            .expect("Failed to set password with userd")
            .map(|_| ())
    }

    pub fn verify_password(&mut self, uid: u32, password: String) -> Result<(), Error> {
        self.ipc
            .send::<_, Result<Response, Error>>(Command::VerifyPassword(uid, password))
            .expect("Failed to send command to userd")
            .expect("Failed to verify password with userd")
            .map(|_| ())
    }

    pub fn fetch_user(&mut self, username: String) -> Result<User, Error> {
        self.ipc
            .send::<_, Result<Response, Error>>(Command::FetchUser(username))
            .expect("Failed to send command to userd")
            .expect("Failed to fetch user from userd")
            .map(|result| match result {
                Response::User(user) => user,
                _ => unreachable!(),
            })
    }

    pub fn get_users(&mut self) -> Result<Vec<User>, Error> {
        self.ipc
            .send::<_, Result<Response, Error>>(Command::GetUsers)
            .expect("Failed to send command to userd")
            .expect("Failed to fetch users from userd")
            .map(|result| match result {
                Response::Vec(users) => {
                    return users
                        .iter()
                        .map(|user| match user {
                            Response::User(user) => user.clone(),
                            _ => unreachable!(),
                        })
                        .collect::<Vec<User>>()
                }
                _ => unreachable!(),
            })
    }
}
