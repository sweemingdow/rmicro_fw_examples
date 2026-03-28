use fw_base::configuration::static_config::{CommStaticConfig, GwDispatchConfig, RpcChannelConfig};
use fw_boot::ext::RunConfigExt;
use fw_error::{FwError, FwResult};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StaticConfig {
    pub comm_static_cfg: CommStaticConfig,
}

impl RunConfigExt for StaticConfig {
    fn get_gw_dispatch_cfg(&self) -> &GwDispatchConfig {
        &self.comm_static_cfg.gw_dispatch_cfg
    }
}

impl StaticConfig {
    pub fn get_rpc_config(&self) -> FwResult<&HashMap<String, RpcChannelConfig>> {
        self.comm_static_cfg
            .rpc_call_cfg
            .as_ref()
            .ok_or_else(|| FwError::ConfigError("rpc callee", "config missing".to_string()))
    }
}
