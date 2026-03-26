use crate::repository::user_repository::UserRepositoryImpl;
use crate::service::user_info::{UserInfoService, UserInfoServiceImpl};
use sqlx::{MySql, Pool};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserState {
    pub user_info_svc: Arc<dyn UserInfoService + Send + Sync>,
}

impl UserState {
    pub fn init(sql_pool: Pool<MySql>) -> Self {
        let user_repo = UserRepositoryImpl::new(sql_pool.clone());
        let user_info_svc = UserInfoServiceImpl::new(user_repo);
        Self { user_info_svc }
    }
}

