use crate::rpc::user_info::UserInfoProviderImpl;
use crate::rpc::user_security::UserSecurityProviderImpl;
use crate::state::user_state::UserState;
use proto_bin::user_api::user_info_provider_server::UserInfoProviderServer;
use proto_bin::user_api::user_security_provider_server::UserSecurityProviderServer;
use tonic::transport::{Server, server};

pub fn configure_svc_route(srv: &mut Server, us: UserState) -> server::Router {
    srv.add_service(UserInfoProviderServer::new(UserInfoProviderImpl::new(us)))
        .add_service(UserSecurityProviderServer::new(
            UserSecurityProviderImpl::default(),
        ))
}
