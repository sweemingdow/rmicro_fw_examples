use axum::routing;
use fw_boot::App;
use fw_rpc::tonic_srv::chain_ext::BootChainRpcTimeoutExt;
use fw_web::axum_srv::chain_ext::BootChainWebExt;

use auth_api::config::static_config::StaticConfig;
use auth_api::rpc::route;
use fw_boot::ext::RunConfigExt;
use fw_error::FwResult;
use std::sync::Arc;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    let (rs, static_cfg) = app.clone().prepare::<StaticConfig>().await?;

    app.run_with(
        rs.clone(),
        |chain| async move {
            chain.add_rpc_server_with_global_timeout(
                "AuthApiRpc",
                rs.clone(),
                static_cfg.comm_static_cfg.get_rpc_global_timeout().unwrap(),
                move |srv| route::configure_rpc_route(srv, &static_cfg),
            )
        },
        || async move {},
    )
    .await
}
