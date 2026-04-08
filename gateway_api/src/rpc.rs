use crate::config::StaticConfig;
use fw_boot::config::Config;
use fw_boot::state::RunState;
use fw_error::FwResult;
use fw_rpc::tonic_srv::chan_factory::RpcChanFactory;
use fw_rpc::tonic_srv::provider::RpcProviderHolder;
use proto_bin::auth_api::auth_info_provider_client::AuthInfoProviderClient;
use std::sync::Arc;
use tokio::sync;
use tonic::transport;

pub struct GatewayApiRpcClientHolder {
    _holder: RpcProviderHolder,

    auth_api_client_cell: sync::OnceCell<AuthInfoProviderClient<transport::Channel>>,
}

impl GatewayApiRpcClientHolder {
    pub async fn new(
        rs: Arc<RunState>,
        static_cfg: &StaticConfig,
    ) -> FwResult<Arc<Self>> {
        let rpc_chan_factory = RpcChanFactory::with_preload_then_log(
            &rs.cfg().nacos_center_cfg.registry.group_name,
            rs.nacos_proxy(),
            static_cfg.comm_static_cfg.get_rpc_srv_ele()?,
        )
        .await?;

        Ok(Arc::new(Self {
            _holder: RpcProviderHolder::new(rpc_chan_factory),
            auth_api_client_cell: Default::default(),
        }))
    }

    pub async fn get_auth_api_client(
        &self,
    ) -> FwResult<AuthInfoProviderClient<transport::Channel>> {
        self._holder
            .get_or_init_client(
                None,
                &self.auth_api_client_cell,
                "auth_api",
                AuthInfoProviderClient::new,
            )
            .await
    }
}
