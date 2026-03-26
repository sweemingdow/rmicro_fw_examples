use axum::extract::State;
use axum::{Json, Router, routing};
use serde::{Deserialize, Serialize};

pub fn router() -> Router<OrderState> {
    Router::new().route("/submit", routing::post(submit_order))
}

#[derive(Clone)]
pub struct OrderState {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitReq {
    pub uid: String,
    pub spu_id: u64,
    pub sku_id: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResp {
    pub order_id: u64,
}

async fn submit_order(State(s): State<OrderState>, Json(req): Json<SubmitReq>) -> Json<SubmitResp> {
    panic!()
}
