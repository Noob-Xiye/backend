# Merchant 模块

该模块提供面向商家的前台功能。

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
