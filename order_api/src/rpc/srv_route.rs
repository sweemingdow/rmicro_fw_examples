use crate::rpc::order_info::OrderInfoProviderImpl;
use proto_bin::order_api::order_info_provider_server::OrderInfoProviderServer;
use tonic::transport::{Server, server};

pub fn configure_svc_route(srv: &mut Server) -> server::Router {
    srv.add_service(OrderInfoProviderServer::new(
        OrderInfoProviderImpl::default(),
    ))
}
