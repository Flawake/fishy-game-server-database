use crate::domain::User;
use rocket::async_trait;
use sqlx::types::Uuid;
use sqlx::PgPool;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> Result<(), sqlx::Error>;

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error>;

    async fn from_email(&self, email: String) -> Result<Option<User>, sqlx::Error>;

    // add more functions such as update or delete.
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (user_id, name, email, password, salt)
             VALUES ($1, $2, $3, $4, $5)",
            user.user_id,
            user.name,
            user.email,
            user.password,
            user.salt
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn from_uuid(&self, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, email, password, salt
             FROM users
             WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn from_email(&self, email: String) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT user_id, name, email, password, salt
             FROM users
             WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }
}
