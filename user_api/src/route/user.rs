use crate::state::user_state::UserState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Json, Router, routing};
use fw_adapter::web_bridge::{RespResult, WebResult};

pub fn router() -> Router<UserState> {
    Router::new().route("/simple_info/:uid", routing::get(simple_info))
}

async fn simple_info(
    State(s): State<UserState>,
    Path(uid): Path<String>,
) -> WebResult<impl IntoResponse> {
    let resp = s.user_info_svc.user_info(&uid).await?;

    Ok(RespResult::ok(resp))
}
