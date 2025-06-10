# Open API 模块

## 项目整体结构
```
backend/
├── admin/          # 后台管理模块
├── bin/            # 项目入口
├── core/           # 核心基础库
├── customer/       # 客户前台模块
├── merchant/       # 商家前台模块
├── open_api/       # API 文档和路由整合
│   ├── src/
│   │   ├── routers/
│   │   │   ├── admin_router.rs    # 管理员路由
│   │   │   ├── customer_router.rs # 客户路由
│   │   │   └── merchant_router.rs # 商家路由
│   │   ├── handlers
│   │   │   ├── admin_auth_handler.rs  # 处理管理员的公开路由和需要登陆的路由
│   │   └── lib.rs            # 模块入口
│   └── Cargo.toml            # 依赖配置
├── payment/        # 支付处理模块
├── config.toml     # 配置文件
└── Cargo.toml      # 项目依赖配置
```

## Open API 模块说明
API网关层模块，负责路由整合和API文档生成。

### 主要功能
- 路由整合
  - 统一路由管理
  - 中间件挂载
  - 请求转发
  - 路由分组
- OpenAPI文档
  - 文档生成
  - Swagger UI集成
  - API版本管理
  - 接口测试
- 健康检查
  - 服务状态监控
  - 依赖检查
  - 性能指标
  - 服务发现

### 依赖说明
- salvo: Web框架
- utoipa: OpenAPI文档生成
- serde: 序列化/反序列化
- tokio: 异步运行时
- tracing: 日志追踪
- swagger-ui: API文档UI
- ./core/src/entities: 数据库实体
- ./core/src/models: 数据模型
- ./core/src/middleware: 中间件
- ./core/src/utils: 工具函数
- ./admin/src/account: 管理管账号管理
- ./admin/src/logging: 日志管理
- ./admin/src/web: 网站管理
- ./admin/src/customer: 客户账号管理
- ./admin/src/merchant: 商家账号管理
- ./admin/src/monitor: 网站报表
- ./admin/src/product: 商品管理
- ./customer/src/account: 客户账号管理
- ./customer/src/order: 客户账号订单管理
- ./merchant/src/account: 商家账号管理
- ./merchant/src/payment: 商家账号收款记录
- ./merchant/src/product: 商家账号商品管理
- ./merchant/src/monitor: 商家报表

