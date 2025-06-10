# Admin 模块

## 项目整体结构
```
backend/
├── admin/          # 后台管理模块
│   ├── src/
│   │   ├── handlers/       # 请求处理器、业务服务
│   │   │   ├── mod.rs
│   │   │   ├── account.rs
│   │   │   ├── web_conf.rs
│   │   │   ├── customer.rs
│   │   │   ├── monitor.rs
│   │   │   ├── log.rs
│   │   │   └── 、、、
│   │   └── lib.rs         # 模块入口
│   └── Cargo.toml         # 依赖配置
├── bin/            # 项目入口
├── core/           # 核心基础库
├── customer/       # 客户前台模块
├── merchant/       # 商家前台模块
├── open_api/       # API 文档和路由整合
├── payment/        # 支付处理模块
├── config.toml     # 配置文件
└── Cargo.toml      # 项目依赖配置
```

## Admin 模块说明
后台管理系统模块，提供完整的系统管理功能。

### 主要功能
- 管理员账号管理
  - 账号认证
  - 权限管理
  - 角色分配
- 网站配置管理
  - 系统设置
  - 参数配置
  - 功能开关
- 用户管理
  - 商家管理
  - 客户管理
  - 用户审核
- 系统监控
  - 性能监控
  - 资源使用
  - 异常告警
- 日志管理
  - 操作日志
  - 系统日志
  - 安全日志

### 依赖说明
- salvo: Web框架
- sea-orm: 数据库操作
- serde: 序列化/反序列化
- tokio: 异步运行时
- tracing: 日志追踪
- ./core/src/models: 数据模型
- ./core/src/entities: 数据库实体
- ./core/src/utils: 工具函数
