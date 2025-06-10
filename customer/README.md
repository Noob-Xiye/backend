# Customer 模块

## 项目整体结构
```
backend/
├── admin/          # 后台管理模块
├── bin/            # 项目入口
├── core/           # 核心基础库
├── customer/       # 客户前台模块
│   ├── src/
│   │   ├── account/       # 账号管理
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── profile.rs
│   │   │   └── security.rs
│   │   ├── order/         # 订单管理
│   │   │   ├── mod.rs
│   │   │   ├── purchase.rs
│   │   │   ├── history.rs
│   │   │   └── status.rs
│   │   └── lib.rs         # 模块入口
│   └── Cargo.toml         # 依赖配置
├── merchant/       # 商家前台模块
├── open_api/       # API 文档和路由整合
├── payment/        # 支付处理模块
├── config.toml     # 配置文件
└── Cargo.toml      # 项目依赖配置
```

## Customer 模块说明
客户前台系统模块，为终端用户提供商品浏览和购买功能。

### 主要功能
- 账号管理
  - 注册与登录
  - 个人信息管理
  - 安全设置
  - 密码管理
- 订单管理
  - 商品购买
  - 订单历史
  - 订单状态追踪
  - 支付管理
- 商品相关
  - 商品浏览
  - 购物车管理
  - 商品搜索
  - 收藏功能

### 依赖说明
- salvo: Web框架
- sea-orm: 数据库操作
- serde: 序列化/反序列化
- tokio: 异步运行时
- tracing: 日志追踪
- chrono: 时间处理

### 模块依赖
- ./core/src/entities: 数据库实体
- ./core/src/models: 数据模型
- ./core/src/middleware: 中间件
- ./core/src/utils: 工具函数

## 概述与依赖

Customer 模块提供面向客户的前台功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。部分功能（如购买）可能依赖于 `payment` 模块。

## 客户功能特性

- 账号管理 [account](./src/account/)
  - 注册 [register.rs](./src/account/register.rs)
  - 登录 [login.rs](./src/account/login.rs)
  - 头像管理 [profile.rs](./src/account/profile.rs)
  - 密码修改 [profile.rs](./src/account/profile.rs)
  - 个人信息维护 [profile.rs](./src/account/profile.rs)
- 订单管理 [order](./src/order/)
  - 商品购买 [buy.rs](./src/order/buy.rs)
  - 购买记录查看 [history.rs](./src/order/history.rs)
- 数据存储：相关实体定义在 Core 模块中的 [entities](./../core/src/entities/) 目录下。
  - 管理客户相关的数据，如雪花ID、账号、昵称、密码、头像、权限、状态、注册/登录时间、IP地址记录等。
