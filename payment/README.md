# Payment 模块

该模块专注于处理所有的支付和收款相关业务逻辑。

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
