use crate::context::{GatewayContext, PassContext};
use async_trait::async_trait;
use base64::{Engine, engine::general_purpose};
use fw_base::constants::web_const::{AUTH_INFO_KEY, GW_DISPATCH_KEY};
use fw_base::my_utils::rand;
use fw_base::pass::gw_pass::{AuthInfoPassStrategy, AuthInfoPassStrategyEnum};  // 改为引入枚举
use fw_base::{WebPassContext, fmt_json_as_u8, get_gw_dispatch_val};
use fw_gateway::PingoraResult;
use fw_gateway::ext::GatewayHookExt;
use http::{HeaderValue, StatusCode};
use pingora::Error;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::Session;
use prost::bytes::Bytes;
use std::borrow::Cow;
use std::time::{Duration, Instant};

// 移除泛型参数，直接使用枚举
pub struct HttpRequestHook {
    pass_strategy: AuthInfoPassStrategyEnum,
}

#[async_trait]
impl GatewayHookExt for HttpRequestHook {
    type CTX = GatewayContext;

    fn new_ctx(&self) -> Self::CTX {
        GatewayContext {
            req_id: rand::gen_uuid(),
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
                    Some("Bad Request".into()),
                    None,
                )
            })?;

        ctx.client_type = cli_type;
        ctx.client_version = Cow::Owned(cli_version.to_string());

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
    pub fn new(strategy: AuthInfoPassStrategyEnum) -> Self {
        Self {
            pass_strategy: strategy,
        }
    }

    fn write_info_to_upstream_with_header(
        &self,
        rh: &mut RequestHeader,
        ctx: &GatewayContext,
    ) -> PingoraResult<()> {
        Self::insert_dispatch_val(rh)?;

        self.insert_auth_info(rh, self.build_authed_info_context(ctx, "test_u1"))?;

        Ok(())
    }

    fn build_authed_info_context(&self, ctx: &GatewayContext, uid: &str) -> WebPassContext {
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
        // 直接调用枚举的 encode 方法
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