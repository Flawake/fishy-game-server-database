use crate::domain::User;
use chrono::Utc;
use crate::repository::user::*;
use bcrypt::hash;
use rocket::async_trait;
use uuid::Uuid;

// Here you add your business logic here.
#[async_trait]
pub trait UserService: Send + Sync {
    async fn create(&self,
        name: String,
        email: String,
        password: String,
    ) -> Result<(), sqlx::Error>;

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
   async fn create(&self,
        name: String,
        email: String,
        password: String,
    ) -> Result<(), sqlx::Error> {
        let salt = Uuid::new_v4().to_string();
        let user = User {
            user_id: Uuid::new_v4(),
            salt: salt.clone(),
            name,
            email,
            password: hash_password(&password, &salt),
            created: Utc::now(),
        };

        self.user_repository.create(user).await
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        // recieve the user from the database given a user_id.
        self.user_repository.from_uuid(user_id).await
    }
}

/// Hashes a password with bcrypt together with a salt.
pub fn hash_password(password: &str, salt: &str) -> String {
    // Generate a hashed password
    hash(format!("{}{}", password, salt), bcrypt::DEFAULT_COST).expect("Failed to hash password")
}
