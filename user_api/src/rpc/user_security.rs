use tonic;
use proto_bin::user_api::user_security_provider_server::UserSecurityProvider;
use proto_bin::user_api::{VerifyTradePwdReq, VerifyTradePwdResp};

#[derive(Default)]
pub struct UserSecurityProviderImpl {
    
    
}

#[tonic::async_trait]
impl UserSecurityProvider for UserSecurityProviderImpl {
    async fn verify_trade_pwd(&self, request: tonic::Request<VerifyTradePwdReq>) -> Result<tonic::Response<VerifyTradePwdResp>, tonic::Status> {
        todo!()
    }
}