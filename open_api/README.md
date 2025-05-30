# Open_api 模块

该模块负责集成各业务模块的路由，并生成 OpenAPI (Swagger) 文档。

## open_api模块 概述与依赖

Open-api 模块负责集成各业务模块的路由，并生成 OpenAPI (Swagger) 文档。它依赖于 `admin`, `customer`, `merchant`, `payment` 模块的路由定义，并依赖于 `core` 模块提供的中间件。

## Swagger特性

- 基于Salvo-oapi实现的OpenAPI接口文档。该模块汇集了各个业务模块的路由，并在此基础上生成和挂载Swagger UI。
  - 模块内包含各业务模块的路由定义：
    - admin模块的Router [admin_router.rs](./src/admin_router.rs) (包含公开和需要鉴权页面路由)
    - customer模块的Router [customer_router.rs](./src/customer_router.rs) (包含公开和需要鉴权页面路由)
    - merchant模块的Router [merchant_router.rs](./src/merchant_router.rs) (包含公开和需要鉴权页面路由)
  - 将模块Router合并的总Router [lib.rs](./src/lib.rs)
  - 总Router上挂载了 core 模块内的常用中间件，如会话管理、日志服务、请求追踪、跨域、鉴权等。
  - 最后在总Router上挂载基于Salvo-oapi实现的Swagger.
