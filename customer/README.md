# Customer 模块

该模块提供面向客户的前台功能。

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
