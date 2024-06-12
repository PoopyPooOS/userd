#![allow(unused_variables)]
use ipc_userd::{Error, Response, User};

use crate::{password::Hasher, user::UserManager};

#[derive(Debug)]
pub struct Commands<'a> {
    hasher: Hasher<'a>,
    user_manager: &'a UserManager,
}

impl<'a> Commands<'a> {
    pub fn new(hasher: Hasher<'a>, user_manager: &'a UserManager) -> Self {
        Self { hasher, user_manager }
    }

    pub fn add_user(&self, user: User) -> Result<Response, Error> {
        println!("[ userd ] add_user {:?}", user);

        Ok(Response::Ok)
    }

    pub fn remove_user(&self, uid: u32) -> Result<Response, Error> {
        println!("[ userd ] remove_user {}", uid);

        Ok(Response::Ok)
    }

    pub fn set_password(&self, uid: u32, original_password: String, new_password: String) -> Result<Response, Error> {
        println!("[ userd ] set_password {} {} {}", uid, original_password, new_password);

        Ok(Response::Ok)
    }

    pub fn verify_password(&self, uid: u32, password: String) -> Result<Response, Error> {
        let user = self.fetch_user_by_uid(uid)?;

        match &user.password {
            Some(user_password) => {
                let is_hash = self.is_hash(user_password);
                if !is_hash && *user_password == password {
                    return Ok(Response::Ok);
                }

                self.hasher
                    .verify(&password, user_password)
                    .map_err(|_| Error::WrongPassword)
                    .map(|_| Response::Ok)
            }
            None => Ok(Response::Ok),
        }
    }

    pub fn fetch_user(&self, username: String) -> Result<Response, Error> {
        let users = self.user_manager.get_users();
        let user = users
            .iter()
            .find(|user| user.username == username)
            .cloned()
            .ok_or(Error::NoSuchUser)?;

        Ok(Response::User(user))
    }

    fn fetch_user_by_uid(&self, uid: u32) -> Result<User, Error> {
        let users = self.user_manager.get_users();
        let user = users.iter().find(|user| user.uid == uid).cloned().ok_or(Error::NoSuchUser)?;

        Ok(user)
    }

    fn is_hash(&self, password: &str) -> bool {
        self.hasher.is_hash(password)
    }

    pub fn get_users(&self) -> Result<Response, Error> {
        let users = self
            .user_manager
            .get_users()
            .iter()
            .map(|user| Response::User(user.clone()))
            .collect::<Vec<Response>>();

        Ok(Response::Vec(users))
    }
}
