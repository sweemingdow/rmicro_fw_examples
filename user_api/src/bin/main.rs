use fw_boot::App;
use fw_rpc::tonic_srv::chain_ext::BootChainRpcTimeoutExt;
use fw_rpc::tonic_srv::chain_ext::BootChainRpcTraceExt;
use fw_web::axum_srv::chain_ext::BootChainWebExt;

use chrono::Duration;
use fw_adapter::cfg_bridge::MysqlConfigWrapper;
use fw_error::FwResult;
use fw_sqlx::mysql::client;
use std::sync::Arc;
use std::time;
use user_api::config::dynamic_listen::{DynamicConfig, DynamicConfigUpdater};
use user_api::config::static_config::StaticConfig;
use user_api::route;
use user_api::rpc::svc_route;
use user_api::state::app_state::AppState;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

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
    let app_state = AppState::new(sql_pool_clone);

    let rs_clone = rs.clone();
    let us_clone = app_state.user_state();

    app.run_with(
        rs.clone(),
        move |chain| async move {
            chain
                .add_web_server("UserApiWeb", rs_clone, move |router| async move {
                    route::configure_user_model(router, app_state.user_state())
                })
                .add_rpc_server_with_global_timeout(
                    "UserApiRpc",
                    rs.clone(),
                    rpc_callee_timeout,
                    |srv| svc_route::configure_svc_route(us_clone, srv),
                )
        },
        || async move { sql_pool.close().await },
    )
    .await
}
