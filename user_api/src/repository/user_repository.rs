use crate::model::sql_mod::user::User;
use anyhow::Context;
use axum::async_trait;
use fw_error::{AnyResult, AppError};
use sqlx;
use std::sync::Arc;

#[async_trait]
pub trait UserRepository {
    async fn user_info(&self, uid: &str) -> AnyResult<Option<User>>;
}

pub struct UserRepositoryImpl {
    sql_pool: sqlx::Pool<sqlx::MySql>,
}

impl UserRepositoryImpl {
    pub fn new(sql_pool: sqlx::Pool<sqlx::MySql>) -> Arc<dyn UserRepository + Send + Sync> {
        Arc::new(Self { sql_pool })
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn user_info(&self, uid: &str) -> AnyResult<Option<User>> {
        sqlx::query_as::<_, User>("SELECT * FROM t_user WHERE uid = ?")
            .bind(uid)
            .fetch_optional(&self.sql_pool)
            .await
            .map_err(|e| AppError::SqlDbError(e.to_string()))
            .with_context(|| "qry user failed")
    }
}
