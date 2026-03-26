use fw_regdis::nacos::configuration::CommStaticConfig;
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StaticConfig {
    pub comm_static_cfg: CommStaticConfig,
}
