use fw_base::configuration::static_config::{CommStaticConfig, GwDispatchConfig};
use fw_boot::ext::RunConfigExt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StaticConfig {
    pub comm_static_cfg: CommStaticConfig,
    pub auth_cfg: AuthConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AuthConfig {
    pub decrypt_key: String,
}

impl RunConfigExt for StaticConfig {
    fn get_gw_dispatch_cfg(&self) -> &GwDispatchConfig {
        &self.comm_static_cfg.gw_dispatch_cfg
    }
}
