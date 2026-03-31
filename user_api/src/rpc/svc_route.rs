use crate::rpc::user_info::UserInfoProviderImpl;
use crate::rpc::user_security::UserSecurityProviderImpl;
use crate::state::user_state::UserState;
use fw_rpc::tonic_srv::tracer::{FwTraceRouter, FwTraceServer, FwTraceTimeoutRouter, FwTraceTimeoutServer};
use proto_bin::user_api::user_info_provider_server::UserInfoProviderServer;
use proto_bin::user_api::user_security_provider_server::UserSecurityProviderServer;

pub fn configure_svc_route(us: UserState, srv: &mut FwTraceTimeoutServer) -> FwTraceTimeoutRouter {
    srv.add_service(UserInfoProviderServer::new(UserInfoProviderImpl::new(us)))
        .add_service(UserSecurityProviderServer::new(
            UserSecurityProviderImpl::default(),
        ))
}
