use salvo::prelude::*;
use uuid::Uuid;

// TODO: 需要从配置中读取 request ID 的头部名称等。

#[derive(Clone)]
pub struct RequestIdMiddleware {
    header_name: String,
}

impl RequestIdMiddleware {
    pub fn new(header_name: &str) -> Self {
        RequestIdMiddleware { header_name: header_name.to_string() }
    }
}

#[async_trait]
impl Handler for RequestIdMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let request_id = req.header(&self.header_name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        // 将 request ID 存储在 depot 中，供后续 handler 使用
        depot.insert("request_id", request_id.clone());

        // 将 request ID 添加到响应头中
        res.add_header(self.header_name.clone(), request_id, true).unwrap();

        ctrl.call_next(req, depot, res, ctrl).await;
    }
}
