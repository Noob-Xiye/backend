# RSWS (Rust Salvo Web Site)

本项目是一个基于 [Salvo](https://github.com/salvo-rs/salvo) 框架构建的现代化 Web 服务系统，旨在提供一个完整的商家-客户交易平台解决方案。

## 主要技术栈

### Web 框架

- **Salvo**: 用于处理 HTTP 请求、管理会话、生成 API 文档 (Salvo-oapi)、记录日志、处理文件上传、追踪请求和统一错误处理。

### 验证码

- **邮件验证码**: 基于 SMTP 实现，用于用户注册和登录流程，SMTP 服务通过网站基础信息进行配置。

### 数据存储

- **PostgreSQL**: 关系型数据库，使用 [Sea-ORM](https://www.sea-ql.org/SeaORM/docs/latest/zh-cn/) 进行数据库操作，支持连接池和实体关系映射 (ORM)。

### 缓存系统

- **Redis**: 内存数据库，用于数据缓存、用户会话存储和请求限流。

### Web3 收款系统

- **[anychain](https://github.com/0xcregis/anychain)**: 支持 TRC20 和 ERC20 加密货币支付，并提供自动回调处理。具体实现位于 `anychain_tron` 和 `anychain_eth` crates。
- **[Cregis API](https://developer-cn.cregis.com/api-reference/request)**: 提供企业钱包、多币种支付、订单管理及支付回调功能。

## 系统架构

<!-- 在这里插入您的系统架构图 -->

这是一个基于模块化设计的 Web 服务系统，主要由以下核心模块组成：

- **Bin**: 项目入口，负责初始化和启动服务器。
- **Core**: 核心基础库，提供数据实体、中间件、工具函数、错误处理和配置加载等公共功能。
- **Open_api**: 整合各业务模块路由，生成并挂载 OpenAPI (Swagger) 文档。
- **Admin**: 后台管理模块。
- **Customer**: 客户前台模块。
- **Merchant**: 商家前台模块。
- **Payment**: 支付处理模块。

数据流程大致如下：前端请求 -> Salvo Router (Open_api 整合) -> 中间件处理 (Core) -> Handler (Admin, Customer, Merchant, Payment) -> 数据操作 (Core 提供的 Sea-ORM 和 Redis 工具) -> 响应返回。

### 1. Admin 模块 [admin](./admin/)

#### 概述与依赖

Admin 模块负责后台管理系统的各项功能，它依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。

#### 功能特性

- **管理员账号管理** [account](./admin/src/account/): 包括登录认证、头像管理和个人信息维护。
- **网站基础信息配置** [settings](./admin/src/settings/): 用于配置网站名称、版本、Logo、SMTP 服务和支付回调 URL。
- **用户管理** [user](./admin/src/user/): 管理客户和商家账号，支持列表查看、详情查看、账号禁用和强制下线。
- **系统监控** [monitor](./admin/src/monitor/): 提供网站报表和基础信息查看修改功能。
- **日志管理** [log](./admin/src/log/): 查看和删除客户、商家和系统操作日志（如注册、登录、文件上传、信息修改、密码修改、购买记录等）。

### 2. Customer 模块 [customer](./customer/)

#### 概述与依赖

Customer 模块提供面向客户的前台功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。部分功能（如购买）可能依赖于 `payment` 模块。

#### 功能特性

- **账号管理** [account](./customer/src/account/): 包括注册、登录、头像管理、密码修改和个人信息维护。
- **订单管理** [order](./customer/src/order/): 支持商品购买和购买记录查看。
- **数据存储**: 客户相关数据实体定义在 `core` 模块的 [entities](./core/src/entities/) 目录下，包括雪花 ID、账号、昵称、密码、头像、权限、状态、注册/登录时间、IP 地址记录等。

### 3. Merchant 模块 [merchant](./merchant/)

#### 概述与依赖

Merchant 模块提供面向商家的前台功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。同时，它依赖于 `payment` 模块处理收款和支付相关逻辑。

#### 功能特性

- **账号管理** [account](./merchant/src/account/): 包括注册、登录、头像管理、密码修改和个人信息维护。
- **商品管理** [product](./merchant/src/product/): 支持商品上架、下架、信息修改和版本更新。
- **收款管理** [payment](./merchant/src/payment/): 管理 ERC/TRC 钱包地址和 Cregis 企业钱包。
- **销售管理** [sales](./merchant/src/sales/): 提供销售报表和订单记录查看。
- **数据存储**: 商家和商品相关数据实体定义在 `core` 模块的 [entities](./core/src/entities/) 目录下。
    - 商家数据包括雪花 ID、账号、昵称、密码、头像、权限、状态、各种钱包地址、注册/登录/更新时间、IP 地址记录等。
    - 商品数据包括雪花 ID、商品名称/价格/图片/描述文件、上架时间、商品版本/更新时间、下载链接、销量、商家雪花 ID 等。

### 4. Core 模块 [core](./core/)

#### 概述与依赖

Core 模块是系统的核心基础库，提供了其他所有模块所需的公共功能和数据结构，并且不直接依赖于其他业务模块。

#### 核心功能

- **数据库实体** [entities](./core/src/entities/): 使用 Sea-ORM 定义了系统中所有主要的数据表结构（实体），并包含相应的数据库操作函数。例如：管理员表、商家表、客户表、商品表、订单表、收款记录表、权限表、网站报表、操作日志表、日志配置表、网站信息表。
- **模型和数据库操作** [model](./core/src/model): 包含请求、响应返回的结构体，以及基于 [database.rs](./core/src/utils/database.rs) 数据库连接池的数据库操作函数。
- **中间件** [middleware](./core/src/middleware/): 基于 Salvo 实现，包含了处理请求和响应的通用中间件逻辑，如会话管理、日志服务、请求追踪、CSRF 防护、CORS 配置和权限验证。
- **工具函数** [utils](./core/src/utils/): 提供各种辅助功能，例如数据库/Redis 连接池 ([database.rs](./core/src/utils/database.rs), [redis.rs](./core/src/utils/redis.rs))、密码加密 ([crypto.rs](./core/src/utils/crypto.rs))、邮件验证码发送 ([email.rs](./core/src/utils/email.rs)) 和雪花 ID 生成 ([snowflake.rs](./core/src/utils/snowflake.rs))。此外，还定义了统一的响应返回结构体 `AppResult` 和错误返回结构体 `AppError`，JSON 格式统一为 `{code: *, msg: "", data: {} }`，错误代码调用 `error_list`。
- **错误处理** [error](./core/src/error/): 定义了系统统一的错误代码及处理逻辑。`error_list.rs` 中定义了统一错误代码常量，并划分了错误代码范围：1000-1999 系统错误，2000-2999 数据错误，3000-3999 业务错误等。
- **配置文件处理** [settings](./core/src/settings/): 负责读取和加载配置文件 `config.toml` 中的服务器、数据库和 Redis 配置。

### 5. Payment 模块 [payment](./payment/)

#### 概述与依赖

Payment 模块专注于处理所有的支付和收款相关业务逻辑，依赖于 `core` 模块提供基础数据结构和工具。它集成了第三方支付库和 API (anychain, Cregis)。为了防止订单之间混淆，**Payment 模块会为每一个订单生成带有 request-id 的异步线程进行处理**。

#### Web3 收款功能

- **支付处理** [handlers](./payment/src/handlers/): 包含具体的支付处理逻辑。
    - **anychain 收款** [anychain](./payment/src/anychain/): 实现基于 anychain 的 TRC20 和 ERC20 收款 ([trc.rs](./payment/src/anychain/trc.rs), [eth.rs](./payment/src/anychain/eth.rs))。
    - **Cregis 支付** [cregis](./payment/src/cregis/): 实现与 Cregis API 的交互，处理订单创建、查询和回调 ([create_order.rs](./payment/src/cregis/create_order.rs), [query_order.rs](./payment/src/cregis/query_order.rs), [callback.rs](./payment/src/cregis/callback.rs))。
- **订单管理** [order](./payment/src/order/): 处理支付相关的订单状态跟踪、支付记录和交易确认。

### 6. Open_api 模块 [open_api](./open_api/)

#### 概述与依赖

Open_api 模块负责集成各业务模块的路由，并生成 OpenAPI (Swagger) 文档。它依赖于 `admin`, `customer`, `merchant`, `payment` 模块的路由定义，并依赖于 `core` 模块提供的中间件。

#### Swagger 特性

- 基于 Salvo-oapi 实现 OpenAPI 接口文档。该模块汇集各业务模块的路由，并在此基础上生成和挂载 Swagger UI。
    - 模块内包含各业务模块的路由定义：admin 模块路由 ([admin_router.rs](./open_api/src/admin_router.rs))、customer 模块路由 ([customer_router.rs](./open_api/src/customer_router.rs))、merchant 模块路由 ([merchant_router.rs](./open_api/src/merchant_router.rs))。
    - 将模块路由合并的总路由在 [lib.rs](./open_api/src/lib.rs) 中。
    - 总路由上挂载了 core 模块内的常用中间件，如会话管理、日志服务、请求追踪、跨域和鉴权等。
    - 最后在总路由上挂载基于 Salvo-oapi 实现的 Swagger UI。
- `exposed_headers`: 暴露的跨域响应头列表。
- `max_age`: 跨域预检请求缓存时间（秒）。

### 7. Bin 模块 [bin](./bin/)

#### 概述与依赖

Bin 模块作为整个项目的入口点，负责服务器的初始化和启动。它主要依赖于 `core` 模块进行配置加载和初始化设置，并依赖于 `open_api` 模块获取整合后的路由配置。

#### 程序入口与服务初始化

- 该模块包含：
    - [init.rs](./bin/src/init.rs): 用于 Salvo 服务器的初始化设置，包括加载配置文件、建立数据库和 Redis 连接池等，依赖于 core 模块中的实体定义和配置文件加载。
    - [main.rs](./bin/src/main.rs): 整个程序的入口，负责解析命令行参数、调用 `init.rs` 进行初始化，并启动 Salvo 服务器，实现优雅关机。

## 数据流程概览

1. 前端请求发送至 Salvo Router (由 `open_api` 模块整合)。
2. 请求经过 `core` 模块提供的中间件处理，如日志记录、权限验证、会话管理、CSRF 防护、CORS 处理等。
3. Handler (位于 Admin, Customer, Merchant, Payment 模块中) 处理具体的业务逻辑。
4. 数据操作通过 `core` 模块提供的 Sea-ORM 和 Redis 工具进行，包括 Redis 缓存的查询/更新和 PostgreSQL 数据库的数据持久化。
5. 处理结果作为响应返回给前端。

## 快速开始

### 环境要求

- Rust 1.70+
- PostgreSQL 12+
- Redis 6+

### 安装与运行

```bash
# 克隆项目
git clone https://github.com/Noob-Xiye/backend.git

# 切换到项目目录
cd backend

# 安装依赖
cargo build --workspace

# 配置项目
# 将 config.toml.example 复制为 config.toml (如果存在示例文件的话)
# cp config.toml.example config.toml
# 编辑 config.toml 文件，根据"配置说明"部分配置数据库、Redis 等信息。

# 运行项目
cargo run
```

## 配置说明

项目通过 `config.toml` 文件进行配置。请根据您的实际环境修改以下配置项：

### 服务器配置 `[server]`

- `host`: 服务器绑定的 IP 地址，默认为 "0.0.0.0"。
- `port`: 服务器监听的端口，默认为 8080。
- `workers`: 工作线程数，默认为 4。
- `keep_alive`: Keep-Alive 超时时间（秒），默认为 60。
- `max_connections`: 最大连接数，默认为 10000。
- `tcp_nodelay`: 是否启用 TCP_NODELAY，默认为 true。
- `tcp_keepalive`: TCP Keep-Alive 时间间隔（秒），默认为 300。

### 数据库配置 `[database]`

- `url`: PostgreSQL 数据库连接 URL，格式为 `postgres://user:password@host:port/database`。
- `max_connections`: 数据库连接池最大连接数，默认为 100。
- `min_connections`: 数据库连接池最小连接数，默认为 5。
- `connect_timeout`: 连接超时时间（秒），默认为 10。
- `idle_timeout`: 空闲连接超时时间（秒），默认为 300。
- `max_lifetime`: 连接最大生命周期（秒），默认为 1800。

### Redis 配置 `[redis]`

- `url`: Redis 连接 URL，格式为 `redis://host:port`。
- `password`: Redis 密码，默认为空。
- `db`: Redis 数据库索引，默认为 0。
- `pool_size`: Redis 连接池大小，默认为 100。
- `timeout`: 连接超时时间（秒），默认为 5。
- `max_retries`: 最大重试次数，默认为 3。

### 会话配置 `[session]`

- `name`: 会话 Cookie 名称。
- `expires`: 会话过期时间（秒），默认为 86400 (24小时)。
- `path`: 会话 Cookie 路径。
- `domain`: 会话 Cookie 域名。
- `secure`: 是否只在 HTTPS 连接中发送 Cookie。
- `http_only`: 是否设置 HttpOnly 属性。
- `same_site`: SameSite 属性设置。

### 监控配置 `[monitoring]`

- `sentry_dsn`: Sentry DSN，用于错误监控。
- `prometheus_port`: Prometheus 监控端口，默认为 9090。
- `metrics_path`: Prometheus 指标路径，默认为 "/metrics"。

### 安全配置 `[security]`

- `rate_limit`: 每秒请求限流数，默认为 1000。
- `trust_proxy`: 是否信任代理。
- `allowed_origins`: 允许的跨域请求来源列表。
- `allowed_methods`: 允许的跨域请求方法列表。
- `allowed_headers`: 允许的跨域请求头列表。
- `exposed_headers`: 暴露的跨域响应头列表。
- `max_age`: 跨域预检请求缓存时间（秒）。

### 雪花算法配置 `[snowflake]`

- `worker_id`: 工作节点 ID。
- `datacenter_id`: 数据中心 ID。
- `epoch`: 雪花算法的起始时间戳（毫秒）。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。
