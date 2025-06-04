use serde::Serialize;
use salvo::prelude::Status;
use salvo::http::StatusCode;
use salvo::Response;
use salvo::prelude::anyhow;

use crate::core::error::{AppError, AppResult, error_list::SUCCESS};

/// 统一的 API 响应结构体
#[derive(Debug, Serialize)]
pub struct UnifiedResponse<T: Serialize> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

// 实现从 AppResult<T> 到 UnifiedResponse<T> 的转换
impl<T: Serialize> From<AppResult<T>> for UnifiedResponse<T> {
    fn from(result: AppResult<T>) -> Self {
        match result {
            Ok(data) => UnifiedResponse {
                code: SUCCESS as i32,
                msg: "Success".to_string(), // 成功消息可以直接写死或从某个地方获取
                data: Some(data),
            },
            Err(err) => UnifiedResponse {
                code: err.code(),
                msg: err.message(), // 使用 AppError 中的消息
                data: None,
            },
        }
    }
}

// 实现 Salvo 的 Responder 特征 for AppResult<T>
// 这将允许你在 handler 中返回 AppResult<T>，然后由 Salvo 自动转换为 HTTP 响应体
impl<T: Serialize> salvo::Responder for AppResult<T> {
    async fn write_body(mut self, _req: &mut salvo::Request, depoter: &mut salvo::Depot, res: &mut salvo::Response) {
        let status_code = match &self {
            Ok(_) => StatusCode::OK,
            Err(err) => err.status_code(), // 使用 AppError 中定义的 HTTP 状态码
        };

        let unified_response: UnifiedResponse<T> = self.into(); // 利用 From 实现转换

        // 将 UnifiedResponse 序列化为 JSON
        match serde_json::to_string(&unified_response) {
            Ok(json_body) => {
                res.status_code(status_code);
                res.header("Content-Type", "application/json");
                res.render(json_body);
            }
            Err(e) => {
                // 如果序列化失败，返回一个内部服务器错误
                let error_response = UnifiedResponse::<()> {
                    code: AppError::from_code(error_list::SYSTEM_INTERNAL_ERROR as i32).code(),
                    msg: format!("Failed to serialize response: {}", e),
                    data: None,
                };
                if let Ok(error_json) = serde_json::to_string(&error_response) {
                     res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                     res.header("Content-Type", "application/json");
                     res.render(error_json);
                } else {
                    // 如果错误响应也无法序列化，则只能返回纯文本错误
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                    res.render("Internal Server Error");
                }
            }
        }
    }
}

