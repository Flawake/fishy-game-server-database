use crate::domain::User;
use crate::repository::user::*;
use bcrypt::hash;
use rocket::async_trait;
use uuid::Uuid;

// Here you add your business logic here.
#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;
}

pub struct UserServiceImpl<T: UserRepository> {
    user_repository: T,
}

impl<R: UserRepository> UserServiceImpl<R> {
    // create a new function for UserServiceImpl.
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }
}

// Implement UserService trait for UserServiceImpl.
#[async_trait]
impl<R: UserRepository> UserService for UserServiceImpl<R> {
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        let mut user = user;
        // hash the password.
        user.password = hash_password(&user.password);
        // And let the user repository store the user.
        self.user_repository.create(user).await
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        // recieve the user from the database given a user_id.
        self.user_repository.from_uuid(user_id).await
    }
}

/// Hashes a password with bcrypt.
pub fn hash_password(password: &str) -> String {
    // Generate a hashed password
    hash(password, bcrypt::DEFAULT_COST).expect("Failed to hash password")
}
