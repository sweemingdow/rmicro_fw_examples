use fw_boot::App;
use fw_web::axum_srv::chain_ext::BootChainWebExt;

use agg_api::config::static_config::StaticConfig;
use agg_api::route;
use agg_api::rpc::holder::AggApiRpcClientHolder;
use agg_api::state::app_state::AppState;
use fw_error::FwResult;
use std::sync::Arc;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    let (rs, static_cfg) = app.clone().prepare::<StaticConfig>().await?;

    let rpc_client_holder =
        AggApiRpcClientHolder::new(app.get_cfg(), rs.clone(), &static_cfg).await?;

    let app_state = AppState::init(rpc_client_holder);

    let rs_clone = rs.clone();
    let order_agg_state = app_state.order_agg_state();
    app.run_with(
        rs.clone(),
        |chain| async move {
            chain.add_web_server("AggApiWeb", rs_clone, move |router| async move {
                route::configure_order_model(router, order_agg_state)
            })
        },
        || async move {},
    )
    .await
}
