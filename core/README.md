# Core 模块

Core 模块是系统的核心基础库，提供了其他所有模块所需的公共功能和数据结构。

## 概述与依赖

Core 模块是系统的核心基础库，提供了其他所有模块所需的公共功能和数据结构。它不直接依赖于其他业务模块。

## 核心功能特性

- 数据库实体 [entities](./src/entities/): 定义了系统中所有主要的数据表结构，使用 Sea-ORM。
  - 管理员表 [admin.rs](./src/entities/admin.rs)
  - 商家表 [merchant.rs](./src/entities/merchant.rs)
  - 客户表 [customer.rs](./src/entities/customer.rs)
  - 商品表 [product.rs](./src/entities/product.rs)
  - 订单表 [order.rs](./src/entities/order.rs)
  - 收款记录表 [payment_record.rs](./src/entities/payment_record.rs)
  - 权限表 [permission.rs](./src/entities/permission.rs)
  - 网站报表 [report.rs](./src/entities/report.rs)
  - 操作日志表 [logging.rs](./src/entities/logging.rs)
  - 日志配置表 [logger_conf.rs](./src/entities/logger_conf.rs)
  - 网站信息表 [web_profile.rs](./src/entities/web_profile.rs)
- 中间件 [middleware](./src/middleware/): 包含了处理请求和响应的通用中间件逻辑，基于 Salvo 实现。
  - 会话管理 [session.rs](./src/middleware/session.rs)
  - 日志服务 [logging.rs](./src/middleware/logging.rs)
  - 请求追踪 [request_id.rs](./src/middleware/request_id.rs)
  - CSRF防护 [csrf.rs](./src/middleware/csrf.rs)
  - CORS配置 [cors.rs](./src/middleware/cors.rs)
  - 权限验证 [auth.rs](./src/middleware/auth.rs)
- 工具函数 [utils](./src/utils/): 提供各种辅助功能，如数据库/Redis连接池、密码加密、邮件发送、雪花ID生成等。
  - 数据库连接池 [database.rs](./src/utils/database.rs)
  - Redis连接池 [redis.rs](./src/utils/redis.rs)
  - 密码加密 [crypto.rs](./src/utils/crypto.rs)
  - 邮件验证码发送 [email.rs](./src/utils/email.rs)
  - 雪花ID生成 [snowflake.rs](./src/utils/snowflake.rs)
- 错误返回代码 [error](./src/error/): 定义了系统统一的错误代码及处理逻辑。
  - `error_list.rs` 中存返回的统一错误代码，并对应常量 [error_list.rs](./src/error/error_list.rs)。错误代码范围划分：1000-1999 系统错误，2000-2999 数据错误，3000-3999 业务错误等。
- 配置文件 [settings](./src/settings/): 处理配置文件的读取和加载。
  - `config.rs` 读取配置文件 `config.toml` 中的 server config、database config、 redis config 并加载 [config.rs](./src/settings/config.rs)
