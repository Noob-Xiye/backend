use salvo::logging::AccessLogMiddleware;
use salvo::prelude::*;

pub fn access_log_middleware() -> AccessLogMiddleware {
    AccessLogMiddleware::new(
        // 使用默认格式或自定义。示例: :status :method :uri :response-time
        salvo::logging::Logger::default(),
    )
}
