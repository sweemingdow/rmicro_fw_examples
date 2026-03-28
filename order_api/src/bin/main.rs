use axum::routing;
use fw_boot::App;
use fw_rpc::tonic_srv::chain_ext::BootChainRpcExt;
use fw_web::axum_srv::chain_ext::BootChainWebExt;

use fw_error::FwResult;
use order_api::rpc::srv_route;
use std::sync::Arc;
use fw_boot::ext::SimpleStaticConfig;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    let (rs, static_cfg) = app.clone().prepare::<SimpleStaticConfig>().await?;

    // let rs_clone = rs.clone();
    app.run_with(
        rs.clone(),
        |chain| async move {
            chain.add_rpc_server("OrderApiRpc", rs.clone(), |srv| {
                srv_route::configure_svc_route(srv)
            })
        },
        || async move {},
    )
    .await
}
