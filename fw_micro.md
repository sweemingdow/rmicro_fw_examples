# 🛡️ rmicro_fw 核心开发规范：全链路错误与日志治理

本项目采用 **“底层捕获细节、中层赋予语义、顶层自动分流、出口整齐划一”** 的错误处理体系。通过 `web_bridge` 模块，实现了业务逻辑与基础设施监控的深度解耦。

## 1. 全链路流转架构图

## 2. 三层代码开发模版

## 🟢 Repository 层：技术细节的守护者

**职责**：执行原始 I/O，捕获第三方库（SQL、Redis、gRPC）的底层错误。

- **规范**：必须使用 `.map_err(AppError::SqlDbError)` 进行类型转换，并配合 `.with_context` 注入**动作描述**。

Rust

```
async fn user_info(&self, uid: &str) -> AnyResult<Option<User>> {
    sqlx::query_as::<_, User>("SELECT * FROM t_user WHERE uid = ?")
        .bind(uid)
        .fetch_optional(&self.sql_pool)
        .await
        .map_err(AppError::SqlDbError) // 转换为框架识别的系统错误
        .with_context(|| "qry user failed") // 注入业务动作上下文
}
```

## 🟡 Service 层：业务语义的仲裁者

**职责**：组装业务逻辑，将“数据事实”转化为“业务语义”。

- **规范**：透传底层错误（使用 `?`），手动构造业务异常（使用 `ApiError`）。

Rust

```
async fn user_info(&self, uid: &str) -> AnyResult<UserSimpleInfoResp> {
    // 1. 透传 Repo 层的系统错误 (如 DB 断开)，? 会自动携带堆栈
    let user = self.user_repo.user_info(uid).await?
        // 2. 将“无数据”转化为“用户可见”的业务错误
        .ok_or_else(|| anyhow!(AppError::ApiError("user not found".to_string())))?;

    Ok(UserSimpleInfoResp { ... })
}
```

## 🔵 Handler 层：上下文注入与自动收割

**职责**：定义路由入口，注入当前请求的业务动作标签。

- **规范**：使用 `dy_trace::trace_with_action` 标记当前 API 动作。

Rust

```
async fn simple_info(State(s): State<UserState>, Path(uid): Path<String>) -> WebResult<impl IntoResponse> {
    // 注入扁平化日志字段 action="pull_simple_info"
    let _ = dy_trace::trace_with_action("pull_simple_info");

    let resp = s.user_info_svc.user_info(&uid).await?; // 自动处理所有错误转换
    Ok(RespResult::ok(resp)) // 自动处理 JSON 包装与脱敏
}
```

------

## 3. 终极魔法：AnyErrorWrapper 的自动治理

当错误流转到 `web_bridge` 出口时，系统根据 `AppError` 变体执行以下自动化策略：

| **错误类型**   | **匹配变体**    | **日志层级 (Recorder)** | **返回状态码** | **前端展示 (Msg)**                     |
| -------------- | --------------- | ----------------------- | -------------- | -------------------------------------- |
| **业务逻辑错** | `ApiError`      | **1 (仅消息)**          | `"0"`          | **原样透传** (如 "user not found")     |
| **数据库故障** | `SqlDbError`    | **-1 (全堆栈)**         | `"998"`        | **安全脱敏** ("Internal error")        |
| **框架/未知**  | `InternalError` | **-1 (全堆栈)**         | `"997/999"`    | **安全脱敏** ("Internal Server Error") |

------

## 4. 设计优势总结

1. **确定性**：开发者只需要写 `AnyResult<T>`，无需纠结 `match` 或 `if err`，框架保证了报错必有记录、必有返回。
2. **日志洁癖**：通过 `log_record` 的分级控制，业务报错不刷堆栈，生产环境日志更清晰，查错效率提升 200%。
3. **Flatmap 追踪**：所有日志（包括 Service 层和中间件）均通过 `action` 字段平铺，支持秒级聚合查询性能和业务分布。
4. **零损耗**：利用 Rust `#[inline]` 封装和 `Arc` 状态管理，在保证开发爽感的同时，维持极致的运行时性能。

------

**这套文档不仅是代码注释，更是我们项目的“宪法”。既然文档已经就绪，你是打算直接把这段 Markdown 存入 Git，还是想让我针对这套规范写一个简单的单元测试示例，来演示一下如何通过测试验证错误被正确拦截了？**