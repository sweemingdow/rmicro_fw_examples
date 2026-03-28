use crate::rpc::holder::AggApiRpcClientHolder;
use anyhow::Context;
use async_trait::async_trait;
use fw_error::AnyResult;
use fw_rpc::tonic_srv::caller::RpcCaller;
use proto_bin::order_api::OrderListReq;
use proto_bin::user_api::UserInfoReq;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use tonic::{Response, Status};

#[async_trait]
pub trait OrderAggService {
    async fn order_list(&self, uid: &str) -> AnyResult<OrderListResp>;
}

pub struct OrderAggServiceImpl {
    rpc_client_holder: Arc<AggApiRpcClientHolder>,
}

impl OrderAggServiceImpl {
    pub fn new(
        rpc_client_holder: Arc<AggApiRpcClientHolder>,
    ) -> Arc<dyn OrderAggService + Send + Sync> {
        Arc::new(Self { rpc_client_holder })
    }
}

#[async_trait]
impl OrderAggService for OrderAggServiceImpl {
    async fn order_list(&self, uid: &str) -> AnyResult<OrderListResp> {
        let uid_owned = uid.to_string();

        tracing::info!("handle order list in order agg service");

        let user_task = async {
            let mut user_api_client = self.rpc_client_holder.get_user_api_client().await?;
            let user_req = UserInfoReq {
                uid: uid_owned.clone(),
            };

            RpcCaller::call_with_trace("order_list", user_req, move |request| async move {
                user_api_client.user_info(request).await
            })
            .await
            .with_context(|| format!("fetch user info failed, uid={}", uid_owned))
            .map(|resp| resp.into_inner())
        };

        let order_task = async {
            let mut order_api_client = self.rpc_client_holder.get_order_api_client().await?;
            let order_req = OrderListReq {
                uid: uid_owned.clone(),
            };

            RpcCaller::call_with_trace("order_list", order_req, move |req| async move {
                order_api_client.order_list(req).await
            })
            .await
            .with_context(|| format!("fetch order list failed, uid={}", uid_owned))
            .map(|resp| resp.into_inner())
        };

        let (user_resp, order_resp) = tokio::try_join!(user_task, order_task)?;

        let items: Vec<OrderListItem> = order_resp
            .items
            .into_iter()
            .map(|item| OrderListItem {
                order_id: item.order_id,
                price: item.price,
                sku_id: item.sku_id,
                spu_id: item.spu_id,
                goods_title: item.goods_title,
            })
            .collect();

        Ok(OrderListResp {
            user_info: Some(UserInfoResp {
                uid: user_resp.uid,
                avatar: user_resp.avatar,
                nicknamer: user_resp.nickname,
            }),
            order_items: items,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct OrderListResp {
    pub user_info: Option<UserInfoResp>,
    pub order_items: Vec<OrderListItem>,
}

#[derive(Debug, Serialize)]
pub struct UserInfoResp {
    pub uid: String,
    pub avatar: String,
    pub nicknamer: String,
}

#[derive(Debug, Serialize)]
pub struct OrderListItem {
    pub order_id: String,
    pub price: String,
    pub sku_id: String,
    pub spu_id: String,
    pub goods_title: String,
}
