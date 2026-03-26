use fw_boot::App;
use fw_error::FwResult;
use fw_web::axum_srv::chain_ext::BootChainWebExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> FwResult<()> {
    let app = Arc::new(App::new()?);

    app.run_with(
        |chain, rs| async move {
            chain.add_web_server("AggApiWeb", rs.clone(), |router| async { router })
        },
        |_rs| async {},
    )
    .await
}
