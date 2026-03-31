use fw_boot::App;
use fw_web::axum_srv::chain_ext::BootChainWebExt;
use agg_api::route;
use agg_api::state::app_state::AppState;
use fw_error::FwResult;
use std::sync::Arc;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    let (app_state, rs) = AppState::init(app.clone()).await?;

    let rs_clone = rs.clone();
    app.run_with(
        rs.clone(),
        |chain| async move {
            chain.add_web_server("AggApiWeb", rs_clone, move |router| async move {
                route::configure_order_model(router, app_state.order_agg_state())
            })
        },
        || async move {},
    )
    .await
}
