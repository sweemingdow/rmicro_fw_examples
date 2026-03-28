use crate::state::order_agg_state::OrderAggState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Extension, Router, routing};
use fw_adapter::web_bridge::{RespResult, WebResult};
use fw_base::context::web::WebContext;
use fw_base::from_scope;
use fw_base::my_utils::dy_trace;

pub fn router() -> Router<OrderAggState> {
    Router::new().route("/list", routing::get(order_list))
}

async fn order_list(State(s): State<OrderAggState>) -> WebResult<impl IntoResponse> {
    let ctx = from_scope()?;

    let _span = dy_trace::trace_with_action("order_list");

    tracing::debug!("order list handle start");

    let resp = s.order_agg_svc.order_list(ctx.uid_with_check()?).await?;

    Ok(RespResult::ok(resp))
}
