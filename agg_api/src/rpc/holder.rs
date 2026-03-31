use crate::config::static_config::StaticConfig;
use fw_boot::config::Config;
use fw_boot::state::RunState;
use fw_error::FwResult;
use fw_rpc::tonic_srv::chan_factory::RpcChanFactory;
use fw_rpc::tonic_srv::provider::RpcProviderHolder;
use proto_bin::order_api::order_info_provider_client::OrderInfoProviderClient;
use proto_bin::user_api::user_info_provider_client::UserInfoProviderClient;
use std::sync::Arc;
use tokio::sync;
use tonic::transport;

pub struct AggApiRpcClientHolder {
    _holder: RpcProviderHolder,

    user_api_client_cell: sync::OnceCell<UserInfoProviderClient<transport::Channel>>,
    order_api_client_cell: sync::OnceCell<OrderInfoProviderClient<transport::Channel>>,
}

impl AggApiRpcClientHolder {
    pub async fn new(
        cfg: Arc<Config>,
        rs: Arc<RunState>,
        static_cfg: &StaticConfig,
    ) -> FwResult<Arc<Self>> {
        let rpc_chan_factory = RpcChanFactory::with_preload_then_log(
            &cfg.nacos_center_cfg.registry.group_name,
            rs.nacos_proxy(),
            static_cfg.comm_static_cfg.get_rpc_srv_ele()?,
        )
        .await?;

        Ok(Arc::new(Self {
            _holder: RpcProviderHolder::new(rpc_chan_factory),
            user_api_client_cell: Default::default(),
            order_api_client_cell: Default::default(),
        }))
    }

    pub async fn get_user_api_client(
        &self,
    ) -> FwResult<UserInfoProviderClient<transport::Channel>> {
        self._holder
            .get_or_init_client(
                None,
                &self.user_api_client_cell,
                "user_api",
                UserInfoProviderClient::new,
            )
            .await
    }

    pub async fn get_order_api_client(
        &self,
    ) -> FwResult<OrderInfoProviderClient<transport::Channel>> {
        self._holder
            .get_or_init_client(
                None,
                &self.order_api_client_cell,
                "order_api",
                OrderInfoProviderClient::new,
            )
            .await
    }
}
