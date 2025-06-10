# Payment 模块

## 项目整体结构
```
backend/
├── admin/          # 后台管理模块
├── bin/            # 项目入口
├── core/           # 核心基础库
├── customer/       # 客户前台模块
├── merchant/       # 商家前台模块
├── open_api/       # API 文档和路由整合
├── payment/        # 支付处理模块
│   ├── src/
│   │   ├── cregis/        # Cregis收款服务包括handler
│   │   └── lib.rs         # 模块入口
│   └── Cargo.toml         # 依赖配置
├── config.toml     # 配置文件
└── Cargo.toml      # 项目依赖配置
```

## Payment 模块说明
支付处理系统模块，负责处理所有支付相关的业务逻辑。

### 主要功能
- Cregis支付服务
  - 企业钱包集成
  - 多币种支付
  - 为每笔收款抽取佣金，佣金比例从数据库的商家账号获取


### 依赖说明
- cregis-api: Cregis支付集成
- sea-orm: 数据库操作
- serde: 序列化/反序列化
- tokio: 异步运行时

### 模块依赖
- ./core/src/entities: 数据库实体
- ./core/src/models: 数据模型
- ./core/src/middleware: 中间件
- ./core/src/utils: 工具函数

## 概述与依赖

Payment 模块专注于处理所有的支付和收款相关业务逻辑，依赖于 `core` 模块提供基础数据结构和工具。它集成了第三方支付库和 API (anychain, Cregis)。

## Web3收款功能特性

- 支付处理 [handlers](./src/handlers/): 包含具体的支付处理逻辑。
  - anychain收款 [anychain](./src/anychain/): 实现基于 anychain 的 TRC20 和 ERC20 收款。
    - TRC20收款 [trc.rs](./src/anychain/trc.rs)
    - ERC20收款 [eth.rs](./src/anychain/eth.rs)
  - Cregis支付 [cregis](./src/cregis/): 实现与 Cregis API 的交互，处理订单创建、查询和回调。
    - [订单创建](https://developer-cn.cregis.com/api-reference/request-apis/payment/payment-engine-create) [create_order.rs](./src/cregis/create_order.rs)
    - [订单查询](https://developer-cn.cregis.com/api-reference/request-apis/payment/payment-engine-query) [query_order.rs](./src/cregis/query_order.rs)
    - [支付回调](https://developer-cn.cregis.com/api-reference/callback/payment-engine) [callback.rs](./src/cregis/callback.rs)
- 订单管理 [order](./src/order/): 处理支付相关的订单状态跟踪、支付记录和交易确认。
  - 订单状态跟踪 [status.rs](./src/order/status.rs)
  - 支付记录 [record.rs](./src/order/record.rs)
  - 交易确认 [confirm.rs](./src/order/confirm.rs)
