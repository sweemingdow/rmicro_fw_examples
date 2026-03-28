use crate::rpc::holder::AggApiRpcClientHolder;
use crate::state::order_agg_state::OrderAggState;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    order_agg_state: OrderAggState,
}

impl AppState {
    pub fn init(rpc_client_holder: Arc<AggApiRpcClientHolder>) -> Self {
        Self {
            order_agg_state: OrderAggState::new(rpc_client_holder),
        }
    }

    pub fn order_agg_state(&self) -> OrderAggState {
        self.order_agg_state.clone()
    }
}
