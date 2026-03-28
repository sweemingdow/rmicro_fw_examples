use crate::state::user_state::UserState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Router, routing};
use fw_adapter::web_bridge::{RespResult, WebResult};
use fw_base::context::web::WebContext;
use fw_base::my_utils::dy_trace;

pub fn router() -> Router<UserState> {
    Router::new().route("/simple_info", routing::get(simple_info))
}

async fn simple_info(
    State(s): State<UserState>,
    Extension(ctx): Extension<WebContext>,
) -> WebResult<impl IntoResponse> {
    let uid = ctx.uid_with_check()?;

    let _ = dy_trace::trace_with_action("pull_simple_info");

    tracing::info!("pull simple info in handler, uid={}", uid);

    let resp = s.user_info_svc.user_info(&uid).await?;

    Ok(RespResult::ok(resp))
}
