use crate::config::static_config::StaticConfig;
use crate::rpc::holder::AggApiRpcClientHolder;
use crate::state::order_agg_state::OrderAggState;
use fw_boot::App;
use fw_boot::state::RunState;
use fw_error::FwResult;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    order_agg_state: OrderAggState,
}

impl AppState {
    pub async fn init(app: Arc<App>) -> FwResult<(Self, Arc<RunState>)> {
        let (rs, static_cfg) = app.clone().prepare::<StaticConfig>().await?;

        let rpc_client_holder =
            AggApiRpcClientHolder::new(app.get_cfg(), rs.clone(), &static_cfg).await?;

        Ok((
            Self {
                order_agg_state: OrderAggState::new(rpc_client_holder),
            },
            rs,
        ))
    }

    pub fn order_agg_state(&self) -> OrderAggState {
        self.order_agg_state.clone()
    }
}
