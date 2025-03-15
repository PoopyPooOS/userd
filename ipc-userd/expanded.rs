use ipc::{Client, IpcError};
use std::path::Path;
pub use types::*;
pub mod types {
    use ipc::{IpcError, define_commands};
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    pub enum Command {
        FetchUser(String),
        GetUsers(),
        AddUser(User),
        RemoveUser(u32),
        VerifyPassword(u32, String),
        HashPassword(String),
        SetPassword(u32, String, String),
    }

    pub enum Response {
        FetchUser(User),
        GetUsers(Vec<User>),
        AddUser(()),
        RemoveUser(()),
        VerifyPassword(bool),
        HashPassword(String),
        SetPassword(()),
    }

    pub enum Error {
        NoSuchUser,
        WrongPassword,
        UserAlreadyExists,
        IpcError(IpcError),
    }

    impl From<IpcError> for Error {
        fn from(value: IpcError) -> Self {
            Self::IpcError(value)
        }
    }

    pub struct User {
        pub uid: u32,
        #[serde(skip)]
        pub username: String,
        pub display_name: Option<String>,
        pub password: Option<String>,
        pub shell: PathBuf,
        pub home: PathBuf,
    }
}

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
                Response::VerifyPassword(_) => {}
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("get_users can not return responses other than GetUsers",),
                    ));
                }
            })
    }
    pub fn fetch_user(&mut self, username: String) -> Result<User, Error> {
        self.ipc.send(Command::FetchUser(username))?;
        self.ipc
            .receive::<Result<Response, Error>>()?
            .map(|response| match response {
                Response::FetchUser(user) => user,
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("fetch_user can not return responses other than FetchUser",),
                    ));
                }
            })
    }
    pub fn get_users(&mut self) -> Result<Vec<User>, Error> {
        self.ipc.send(Command::GetUsers())?;
        self.ipc
            .receive::<Result<Response, Error>>()?
            .map(|response| match response {
                Response::GetUsers(users) => users,
                _ => {
                    ::core::panicking::panic_fmt(format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("get_users can not return responses other than GetUsers",),
                    ));
                }
            })
    }
}
