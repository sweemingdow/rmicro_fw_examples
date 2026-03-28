use crate::service::order_agg_svc::{OrderAggService, OrderAggServiceImpl};
use fw_rpc::tonic_srv::chan_factory::RpcChanFactory;
use std::sync::Arc;
use crate::rpc::holder::AggApiRpcClientHolder;

#[derive(Clone)]
pub struct OrderAggState {
    pub order_agg_svc: Arc<dyn OrderAggService + Send + Sync>,
}

impl OrderAggState {
    pub fn new(rpc_client_holder: Arc<AggApiRpcClientHolder>) -> Self {
        Self {
            order_agg_svc: OrderAggServiceImpl::new(rpc_client_holder),
        }
    }
}
