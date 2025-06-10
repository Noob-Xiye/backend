# Merchant 模块

## 项目整体结构
```
backend/
├── admin/          # 后台管理模块
├── bin/            # 项目入口
├── core/           # 核心基础库
├── customer/       # 客户前台模块
├── merchant/       # 商家前台模块
│   ├── src/
│   │   ├── account/       # 账号管理handler
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── profile.rs
│   │   │   └── security.rs
│   │   ├── product/       # 商品管理handler
│   │   │   ├── mod.rs
│   │   │   ├── inventory.rs
│   │   │   ├── category.rs
│   │   │   └── price.rs
│   │   ├── payment/       # 收款管理handler
│   │   │   ├── mod.rs
│   │   │   ├── account.rs
│   │   │   ├── record.rs
│   │   │   └── settlement.rs
│   │   ├── sales/         # 销售管理handler
│   │   │   ├── mod.rs
│   │   │   ├── order.rs
│   │   │   ├── statistics.rs
│   │   │   └── customer.rs
│   │   └── lib.rs         # 模块入口
│   └── Cargo.toml         # 依赖配置
├── open_api/       # API 文档和路由整合
├── payment/        # 支付处理模块
├── config.toml     # 配置文件
└── Cargo.toml      # 项目依赖配置
```

## Merchant 模块说明
商家前台系统模块，为商家提供完整的业务管理功能。

### 主要功能
- 账号管理
  - 注册与认证
  - 个人信息管理
  - 安全设置
  - 密码管理
- 商品管理
  - 商品上架
  - 库存管理
  - 价格设置
  - 分类管理
- 收款管理
  - 收款账户
  - 交易记录
  - 结算管理
  - 对账功能
- 销售管理
  - 订单处理
  - 销售统计
  - 客户管理
  - 数据分析

### 依赖说明
- salvo: Web框架
- sea-orm: 数据库操作
- serde: 序列化/反序列化
- tokio: 异步运行时
- tracing: 日志追踪
- chrono: 时间处理
- snowflake: 唯一标识符

### 模块依赖
- ./core/src/entities: 数据库实体
- ./core/src/models: 数据模型
- ./core/src/middleware: 中间件
- ./core/src/utils: 工具函数

## 概述与依赖

Merchant 模块提供面向商家的前台功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。同时，它依赖于 `payment` 模块处理收款和支付相关逻辑。

## 商家功能特性

- 账号管理 [account](./src/account/)
  - 注册 [register.rs](./src/account/register.rs)
  - 登录 [login.rs](./src/account/login.rs)
  - 头像管理 [profile.rs](./src/account/profile.rs)
  - 密码修改 [profile.rs](./src/account/profile.rs)
  - 个人信息维护 [profile.rs](./src/account/profile.rs)
- 商品管理 [product](./src/product/)
  - 商品上架 [create.rs](./src/product/create.rs)
  - 商品下架 [remove.rs](./src/product/remove.rs)
  - 商品信息修改 [edit.rs](./src/product/edit.rs)
  - 商品版本更新 [update.rs](./src/product/update.rs)
- 收款管理 [payment](./src/payment/)
  - ERC钱包地址 [wallet.rs](./src/payment/wallet.rs)
  - TRC钱包地址 [wallet.rs](./src/payment/wallet.rs)
  - Cregis企业钱包 [cregis_wallet.rs](./src/payment/cregis_wallet.rs)
- 销售管理 [sales](./src/sales/)
  - 销售报表 [report.rs](./src/sales/report.rs)
  - 订单记录 [order_history.rs](./src/sales/order_history.rs)
- 数据存储：相关实体定义在 Core 模块中的 [entities](./../core/src/entities/) 目录下。
  - 管理商家相关的数据，如雪花ID、账号、昵称、密码、头像、权限、状态、各种钱包地址、注册/登录/更新时间、IP地址记录等。
  - 管理商品相关的数据，如雪花id、商品名称/价格/图片/描述文件、上架时间、商品版本/更新时间、下载链接、销量、商家雪花id等。
