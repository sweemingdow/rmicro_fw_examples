use fw_base::configuration::static_config::{
    CommStaticConfig, GwDispatchConfig, RpcCallConfig, RpcChannelConfig,
};
use fw_boot::ext::RunConfigExt;
use fw_gateway::config::GatewayServerConfig;
use fw_gateway::ext::config_ext::GatewayRunConfigExt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StaticConfig {
    pub comm_static_cfg: CommStaticConfig,

    pub gateway_server_cfg: GatewayServerConfig,
}

impl RunConfigExt for StaticConfig {
    fn get_gw_dispatch_cfg(&self) -> &GwDispatchConfig {
        &self.comm_static_cfg.gw_dispatch_cfg
    }
}

impl GatewayRunConfigExt for StaticConfig {
    fn get_gateway_server_config(&self) -> &GatewayServerConfig {
        &self.gateway_server_cfg
    }
}
