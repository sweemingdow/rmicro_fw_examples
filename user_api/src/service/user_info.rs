use crate::repository::user_repository::UserRepository;
use anyhow::{Context, anyhow, bail};
use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;
use fw_error::{AnyResult, AppError};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSimpleInfoResp {
    pub uid: String,
    pub avatar: Option<String>,
    pub nickname: Option<String>,
}

#[async_trait]
pub trait UserInfoService {
    async fn user_info(&self, uid: &str) -> AnyResult<UserSimpleInfoResp>;
}

pub struct UserInfoServiceImpl {
    user_repo: Arc<dyn UserRepository + Send + Sync>,
}

impl UserInfoServiceImpl {
    pub fn new(
        user_repo: Arc<dyn UserRepository + Send + Sync>,
    ) -> Arc<dyn UserInfoService + Send + Sync> {
        Arc::new(Self { user_repo })
    }
}

#[async_trait]
impl UserInfoService for UserInfoServiceImpl {
    async fn user_info(&self, uid: &str) -> AnyResult<UserSimpleInfoResp> {
        tracing::info!("pull simple info in service");

        let user = self
            .user_repo
            .user_info(uid)
            .await?
            .ok_or_else(|| anyhow!(AppError::ApiError("user not found".to_string())))?;

        Ok(UserSimpleInfoResp {
            uid: user.uid,
            avatar: user.avatar,
            nickname: user.nickname,
        })
    }
}
