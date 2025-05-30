# RSWS (Rust Salvo Web Site)

基于Salvo框架构建的现代化Web服务系统，提供完整的商家-客户交易平台解决方案。

## 技术栈

### 核心框架

- **Web框架**: [Salvo](https://github.com/salvo-rs/salvo)
  - 请求处理：Salvo Router
  - 会话管理：Salvo Session
  - API文档：Salvo-oapi
  - 日志管理：Salvo logging
  - 文件上传：Salvo static
  - 请求追踪：Salvo request-id
  - 错误处理：Salvo anyhow

### 验证码

- **Email**: 基于SMTP实现的邮件验证码功能
  - 通过网站基础信息配置SMTP服务，用于邮件验证码注册和登录流程。

### 数据存储

- **数据库**: PostgreSQL
  - 使用Sea-ORM进行数据库操作，支持连接池、实体关系映射。

### 缓存系统

- **Redis**
  - 用于数据缓存、会话存储和请求限流。

### Web3收款系统

- **[anychain](https://github.com/0xcregis/anychain)**: 支持TRC20/ERC20支付，自动回调处理。
  - 具体实现位于 `anychain_tron` 和 `anychain_eth` crates，指向对应github。
- **[Cregis API](https://developer-cn.cregis.com/api-reference/request)**: 提供企业钱包、多币种支付、订单管理及支付回调支持。

## 系统架构

### 1. Admin模块 [admin](./admin/)

#### Admin模块 概述与依赖

Admin 模块负责后台管理系统的各项功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。

#### 管理员功能特性

- 管理员账号管理 [account](./admin/src/account/)
  - 登录认证 [login.rs](./admin/src/account/login.rs)
  - 头像管理 [profile.rs](./admin/src/account/profile.rs)
  - 个人信息维护 [profile.rs](./admin/src/account/profile.rs)
- 网站基础信息配置 [settings](./admin/src/settings/)
  - 网站名称 [website.rs](./admin/src/settings/website.rs)
  - 网站版本 [website.rs](./admin/src/settings/website.rs)
  - 网站logo [website.rs](./admin/src/settings/website.rs)
  - SMTP配置 [email.rs](./admin/src/settings/email.rs)
  - 支付回调URL [payment.rs](./admin/src/settings/payment.rs)
- 用户管理 [user](./admin/src/user/)
  - 客户账号管理 [customer.rs](./admin/src/user/customer.rs): 账号列表查看、详细信息查看、账号禁用、强制下线。
  - 商家账号管理 [merchant.rs](./admin/src/user/merchant.rs): 账号列表查看、详细信息查看、账号禁用、强制下线。
- 系统监控 [monitor](./admin/src/monitor/)
  - 网站报表 [report.rs](./admin/src/monitor/report.rs)
  - 基础信息查看及修改 [website_info.rs](./admin/src/monitor/website_info.rs)
- 日志管理 [log](./admin/src/log/)
  - 查看、删除客户操作日志 [customer_log.rs](./admin/src/log/customer_log.rs) (涵盖注册、登录、文件上传、信息修改、密码修改、购买记录等)
  - 查看、删除商家操作日志 [merchant_log.rs](./admin/src/log/merchant_log.rs) (涵盖注册、登录、文件上传、信息修改、密码修改、商品操作记录、收款记录等)
  - 查看、删除系统操作日志 [system_log.rs](./admin/src/log/system_log.rs) (如基础信息修改记录)

### 2. Customer模块 [customer](./customer/)

#### Customer模块 概述与依赖

Customer 模块提供面向客户的前台功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。部分功能（如购买）可能依赖于 `payment` 模块。

#### 客户功能特性

- 账号管理 [account](./customer/src/account/)
  - 注册 [register.rs](./customer/src/account/register.rs)
  - 登录 [login.rs](./customer/src/account/login.rs)
  - 头像管理 [profile.rs](./customer/src/account/profile.rs)
  - 密码修改 [profile.rs](./customer/src/account/profile.rs)
  - 个人信息维护 [profile.rs](./customer/src/account/profile.rs)
- 订单管理 [order](./customer/src/order/)
  - 商品购买 [buy.rs](./customer/src/order/buy.rs)
  - 购买记录查看 [history.rs](./customer/src/order/history.rs)
- 数据存储：相关实体定义在 Core 模块中的 [entities](./core/src/entities/) 目录下。
  - 管理客户相关的数据，如雪花ID、账号、昵称、密码、头像、权限、状态、注册/登录时间、IP地址记录等。

### 3. Merchant模块 [merchant](./merchant/)

#### Merchant模块 概述与依赖

Merchant 模块提供面向商家的前台功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。同时，它依赖于 `payment` 模块处理收款和支付相关逻辑。

#### 商家功能特性

- 账号管理 [account](./merchant/src/account/)
  - 注册 [register.rs](./merchant/src/account/register.rs)
  - 登录 [login.rs](./merchant/src/account/login.rs)
  - 头像管理 [profile.rs](./merchant/src/account/profile.rs)
  - 密码修改 [profile.rs](./merchant/src/account/profile.rs)
  - 个人信息维护 [profile.rs](./merchant/src/account/profile.rs)
- 商品管理 [product](./merchant/src/product/)
  - 商品上架 [create.rs](./merchant/src/product/create.rs)
  - 商品下架 [remove.rs](./merchant/src/product/remove.rs)
  - 商品信息修改 [edit.rs](./merchant/src/product/edit.rs)
  - 商品版本更新 [update.rs](./merchant/src/product/update.rs)
- 收款管理 [payment](./merchant/src/payment/)
  - ERC钱包地址 [wallet.rs](./merchant/src/payment/wallet.rs)
  - TRC钱包地址 [wallet.rs](./merchant/src/payment/wallet.rs)
  - Cregis企业钱包 [cregis_wallet.rs](./merchant/src/payment/cregis_wallet.rs)
- 销售管理 [sales](./merchant/src/sales/)
  - 销售报表 [report.rs](./merchant/src/sales/report.rs)
  - 订单记录 [order_history.rs](./merchant/src/sales/order_history.rs)
- 商家数据存储：相关实体定义在 Core 模块中的 [entities](./core/src/entities/) 目录下。
  - 管理商家相关的数据，如雪花ID、账号、昵称、密码、头像、权限、状态、各种钱包地址、注册/登录/更新时间、IP地址记录等。
- 商品数据存储：相关实体定义在 Core 模块中的 [entities](./core/src/entities/) 目录下。
  - 管理商品相关的数据，如雪花id、商品名称/价格/图片/描述文件、上架时间、商品版本/更新时间、下载链接、销量、商家雪花id等。

### 4. Core模块 [core](./core/)

#### Core模块 概述与依赖

Core 模块是系统的核心基础库，提供了其他所有模块所需的公共功能和数据结构。它不直接依赖于其他业务模块。

#### 核心功能特性

- 数据库实体 [entities](./core/src/entities/): 定义了系统中所有主要的数据表结构，使用 Sea-ORM。**在该目录下，不仅定义了数据表结构（实体），也包含了对这些实体进行数据库操作（如增删改查）的函数。**
  - 管理员表 [admin.rs](./core/src/entities/admin.rs)
  - 商家表 [merchant.rs](./core/src/entities/merchant.rs)
  - 客户表 [customer.rs](./core/src/entities/customer.rs)
  - 商品表 [product.rs](./core/src/entities/product.rs)
  - 订单表 [order.rs](./core/src/entities/order.rs)
  - 收款记录表 [payment_record.rs](./core/src/entities/payment_record.rs)
  - 权限表 [permission.rs](./core/src/entities/permission.rs)
  - 网站报表 [report.rs](./core/src/entities/report.rs)
  - 操作日志表 [logging.rs](./core/src/entities/logging.rs)
  - 日志配置表 [logger_conf.rs](./core/src/entities/logger_conf.rs)
  - 网站信息表 [web_profile.rs](./core/src/entities/web_profile.rs)
- 中间件 [middleware](./core/src/middleware/): 包含了处理请求和响应的通用中间件逻辑，基于 Salvo 实现。
  - 会话管理 [session.rs](./core/src/middleware/session.rs)
  - 日志服务 [logging.rs](./core/src/middleware/logging.rs)
  - 请求追踪 [request_id.rs](./core/src/middleware/request_id.rs)
  - CSRF防护 [csrf.rs](./core/src/middleware/csrf.rs)
  - CORS配置 [cors.rs](./core/src/middleware/cors.rs)
  - 权限验证 [auth.rs](./core/src/middleware/auth.rs)
- 工具函数 [utils](./core/src/utils/): 提供各种辅助功能，如数据库/Redis连接池、密码加密、邮件发送、雪花ID生成等。
  - 数据库连接池 [database.rs](./core/src/utils/database.rs)
  - Redis连接池 [redis.rs](./core/src/utils/redis.rs)
  - 密码加密 [crypto.rs](./core/src/utils/crypto.rs)
  - 邮件验证码发送 [email.rs](./core/src/utils/email.rs)
  - 雪花ID生成 [snowflake.rs](./core/src/utils/snowflake.rs)
- 错误返回代码 [error](./core/src/error/): 定义了系统统一的错误代码及处理逻辑。
  - `error_list.rs` 中存返回的统一错误代码，并对应常量 [error_list.rs](./core/src/error/error_list.rs)。错误代码范围划分：1000-1999 系统错误，2000-2999 数据错误，3000-3999 业务错误等。
- 配置文件 [settings](./core/src/settings/): 处理配置文件的读取和加载。
  - `config.rs` 读取配置文件 `config.toml` 中的 server config、database config、 redis config 并加载 [config.rs](./core/src/settings/config.rs)

### 5. Payment模块 [payment](./payment/)

#### Payment模块 概述与依赖

Payment 模块专注于处理所有的支付和收款相关业务逻辑，依赖于 `core` 模块提供基础数据结构和工具。它集成了第三方支付库和 API (anychain, Cregis)。

#### Web3收款功能特性

- 支付处理 [handlers](./payment/src/handlers/): 包含具体的支付处理逻辑。
  - anychain收款 [anychain](./payment/src/anychain/): 实现基于 anychain 的 TRC20 和 ERC20 收款。
    - TRC20收款 [trc.rs](./payment/src/anychain/trc.rs)
    - ERC20收款 [eth.rs](./payment/src/anychain/eth.rs)
  - Cregis支付 [cregis](./payment/src/cregis/): 实现与 Cregis API 的交互，处理订单创建、查询和回调。
    - [订单创建](https://developer-cn.cregis.com/api-reference/request-apis/payment/payment-engine-create) [create_order.rs](./payment/src/cregis/create_order.rs)
    - [订单查询](https://developer-cn.cregis.com/api-reference/request-apis/payment/payment-engine-query) [query_order.rs](./payment/src/cregis/query_order.rs)
    - [支付回调](https://developer-cn.cregis.com/api-reference/callback/payment-engine) [callback.rs](./payment/src/cregis/callback.rs)
- 订单管理 [order](./payment/src/order/): 处理支付相关的订单状态跟踪、支付记录和交易确认。
  - 订单状态跟踪 [status.rs](./payment/src/order/status.rs)
  - 支付记录 [record.rs](./payment/src/order/record.rs)
  - 交易确认 [confirm.rs](./payment/src/order/confirm.rs)

### 6. open_api模块 [open_api](./open_api/)

#### open_api模块 概述与依赖

Open-api 模块负责集成各业务模块的路由，并生成 OpenAPI (Swagger) 文档。它依赖于 `admin`, `customer`, `merchant`, `payment` 模块的路由定义，并依赖于 `core` 模块提供的中间件。

#### Swagger特性

- 基于Salvo-oapi实现的OpenAPI接口文档。该模块汇集了各个业务模块的路由，并在此基础上生成和挂载Swagger UI。
  - 模块内包含各业务模块的路由定义：
    - admin模块的Router [admin_router.rs](./open_api/src/admin_router.rs) (包含公开和需要鉴权页面路由)
    - customer模块的Router [customer_router.rs](./open_api/src/customer_router.rs) (包含公开和需要鉴权页面路由)
    - merchant模块的Router [merchant_router.rs](./open_api/src/merchant_router.rs) (包含公开和需要鉴权页面路由)
  - 将模块Router合并的总Router [lib.rs](./open_api/src/lib.rs)
  - 总Router上挂载了 core 模块内的常用中间件，如会话管理、日志服务、请求追踪、跨域、鉴权等。
  - 最后在总Router上挂载基于Salvo-oapi实现的Swagger。

### 7. bin模块 [bin](./bin/)

#### bin模块 概述与依赖

Bin 模块作为整个项目的入口点，负责服务器的初始化和启动。它主要依赖于 `core` 模块进行配置加载和初始化设置，并依赖于 `open_api` 模块获取整合后的路由配置。

#### 程序入口与服务初始化

- 该模块包含：
  - [init.rs](./bin/src/init.rs): 用于Salvo服务器的初始化设置，包括加载配置文件、建立数据库和 Redis 连接池等，依赖于core模块中的实体定义和配置文件加载。
  - [main.rs](./bin/src/main.rs): 整个程序的入口，负责解析命令行参数、调用 `init.rs` 进行初始化，并启动 Salvo 服务器，实现优雅关机。

## 数据流程

1. 前端请求 → Salvo Router (由 `open_api` 模块整合)
2. 中间件处理 (由 `core` 模块提供，在总 Router 上挂载)
   - Salvo logging日志记录
   - Salvo OAuth权限验证
   - Salvo Session会话管理
   - Salvo CSRF验证
   - Salvo CORS处理
3. Handler处理业务逻辑 (分散在 Admin, Customer, Merchant, Payment 模块中)
4. 数据操作 (通过 `core` 模块提供的 Sea-ORM 和 Redis 工具)
   - Redis缓存查询/更新
   - PostgreSQL数据持久化
5. 响应返回前端

## 快速开始

### 环境要求

- Rust 1.70+
- PostgreSQL 12+
- Redis 6+

### 安装

```bash
# 克隆项目
git clone https://github.com/Noob-Xiye/backend.git

# 安装依赖
cargo build --workspace

# 配置环境变量
cp .env.example .env
# 编辑 .env 文件配置数据库等信息

# 运行项目
cargo run
```

## 许可证

本项目采用 [MIT 许可证](LICENSE)。
