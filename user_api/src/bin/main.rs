use fw_boot::App;
use fw_rpc::tonic_srv::chain_ext::BootChainRpcExt;
use fw_web::axum_srv::chain_ext::BootChainWebExt;

use fw_adapter::cfg_bridge::MysqlConfigWrapper;
use fw_error::FwResult;
use fw_sqlx::mysql::client;
use proto_bin::user_api::user_info_provider_server::UserInfoProviderServer;
use proto_bin::user_api::user_security_provider_server::UserSecurityProviderServer;
use std::sync::Arc;
use user_api::config::dynamic_listen::{DynamicConfig, DynamicConfigUpdater};
use user_api::config::static_config::StaticConfig;
use user_api::route;
use user_api::rpc::user_info::UserInfoProviderImpl;
use user_api::rpc::user_security::UserSecurityProviderImpl;
use user_api::state::app_state::AppState;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    let updater = Arc::new(DynamicConfigUpdater {});
    let (rs, static_cfg, dynamic_cfg) = app
        .clone()
        .prepare_with_dynamic::<StaticConfig, DynamicConfig, DynamicConfigUpdater>(updater)
        .await?;

    let sql_pool = client::init_mysql(MysqlConfigWrapper::try_into_options(
        static_cfg.comm_static_cfg.mysql_cfg,
    )?)
    .await?;

    let sql_pool_clone = sql_pool.clone();
    let app_state = AppState::new(sql_pool_clone);

    let rs_clone = rs.clone();
    app.run_with(
        rs.clone(),
        |chain| async move {
            chain
                .add_web_server("UserApiWeb", rs_clone, move |router| async move {
                    route::configure_user_model(router, app_state.user_state())
                })
                .add_rpc_server("UserApiRpc", rs.clone(), |srv| {
                    srv.add_service(UserInfoProviderServer::new(UserInfoProviderImpl::default()))
                        .add_service(UserSecurityProviderServer::new(
                            UserSecurityProviderImpl::default(),
                        ))
                })
        },
        || async move { sql_pool.close().await },
    )
    .await
}
