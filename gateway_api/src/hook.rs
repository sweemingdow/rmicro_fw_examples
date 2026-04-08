use crate::context::{GatewayContext, PassContext};
use crate::rpc::GatewayApiRpcClientHolder;
use anyhow::Context;
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose};
use fw_base::constants::web_const::{AUTH_INFO_KEY, GW_DISPATCH_KEY};
use fw_base::my_utils::rand;
use fw_base::pass::gw_pass::{AuthInfoPassStrategy, AuthInfoPassStrategyEnum}; // 改为引入枚举
use fw_base::utils::dy_trace;
use fw_base::{WebPassContext, fmt_json_as_u8, get_gw_dispatch_val};
use fw_crypto::aes::AesKeyDisplayType;
use fw_crypto::aes::gcm::AesGcm;
use fw_error::{FwError, FwResult};
use fw_gateway::PingoraResult;
use fw_gateway::ext::{GatewayHookExt, result_ext};
use fw_rpc::tonic_srv::caller::RpcCaller;
use fw_rpc::tonic_srv::tracer::RpcTraceUnit;
use http::{HeaderValue, StatusCode};
use moka::future::Cache;
use pingora::Error;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::Session;
use prost::bytes::Bytes;
use proto_bin::auth_api::{ParseTokenReq, ParseTokenResp};
use std::borrow::Cow;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tonic::{Response, Status};

// 移除泛型参数，直接使用枚举
pub struct HttpRequestHook {
    pass_strategy: AuthInfoPassStrategyEnum,
    rpc_client_holder: Arc<GatewayApiRpcClientHolder>,
    aes_gcm: AesGcm,
    auth_cache: Cache<String, ParseTokenResp>,
}

#[async_trait]
impl GatewayHookExt for HttpRequestHook {
    type CTX = GatewayContext;

    fn new_ctx(&self) -> Self::CTX {
        GatewayContext {
            req_id: rand::gen_uuid().to_string(),
            auth_resp: Cow::Borrowed(&None),
            client_type: 0,
            client_version: Cow::Borrowed(""),
            start: Instant::now(),
        }
    }

    async fn on_request(&self, session: &mut Session, ctx: &mut Self::CTX) -> PingoraResult<bool> {
        tracing::debug!("on_request");

        let cli_type = session
            .get_header("client-type")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u8>().ok())
            .ok_or_else(|| {
                Error::create(
                    pingora::ErrorType::HTTPStatus(StatusCode::BAD_REQUEST.as_u16()),
                    pingora::ErrorSource::Downstream,
                    Some("no client type found".into()),
                    None,
                )
            })?;

        let cli_version = session
            .get_header("client-version")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                Error::create(
                    pingora::ErrorType::HTTPStatus(StatusCode::BAD_REQUEST.as_u16()),
                    pingora::ErrorSource::Downstream,
                    Some("no client version found".into()),
                    None,
                )
            })?;

        let token = session
            .get_header("auth-token")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                Error::create(
                    pingora::ErrorType::HTTPStatus(StatusCode::BAD_REQUEST.as_u16()),
                    pingora::ErrorSource::Downstream,
                    Some("no auth token found".into()),
                    None,
                )
            })?;

        let nonce = session
            .get_header("nonce")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                Error::create(
                    pingora::ErrorType::HTTPStatus(StatusCode::BAD_REQUEST.as_u16()),
                    pingora::ErrorSource::Downstream,
                    Some("no nonce found".into()),
                    None,
                )
            })?;

        let cache_key = token.to_owned();

        let resp: ParseTokenResp = self
            .auth_cache
            .try_get_with(cache_key.clone(), async {
                // 未命中缓存时才调用 RPC
                tracing::info!(token = %token, "Cache miss, calling auth RPC");

                let mut auth_api_client = result_ext::as_pingora_result(
                    self.rpc_client_holder.get_auth_api_client().await,
                    |_| {
                        (
                            pingora::ErrorType::InternalError,
                            pingora::ErrorSource::Internal,
                        )
                    },
                )?;

                let rpc_result = RpcCaller::call_with_trace(
                    "do_auth",
                    Some(Duration::from_secs(1)),
                    Some(RpcTraceUnit::with(&ctx.req_id, "")),
                    ParseTokenReq {
                        token: token.to_owned(),
                        nonce: nonce.to_string(),
                    },
                    |tonic_req| async { auth_api_client.parse_token(tonic_req).await },
                )
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "Auth RPC call failed");
                    Error::create(
                        pingora::ErrorType::InternalError,
                        pingora::ErrorSource::Internal,
                        Some("auth service unavailable".into()),
                        None,
                    )
                })?;

                let resp = rpc_result.into_inner();
                tracing::debug!(uid = %resp.uid, "Auth RPC success");

                Ok::<ParseTokenResp, pingora::BError>(resp)
            })
            .await
            .map_err(|e| {
                // try_get_with 返回的错误需要转为 PingoraError
                tracing::error!(error = %e, "Failed to get auth info from cache or rpc");
                Error::create(
                    pingora::ErrorType::InternalError,
                    pingora::ErrorSource::Internal,
                    Some("auth service unavailable".into()),
                    None,
                )
            })?;

        tracing::debug!(
            "auth completed, token={}, nonce={}, resp={:#?}",
            token,
            nonce,
            resp
        );

        ctx.client_type = cli_type;
        ctx.client_version = Cow::Owned(cli_version.to_string());
        ctx.auth_resp = Cow::Owned(Some(resp));

        Ok(false)
    }

    async fn on_upstream_request(
        &self,
        _: &mut Session,
        rh: &mut RequestHeader,
        ctx: &mut Self::CTX,
    ) -> PingoraResult<()> {
        tracing::debug!("on_upstream_request");

        self.write_info_to_upstream_with_header(rh, ctx)?;

        Ok(())
    }

    async fn on_response(
        &self,
        _: &mut Session,
        rh: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> PingoraResult<()> {
        tracing::debug!("on_response");

        rh.insert_header("x-req-id", ctx.req_id.to_string())?;

        Ok(())
    }

    fn on_response_body(
        &self,
        _: &mut Session,
        _: &mut Option<Bytes>,
        _: bool,
        _ctx: &mut Self::CTX,
    ) -> PingoraResult<Option<Duration>> {
        tracing::debug!("on_response_body");
        Ok(None)
    }

    async fn on_logging(&self, _: &mut Session, _: Option<&Error>, _ctx: &mut Self::CTX) {
        tracing::debug!("on_logging");
    }
}

