use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use uuid::Uuid;

pub struct GatewayContext {
    pub req_id: Uuid,
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
