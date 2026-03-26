use proto_bin::user_api::user_info_provider_server::UserInfoProvider;
use proto_bin::user_api::{UserInfoReq, UserInfoResp};
use tonic;

#[derive(Default)]
pub struct UserInfoProviderImpl {}

#[tonic::async_trait]
impl UserInfoProvider for UserInfoProviderImpl {
    async fn user_info(
        &self,
        request: tonic::Request<UserInfoReq>,
    ) -> Result<tonic::Response<UserInfoResp>, tonic::Status> {
        let req = request.into_inner();

        tracing::debug!(uid = %req.uid, "user info be called");

        Ok(tonic::Response::new(UserInfoResp {
            uid: req.uid,
            nickname: "wsdg".to_string(),
            avatar: "wsdg.jpg".to_string(),
            email: "wsdg@gmail.com".to_string(),
            phone: "11281556".to_string(),
        }))
    }
}
