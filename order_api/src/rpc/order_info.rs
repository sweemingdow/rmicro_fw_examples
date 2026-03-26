use tonic;
use proto_bin::order_api::order_info_provider_server::OrderInfoProvider;
use proto_bin::order_api::{OrderListReq, OrderListResp};

#[derive(Default)]
pub struct OrderInfoProviderImpl{

}

#[tonic::async_trait]
impl OrderInfoProvider for OrderInfoProviderImpl{
    async fn order_list(&self, request: tonic::Request<OrderListReq>) -> Result<tonic::Response<OrderListResp>, tonic::Status> {
        todo!()
    }
}