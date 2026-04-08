use crate::config::static_config::StaticConfig;
use crate::rpc::auth_info::AuthInfoProviderImpl;
use fw_rpc::tonic_srv::tracer::{FwTraceTimeoutRouter, FwTraceTimeoutServer};
use proto_bin::auth_api::auth_info_provider_server::AuthInfoProviderServer;

pub fn configure_rpc_route(
    srv: &mut FwTraceTimeoutServer,
    static_cfg: &StaticConfig,
) -> FwTraceTimeoutRouter {
    srv.add_service(AuthInfoProviderServer::new(AuthInfoProviderImpl::new(
        &static_cfg.auth_cfg.decrypt_key,
    )))
}
