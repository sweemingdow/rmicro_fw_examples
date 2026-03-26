use crate::route::order::OrderState;
use axum::{Router};

pub mod order;

pub fn configure_order_model(router: Router) -> Router {
    router.nest("/order", order::router().with_state(OrderState {}))
}
