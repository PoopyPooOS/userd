use ipc::{Client, IpcError};
use std::path::Path;

mod types;
pub use types::*;

pub struct Userd {
    ipc: Client,
}

impl Userd {
    pub fn new(socket_path: impl AsRef<Path>) -> Result<Self, IpcError> {
        let ipc = Client::connect(socket_path)?;
        Ok(Self { ipc })
    }

    pub fn add_user(&mut self, user: User) -> Result<(), Error> {
        self.ipc.send(Command::AddUser(user))?;
        self.ipc.receive::<Result<Response, Error>>()??;
        Ok(())
    }

    pub fn remove_user(&mut self, uid: u32) -> Result<(), Error> {
        self.ipc.send(Command::RemoveUser(uid))?;
        self.ipc.receive::<Result<Response, Error>>()??;
        Ok(())
    }

    pub fn set_password(
        &mut self,
        uid: u32,
        original_password: String,
        new_password: String,
    ) -> Result<(), Error> {
        self.ipc
            .send(Command::SetPassword(uid, original_password, new_password))?;
        self.ipc.receive::<Result<Response, Error>>()??;
        Ok(())
    }

    /// Verify a password.
    /// () is returned if the password is correct
    pub fn verify_password(&mut self, uid: u32, password: String) -> Result<(), Error> {
        self.ipc.send(Command::VerifyPassword(uid, password))?;
        self.ipc
            .receive::<Result<Response, Error>>()?
            .map(|response| match response {
                Response::VerifyPassword(_) => (),
                _ => unreachable!("get_users can not return responses other than GetUsers"),
            })
    }

    pub fn fetch_user(&mut self, username: String) -> Result<User, Error> {
        self.ipc.send(Command::FetchUser(username))?;
        self.ipc
            .receive::<Result<Response, Error>>()?
            .map(|response| match response {
                Response::FetchUser(user) => user,
                _ => unreachable!("fetch_user can not return responses other than FetchUser"),
            })
    }

    pub fn get_users(&mut self) -> Result<Vec<User>, Error> {
        self.ipc.send(Command::GetUsers)?;
        self.ipc
            .receive::<Result<Response, Error>>()?
            .map(|response| match response {
                Response::GetUsers(users) => users,
                _ => unreachable!("get_users can not return responses other than GetUsers"),
            })
    }
}
