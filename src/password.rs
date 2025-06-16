use argon2::{
    password_hash::{self, rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};

/// Wrapper around [`argon2::Argon2`]
#[derive(Debug, Clone)]
pub struct Hasher<'a>(Argon2<'a>);

impl Hasher<'_> {
    pub fn new() -> Self {
        Self(Argon2::default())
    }

    pub fn hash(&self, content: &str) -> Result<String, password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(self.0.hash_password(content.as_bytes(), &salt)?.to_string())
    }

    pub fn verify(&self, password1: &str, password2: &str) -> Result<(), password_hash::Error> {
        let trimmed = password1.trim_end_matches('\n').as_bytes();
        let password_hash = PasswordHash::new(password2)?;

        self.0.verify_password(trimmed, &password_hash)
    }

    pub fn is_hash(&self, content: &str) -> bool {
        PasswordHash::new(content.trim_end_matches('\n')).is_ok()
    }
}
