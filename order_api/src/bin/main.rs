use axum::routing;
use fw_boot::App;
use fw_rpc::tonic_srv::chain_ext::BootChainRpcExt;
use fw_web::axum_srv::chain_ext::BootChainWebExt;

use fw_adapter::cfg_bridge::MysqlConfigWrapper;
use fw_error::FwResult;
use fw_sqlx::mysql::client;
use std::sync::Arc;
use order_api::config::static_config::StaticConfig;
use order_api::rpc::order_info::OrderInfoProviderImpl;
use proto_bin::order_api::order_info_provider_server::OrderInfoProviderServer;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    let (rs, static_cfg) = app
        .clone()
        .prepare::<StaticConfig>()
        .await?;

    let sql_pool = client::init_mysql(MysqlConfigWrapper::try_into_options(
        static_cfg.comm_static_cfg.mysql_cfg,
    )?)
        .await?;

    let rs_clone = rs.clone();
    app.run_with(
        rs.clone(),
        |chain| async move {
            chain
                .add_rpc_server("OrderApiRpc", rs.clone(), |srv| {
                    srv.add_service(OrderInfoProviderServer::new(OrderInfoProviderImpl::default()))
                })
        },
        || async move {
        },
    )
        .await
}
