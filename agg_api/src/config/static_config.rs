use fw_base::configuration::static_config::{
    CommStaticConfig, GwDispatchConfig, RpcCallConfig, RpcChannelConfig,
};
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
