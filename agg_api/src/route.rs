pub mod order_agg;

use crate::state::order_agg_state::OrderAggState;
use axum::Router;

pub fn configure_order_model(router: Router, order_agg_state: OrderAggState) -> Router {
    router.nest("/order", order_agg::router().with_state(order_agg_state))
}
