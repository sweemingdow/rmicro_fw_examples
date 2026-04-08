use proto_bin::auth_api::ParseTokenResp;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::time::Instant;

pub struct GatewayContext {
    pub req_id: String,
    pub auth_resp: Cow<'static, Option<ParseTokenResp>>,
    pub client_type: u8,
    pub client_version: Cow<'static, str>,
    pub start: Instant,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PassContext {
    pub req_id: String,

    pub uid: Option<String>,

    pub client_type: u8,

    pub client_version: String,

    pub in_white: bool,

    pub in_callback: bool,

    pub in_open: bool,
}
