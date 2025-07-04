#![allow(unused_variables)]

use crate::password::Hasher;
use prelude::logger::info;
use std::collections::HashMap;
use userd::{Error, Response, User};

#[derive(Debug, Clone)]
pub struct Manager<'a> {
    pub hasher: Hasher<'a>,
    pub users: Vec<User>,
}

impl Manager<'_> {
    pub fn new(users: HashMap<String, User>) -> Self {
        let users = users
            .into_iter()
            .map(|(username, mut user)| {
                user.username = username;
                user
            })
            .collect::<Vec<User>>();

        Self {
            hasher: Hasher::new(),
            users,
        }
    }

    pub fn add_user(&self, user: &User) -> Result<Response, Error> {
        info!("add_user {user:#?}");

        Ok(Response::AddUser)
    }

    pub fn remove_user(&self, uid: u32) -> Result<Response, Error> {
        info!("remove_user {uid}");

        Ok(Response::RemoveUser)
    }

    pub fn set_password(
        &self,
        uid: u32,
        original_password: &str,
        new_password: &str,
    ) -> Result<Response, Error> {
        info!("set_password {uid} {original_password} {new_password}");

        Ok(Response::SetPassword)
    }

    pub fn verify_password(&self, uid: u32, password: &str) -> Result<Response, Error> {
        let user = self.fetch_user_by_uid(uid)?;

        match &user.password {
            Some(user_password) => {
                let is_hash = self.hasher.is_hash(user_password);
                if !is_hash && *user_password == password {
                    return Ok(Response::VerifyPassword(true));
                }

                self.hasher
                    .verify(password, user_password)
                    .map_err(|_| Error::WrongPassword)
                    .map(|()| Response::VerifyPassword(true))
            }
            None => Ok(Response::VerifyPassword(true)),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<Response, Error> {
        Ok(Response::HashPassword(self.hasher.hash(password)?))
    }

    pub fn fetch_user(&self, username: &str) -> Result<Response, Error> {
        Ok(Response::FetchUser(
            self.users
                .iter()
                .find(|user| user.username == username)
                .cloned()
                .map(|mut user| {
                    user.password = None;
                    user
                })
                .ok_or(Error::NoSuchUser)?,
        ))
    }

    fn fetch_user_by_uid(&self, uid: u32) -> Result<User, Error> {
        self.users
            .iter()
            .find(|user| user.uid == uid)
            .cloned()
            .ok_or(Error::NoSuchUser)
    }

    pub fn get_users(&self) -> Response {
        Response::GetUsers(
            self.users
                .clone()
                .into_iter()
                .map(|mut user| {
                    user.password = None;
                    user
                })
                .collect(),
        )
    }
}
