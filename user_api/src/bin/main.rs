use fw_boot::App;
use fw_rpc::tonic_srv::chain_ext::BootChainRpcTimeoutExt;
use fw_web::axum_srv::chain_ext::BootChainWebExt;

use fw_error::FwResult;
use std::sync::Arc;
use user_api::route;
use user_api::rpc::svc_route;
use user_api::state::app_state::AppState;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    let (app_state, rs, rpc_callee_timeout, sql_pool) = AppState::init(app.clone()).await?;

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
