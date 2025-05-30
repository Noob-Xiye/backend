# Admin 模块

该模块负责后台管理系统的各项功能。

## 概述与依赖

Admin 模块负责后台管理系统的各项功能，依赖于 `core` 模块提供的数据实体、中间件、工具函数和错误处理。

## 功能特性

- 管理员账号管理 [account](./src/account/)
  - 登录认证 [login.rs](./src/account/login.rs)
  - 头像管理 [profile.rs](./src/account/profile.rs)
  - 个人信息维护 [profile.rs](./src/account/profile.rs)
- 网站基础信息配置 [settings](./src/settings/)
  - 网站名称 [website.rs](./src/settings/website.rs)
  - 网站版本 [website.rs](./src/settings/website.rs)
  - 网站logo [website.rs](./src/settings/website.rs)
  - SMTP配置 [email.rs](./src/settings/email.rs)
  - 支付回调URL [payment.rs](./src/settings/payment.rs)
- 用户管理 [user](./src/user/)
  - 客户账号管理 [customer.rs](./src/user/customer.rs)
    - 账号列表查看
    - 详细信息查看
    - 账号禁用
    - 强制下线
  - 商家账号管理 [merchant.rs](./src/user/merchant.rs)
    - 账号列表查看
    - 详细信息查看
    - 账号禁用
    - 强制下线
- 系统监控 [monitor](./src/monitor/)
  - 网站报表 [report.rs](./src/monitor/report.rs)
  - 基础信息查看 [website_info.rs](./src/monitor/website_info.rs)
  - 基础信息修改 [website_info.rs](./src/monitor/website_info.rs)
- 日志管理 [log](./src/log/)
  - 查看、删除客户操作日志 [customer_log.rs](./src/log/customer_log.rs) (涵盖注册、登录、文件上传、信息修改、密码修改、购买记录等)
  - 查看、删除商家操作日志 [merchant_log.rs](./src/log/merchant_log.rs) (涵盖注册、登录、文件上传、信息修改、密码修改、商品操作记录、收款记录等)
  - 查看、删除系统操作日志 [system_log.rs](./src/log/system_log.rs) (如基础信息修改记录)