impl HttpRequestHook {
    // new 方法直接接受枚举
    pub fn new(
        strategy: AuthInfoPassStrategyEnum,
        rpc_client_holder: Arc<GatewayApiRpcClientHolder>,
    ) -> Self {
        let aes_gcm = AesGcm::from_str(
            "VpQYmYyoY/HQ4XAP1fKvVKmc4kfiOAkVjKF7VHebgeA=",
            AesKeyDisplayType::B64,
        )
        .unwrap();

        let auth_cache = Cache::builder()
            .max_capacity(100)
            .time_to_live(Duration::from_secs(900))
            .time_to_idle(Duration::from_secs(900))
            .build();

        Self {
            pass_strategy: strategy,
            rpc_client_holder,
            aes_gcm,
            auth_cache,
        }
    }

    fn write_info_to_upstream_with_header(
        &self,
        rh: &mut RequestHeader,
        ctx: &GatewayContext,
    ) -> PingoraResult<()> {
        Self::insert_dispatch_val(rh)?;

        self.insert_auth_info(rh, self.build_authed_info_context(ctx))?;

        Ok(())
    }

    fn build_authed_info_context(&self, ctx: &GatewayContext) -> WebPassContext {
        let uid = match ctx.auth_resp.as_ref() {
            None => "",
            Some(resp) => &resp.uid,
        };

        WebPassContext {
            req_id: ctx.req_id.to_string(),
            uid: Some(uid.to_string()),
            client_type: ctx.client_type,
            client_version: ctx.client_version.as_ref().to_string(),
            in_white: false,
            in_callback: false,
            in_open: false,
        }
    }

    fn insert_dispatch_val(rh: &mut RequestHeader) -> PingoraResult<()> {
        let dis_val = get_gw_dispatch_val().map_err(|e| {
            Error::create(
                pingora::ErrorType::InternalError,
                pingora::ErrorSource::Internal,
                Some(e.to_string().into()),
                None,
            )
        })?;

        Ok(rh.insert_header(GW_DISPATCH_KEY, dis_val)?)
    }

    fn insert_auth_info(&self, rh: &mut RequestHeader, ctx: WebPassContext) -> PingoraResult<()> {
        let encoded = self.pass_strategy.encode(&ctx).map_err(|e| {
            Error::create(
                pingora::ErrorType::HTTPStatus(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
                pingora::ErrorSource::Internal,
                Some(format!("auth info encode failed, err={}", e).into()),
                None,
            )
        })?;

        Ok(rh.insert_header(AUTH_INFO_KEY, &encoded[..])?)
    }
}

/*
/*let mut auth_api_client = result_ext::as_pingora_result(
            self.rpc_client_holder.get_auth_api_client().await,
            |_| {
                (
                    pingora::ErrorType::InternalError,
                    pingora::ErrorSource::Internal,
                )
            },
        )?;

        let resp = result_ext::as_pingora_result(
            RpcCaller::call_with_trace(
                "do_auth",
                Some(Duration::from_secs(1)),
                Some(RpcTraceUnit::with(&ctx.req_id, "")),
                ParseTokenReq {
                    token: token.to_owned(),
                    nonce: nonce.to_string(),
                },
                |tonic_req| async { auth_api_client.parse_token(tonic_req).await },
            )
            .await,
            |_| {
                (
                    pingora::ErrorType::InternalError,
                    pingora::ErrorSource::Internal,
                )
            },
        )?
        .into_inner();*/
*/
