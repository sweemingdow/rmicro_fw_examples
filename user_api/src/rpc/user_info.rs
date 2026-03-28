use crate::state::user_state::UserState;
use proto_bin::user_api::user_info_provider_server::UserInfoProvider;
use proto_bin::user_api::{UserInfoReq, UserInfoResp};
use tonic;

pub struct UserInfoProviderImpl {
    user_state: UserState,
}

impl UserInfoProviderImpl {
    pub fn new(user_state: UserState) -> Self {
        Self { user_state }
    }
}

#[tonic::async_trait]
impl UserInfoProvider for UserInfoProviderImpl {
    async fn user_info(
        &self,
        request: tonic::Request<UserInfoReq>,
    ) -> Result<tonic::Response<UserInfoResp>, tonic::Status> {
        tracing::info!("user info, metadata:{:#?}", request.metadata());

        let req = request.into_inner();

        tracing::debug!(uid = %req.uid, "user info be called");

        let user = self.user_state.user_info_svc.user_info(&req.uid).await;

        Ok(tonic::Response::new(UserInfoResp {
            uid: req.uid,
            nickname: "fs".to_string(),
            avatar: "wsdg.jpg".to_string(),
            email: "wsdg@gmail.com".to_string(),
            phone: "11281556".to_string(),
        }))
    }
}
