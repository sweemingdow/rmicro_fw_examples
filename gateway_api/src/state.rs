use crate::config::StaticConfig;
use crate::rpc::GatewayApiRpcClientHolder;
use fw_boot::state::RunState;
use fw_error::FwResult;
use std::sync::Arc;

pub struct AppState {
    rpc_client_holder: Arc<GatewayApiRpcClientHolder>,
}

impl AppState {
    pub async fn init(rs: Arc<RunState>, static_cfg: &StaticConfig) -> FwResult<Self> {
        let holder = GatewayApiRpcClientHolder::new(rs, static_cfg).await?;
        Ok(Self {
            rpc_client_holder: holder,
        })
    }

    pub fn rpc_client_holder(&self) -> Arc<GatewayApiRpcClientHolder> {
        self.rpc_client_holder.clone()
    }
}
