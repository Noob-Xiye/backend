# RSWS (Rust Salvo Web Site)

## 项目简介
这是一个基于 [Salvo](https://github.com/salvo-rs/salvo) 框架构建的现代化 Web 服务系统，采用模块化设计，提供完整的商家-客户交易平台解决方案。

## 快速开始

### 环境要求
- Rust 1.70+
- PostgreSQL 12+
- Redis 6+

### 安装步骤
```bash
# 克隆项目
git clone https://github.com/Noob-Xiye/backend.git

# 切换到项目目录
cd backend

# 安装依赖
cargo build --workspace

# 配置项目
cp config.toml.example config.toml
# 编辑 config.toml 文件，配置数据库、Redis 等信息

# 运行项目
cargo run
```

## 项目结构
```
backend/
├── admin/          # 后台管理模块
├── bin/            # 项目入口
├── core/           # 核心基础库
├── customer/       # 客户前台模块
├── merchant/       # 商家前台模块
├── open_api/       # API 文档和路由整合
├── payment/        # 支付处理模块
├── config.toml     # 配置文件
└── Cargo.toml      # 项目依赖配置
```

## 核心模块说明

### 1. Core 模块
核心基础设施，提供：
- 数据库实体和ORM操作
- 中间件（会话、日志、追踪等）
- 工具函数（数据库连接、Redis、加密等）
- 错误处理机制
- 配置管理

### 2. Payment 模块
支付处理系统，集成：
- Cregis支付

### 3. Merchant 模块
商家前台系统，包含：
- 账号管理
- 商品管理
- 收款管理
- 销售管理

### 4. Customer 模块
客户前台系统，提供：
- 账号管理
- 订单管理
- 商品购买

### 5. Admin 模块
后台管理系统，包含：
- 管理员账号管理
- 网站配置管理
- 用户管理
- 系统监控
- 日志管理

### 6. Open_api 模块
API网关层，负责：
- 路由整合
- OpenAPI文档生成
- Swagger UI集成
- 中间件挂载

### 7. Bin 模块
项目入口，处理：
- 服务器初始化
- 配置加载
- 服务启动

## 技术栈

### Web 框架
- **Salvo**: 处理 HTTP 请求、会话管理、API 文档生成、日志记录等

### 数据存储
- **PostgreSQL**: 关系型数据库，使用 Sea-ORM 进行 ORM 操作
- **Redis**: 缓存和会话存储

### 支付系统
- **anychain**: TRC20 和 ERC20 加密货币支付
- **Cregis API**: 企业钱包、多币种支付、订单管理

### 安全特性
- CSRF 防护
- CORS 配置
- 会话管理
- 请求限流
- 权限验证

### 监控和运维
- 日志系统
- 请求追踪
- 性能监控
- 错误处理

## 配置说明

### 服务器配置
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4
keep_alive = 60
max_connections = 10000
```

### 数据库配置
```toml
[database]
url = "postgres://user:password@host:port/database"
max_connections = 100
min_connections = 5
```

### Redis 配置
```toml
[redis]
url = "redis://host:port"
password = ""
db = 0
pool_size = 100
```

## 开发指南

### 错误处理
系统使用统一的错误处理机制，错误代码范围：
- 1000-1999: 系统错误
- 2000-2999: 数据错误
- 3000-3999: 业务错误

### API 响应格式
所有 API 响应遵循统一格式：
```json
{
    "code": 0,
    "msg": "success",
    "data": {}
}
```

## 许可证
本项目采用 [MIT 许可证](LICENSE)。
