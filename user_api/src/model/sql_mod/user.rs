use sqlx::FromRow;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    /// 主键ID
    pub id: i32,
    /// 用户唯一标识
    pub uid: String,
    /// 用户昵称
    pub nickname: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
    /// 状态: 1=正常
    pub state: i8,
    /// 创建时间戳（毫秒或秒）
    pub cts: i64,
    /// 更新时间戳（毫秒或秒）
    pub uts: Option<i64>,
}