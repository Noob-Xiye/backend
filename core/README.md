# Core 模块

## 项目整体结构
```
backend/
├── admin/          # 后台管理模块
├── bin/            # 项目入口
├── core/           # 核心基础库
│   ├── src/
│   │   ├── config/         # 配置管理
│   │   │   ├── mod.rs
│   │   │   └── config.rs   # 配置加载
│   │   ├── constants/      # 常量定义
│   │   │   ├── mod.rs
│   │   │   ├── depot_key.rs
|   |   |   ├── http_header.rs
│   │   │   └── error_code.rs
│   │   ├── entities/       # 数据库实体
│   │   │   ├── mod.rs
│   │   │   ├── admin.rs
│   │   │   ├── customer.rs
│   │   │   ├── merchant.rs
│   │   │   ├── order.rs
│   │   │   ├── payment_record.rs
│   │   │   ├── permission.rs
│   │   │   ├── product.rs
│   │   │   ├── web_profile.rs
│   │   │   ├── logging.rs
│   │   │   ├── report.rs
│   │   │   └──、、、
│   │   ├── error/          # 错误处理机制
│   │   │   ├── mod.rs
│   │   │   └── app_error.rs
│   │   ├── middleware/     # 中间件组件
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs     # 简易鉴权中间件
│   │   │   ├── cors.rs     # CORS
│   │   │   ├── csrf.rs     # CSRF
│   │   │   ├── logging.rs  # 日志中间件
│   │   │   ├── request_id.rs # 请求追踪
│   │   │   └── session.rs  # 会话管理
│   │   ├── models/         # 数据模型
│   │   │   ├── mod.rs
│   │   │   ├── response.rs
│   │   │   └──
│   │   ├── utils/          # 工具函数
│   │   │   ├── mod.rs
│   │   │   ├── pwd_hash.rs   #密码加密
│   │   │   ├── database.rs #数据库连接池
│   │   │   ├── email.rs    #邮件验证码
│   │   │   ├── get_request_id.rs   #获取请求ID
│   │   │   ├── redis.rs    #redis连接池
│   │   │   └── snowflake.rs  #生成雪花ID
│   │   └── lib.rs          # 模块入口
│   └── Cargo.toml          # 依赖配置
├── customer/       # 客户前台模块
├── merchant/       # 商家前台模块
├── open_api/       # API 文档和路由整合
├── payment/        # 支付处理模块
├── config.toml     # 配置文件
└── Cargo.toml      # 项目依赖配置
```

## Core 模块说明
核心基础设施模块，提供项目的基础功能和通用组件。

### 主要功能
- 数据库实体和ORM操作
  - 用户实体（管理员、商家、客户）
  - 业务实体（订单、商品、支付记录）
  - 系统实体（日志、权限、配置）
- 中间件组件
  - 认证授权
  - 跨域处理
  - CSRF防护
  - 日志记录
  - 会话管理
  - 请求追踪
- 工具函数
  - 加密解密
  - 数据库操作
  - 邮件发送
  - Redis缓存
  - ID生成器
- 错误处理机制
  - 统一错误码
  - 错误响应格式
  - 异常处理
- 配置管理
  - 系统配置
  - 应用设置
  - 常量定义

### 依赖说明
- sea-orm: 数据库 ORM
- redis: Redis 客户端
- serde: 序列化/反序列化
- tokio: 异步运行时
- tracing: 日志追踪
- lettre: 邮件发送
- snowflake: 唯一标识符
- Argon2：密码加密

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
