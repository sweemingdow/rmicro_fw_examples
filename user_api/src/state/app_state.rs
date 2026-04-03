use crate::config::dynamic_listen::{DynamicConfig, DynamicConfigUpdater};
use crate::config::static_config::StaticConfig;
use crate::state::user_state::UserState;
use axum::extract::FromRef;
use fw_adapter::cfg_bridge::MysqlConfigWrapper;
use fw_boot::App;
use fw_boot::state::RunState;
use fw_error::FwResult;
use fw_sqlx::mysql::client;
use sqlx::{MySql, Pool};
use std::sync::Arc;
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    user_state: UserState,
}

impl AppState {
    pub async fn init(app: Arc<App>) -> FwResult<(Self, Arc<RunState>, Duration, Pool<MySql>)> {
        let updater = Arc::new(DynamicConfigUpdater {});
        let (rs, static_cfg, dynamic_cfg) = app
            .clone()
            .prepare_with_dynamic::<StaticConfig, DynamicConfig, DynamicConfigUpdater>(updater)
            .await?;

        let rpc_callee_timeout = static_cfg.comm_static_cfg.get_rpc_global_timeout()?;

        let sql_pool = client::init_mysql(MysqlConfigWrapper::try_into_options(
            static_cfg.comm_static_cfg.mysql_cfg,
        )?)
        .await?;

        let sql_pool_clone = sql_pool.clone();

        Ok((
            Self {
                user_state: UserState::init(sql_pool_clone),
            },
            rs,
            rpc_callee_timeout,
            sql_pool,
        ))
    }

    #[inline]
    pub fn user_state(&self) -> UserState {
        self.user_state.clone()
    }
}
