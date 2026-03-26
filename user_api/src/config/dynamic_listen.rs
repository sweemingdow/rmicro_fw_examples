use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DynamicConfig {
    pub name: String,
}

pub struct DynamicConfigUpdater {}

impl ConfigChangeListener for DynamicConfigUpdater {
    fn notify(&self, config_resp: ConfigResponse) {
        tracing::info!("dynamic config changed, content\n{}", config_resp.content());
    }
}
