// TODO: 实现功能

// admin 模块 -> 用户管理 -> 客户用户
// 该文件包含处理管理员对客户用户进行管理（查看、详情、创建、更新、删除）的逻辑。

use salvo::prelude::*; // 引入 salvo 框架相关依赖
use anyhow::Result; // 引入 anyhow::Result 用于错误处理
use serde::{Deserialize, Serialize}; // 引入 serde 用于序列化和反序列化
use validator::Validate; // 引入 validator 用于数据验证
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, PaginatorTrait, ActiveModelTrait, Set}; // 引入 sea-orm 用于数据库操作

use core::error::AppError; // 引入核心模块的错误类型 AppError
use core::entities::customer; // 引入核心模块的 customer 实体
// TODO: 引入核心模块的加密工具函数 (例如用于设置初始密码)
// use core::utils::crypto;

// 定义客户用户响应数据结构 (用于列表和详情)
#[derive(Serialize)]
pub struct CustomerResponse {
    pub id: i64,
    pub account: String,
    pub nickname: String,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: String, // 例如: "active", "disabled"
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

// TODO: 定义查看客户列表的请求参数结构 (例如分页、过滤条件)
// #[derive(Deserialize)]
// pub struct GetCustomersRequest {
//     pub page: u64,
//     pub page_size: u64,
//     pub status: Option<String>, // 过滤状态
//     pub keyword: Option<String>, // 模糊搜索 (账号或昵称)
// }

// 定义创建客户的请求数据结构
#[derive(Deserialize, Validate)]
pub struct CreateCustomerRequest {
    #[validate(length(min = 1, message = "账号不能为空"))]
    pub account: String,
    #[validate(length(min = 6, message = "密码长度不能少于6位"))]
    pub password: String,
    #[validate(length(min = 1, message = "昵称不能为空"))]
    pub nickname: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    pub phone: Option<String>,
    // 管理员创建时可能直接设置状态
    pub status: Option<String>,
}

// 定义更新客户的请求数据结构
#[derive(Deserialize, Validate)]
pub struct UpdateCustomerRequest {
    // 可选字段，只提交需要更新的字段
    #[validate(length(min = 1, message = "昵称不能为空"))]
    pub nickname: Option<String>,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    // TODO: 修改密码需要单独的接口和流程
    // pub password: Option<String>,
}


/// 处理获取客户用户列表的请求
#[salvo::handler]
pub async fn get_customer_list(req: &mut Request, depot: &mut Depot) -> Result<Json<Vec<CustomerResponse>>, AppError> {
    // TODO: 解析请求参数 (例如分页、过滤条件)
    // let params = req.parse_json::<GetCustomersRequest>().await.map_err(|e| {
    //     AppError::BusinessError { code: 3001, message: format!("请求参数解析失败: {}", e) }
    // })?;

    // 获取数据库连接
    let db = depot.get::<sea_orm::DatabaseConnection>().ok_or_else(|| {
        // 标注依赖: salvo::Depot, core::error::AppError
        AppError::SystemError { code: 1000, message: "无法获取数据库连接".to_string() }
    })?;

    // TODO: 构建查询，根据参数进行过滤和分页
    // 标注依赖: core::entities::customer, sea-orm
    // let mut query = customer::Entity::find();
    // if let Some(status) = &params.status {
    //     query = query.filter(customer::Column::Status.eq(status.clone()));
    // }
    // if let Some(keyword) = &params.keyword {
    //     query = query.filter(customer::Column::Account.contains(keyword).or(customer::Column::Nickname.contains(keyword)));
    // }
    // let paginator = query.paginate(db, params.page_size);

    // TODO: 执行查询并获取分页结果
    // 标注依赖: sea-orm::PaginatorTrait
    // let customers = paginator.fetch_page(params.page).await.map_err(|e| {
    //     AppError::DataError { code: 2001, message: format!("查询客户列表失败: {}", e) }
    // })?;

    // 占位符: 模拟查询结果
    let customers: Vec<customer::Model> = vec![
        customer::Model {
            id: 101,
            account: "customer001".to_string(),
            password_hash: "".to_string(), // 列表中不返回密码哈希
            nickname: "普通客户A".to_string(),
            avatar_url: None,
            email: Some("customer_a@example.com".to_string()),
            phone: None,
            status: "active".to_string(),
            created_at: chrono::NaiveDateTime::now(),
            updated_at: chrono::NaiveDateTime::now(),
        },
        customer::Model {
            id: 102,
            account: "customer002".to_string(),
            password_hash: "".to_string(),
            nickname: "普通客户B".to_string(),
            avatar_url: None,
            email: None,
            phone: Some("13800001111".to_string()),
            status: "active".to_string(),
            created_at: chrono::NaiveDateTime::now(),
            updated_at: chrono::NaiveDateTime::now(),
        },
    ];


    // 将实体模型转换为响应结构
    let response_customers: Vec<CustomerResponse> = customers.into_iter().map(|cust| {
        CustomerResponse {
            id: cust.id,
            account: cust.account,
            nickname: cust.nickname,
            avatar_url: cust.avatar_url,
            email: cust.email,
            phone: cust.phone,
            status: cust.status,
            created_at: cust.created_at,
            updated_at: cust.updated_at,
        }
    }).collect();

    Ok(Json(response_customers))
}

/// 处理获取客户用户详情的请求
#[salvo::handler]
pub async fn get_customer_detail(req: &mut Request, depot: &mut Depot) -> Result<Json<CustomerResponse>, AppError> {
    // 从路径参数中获取客户 ID
    let customer_id: i64 = req.param("id").ok_or_else(|| {
        // 标注依赖: salvo::Request::param, core::error::AppError
        AppError::BusinessError { code: 3004, message: "客户 ID 不能为空".to_string() }
    })?;

    // 获取数据库连接
    let db = depot.get::<sea_orm::DatabaseConnection>().ok_or_else(|| {
        // 标注依赖: salvo::Depot, core::error::AppError
        AppError::SystemError { code: 1000, message: "无法获取数据库连接".to_string() }
    })?;

    // TODO: 根据 ID 从数据库查询客户详情
    // 标注依赖: core::entities::customer, sea-orm
    // let customer = customer::Entity::find_by_id(customer_id).one(db).await?.ok_or_else(|| {
    //     AppError::DataError { code: 2004, message: format!("客户用户未找到，ID: {}", customer_id) }
    // })?;

    // 占位符: 模拟查询结果
     let customer = customer::Model {
            id: customer_id,
            account: "customer001".to_string(),
            password_hash: "".to_string(), // 详情也不返回密码哈希
            nickname: "普通客户A".to_string(),
            avatar_url: None,
            email: Some("customer_a@example.com".to_string()),
            phone: None,
            status: "active".to_string(),
            created_at: chrono::NaiveDateTime::now(),
            updated_at: chrono::NaiveDateTime::now(),
        };


    // 将实体模型转换为响应结构
    let response_customer = CustomerResponse {
        id: customer.id,
        account: customer.account,
        nickname: customer.nickname,
        avatar_url: customer.avatar_url,
        email: customer.email,
        phone: customer.phone,
        status: customer.status,
        created_at: customer.created_at,
        updated_at: customer.updated_at,
    };

    Ok(Json(response_customer))
}

/// 处理创建客户用户的请求
#[salvo::handler]
pub async fn create_customer(req: &mut Request, depot: &mut Depot) -> Result<Json<CustomerResponse>, AppError> {
    // 从请求中解析创建数据
    let create_data = req.parse_json::<CreateCustomerRequest>().await.map_err(|e| {
        // 标注依赖: salvo::Request::parse_json, core::error::AppError
        AppError::BusinessError { code: 3001, message: format!("请求数据解析失败: {}", e) }
    })?;

    // 验证输入数据
    create_data.validate().map_err(|e| {
        // 标注依赖: validator::Validate, core::error::AppError
        AppError::BusinessError { code: 3002, message: format!("输入数据验证失败: {}", e) }
    })?;

    // 获取数据库连接
    let db = depot.get::<sea_orm::DatabaseConnection>().ok_or_else(|| {
        // 标注依赖: salvo::Depot, core::error::AppError
        AppError::SystemError { code: 1000, message: "无法获取数据库连接".to_string() }
    })?;

    // TODO: 检查账号是否已存在
    // 标注依赖: core::entities::customer, sea-orm
    // let existing_customer = customer::Entity::find().filter(customer::Column::Account.eq(&create_data.account)).one(db).await?;r
    // if existing_customer.is_some() {
    //     return Err(AppError::BusinessError { code: 3005, message: format!("账号 '{}' 已存在", create_data.account) });
    // }r

    // TODO: 对密码进行哈希处理
    // 标注依赖: core::utils::crypto::hash_password
    // let password_hash = crypto::hash_password(&create_data.password).map_err(|e| {
    //     AppError::SystemError { code: 1002, message: format!("密码哈希失败: {}", e) }
    // })?;r

    // TODO: 创建新的客户实体模型
    // 标注依赖: core::entities::customer, sea-orm
    // let new_customer = customer::ActiveModel {
    //     id: Set(0), // ID 通常由数据库生成
    //     account: Set(create_data.account),
    //     password_hash: Set(password_hash),
    //     nickname: Set(create_data.nickname),
    //     avatar_url: Set(None), // 新建时头像URL为空
    //     email: Set(create_data.email),
    //     phone: Set(create_data.phone),
    //     status: Set(create_data.status.unwrap_or_else(|| "active".to_string())), // 默认状态为 active
    //     created_at: Set(chrono::NaiveDateTime::now()),
    //     updated_at: Set(chrono::NaiveDateTime::now()),
    // };r

    // TODO: 插入到数据库
    // 标注依赖: sea-orm::ActiveModelTrait
    // let inserted_customer = new_customer.insert(db).await.map_err(|e| {
    //     AppError::DataError { code: 2005, message: format!("创建客户用户失败: {}", e) }
    // })?;r

    // 占位符: 模拟插入结果
    let inserted_customer = customer::Model {
        id: 103, // 模拟生成的 ID
        account: create_data.account,
        password_hash: "".to_string(),
        nickname: create_data.nickname,
        avatar_url: None,
        email: create_data.email,
        phone: create_data.phone,
        status: create_data.status.unwrap_or_else(|| "active".to_string()),
        created_at: chrono::NaiveDateTime::now(),
        updated_at: chrono::NaiveDateTime::now(),
    };


    // 将实体模型转换为响应结构
    let response_customer = CustomerResponse {
        id: inserted_customer.id,
        account: inserted_customer.account,
        nickname: inserted_customer.nickname,
        avatar_url: inserted_customer.avatar_url,
        email: inserted_customer.email,
        phone: inserted_customer.phone,
        status: inserted_customer.status,
        created_at: inserted_customer.created_at,
        updated_at: inserted_customer.updated_at,
    };

    Ok(Json(response_customer))
}

/// 处理更新客户用户的请求
#[salvo::handler]
pub async fn update_customer(req: &mut Request, depot: &mut Depot) -> Result<Json<CustomerResponse>, AppError> {
     // 从路径参数中获取客户 ID
    let customer_id: i64 = req.param("id").ok_or_else(|| {
        // 标注依赖: salvo::Request::param, core::error::AppError
        AppError::BusinessError { code: 3004, message: "客户 ID 不能为空".to_string() }
    })?;

    // 从请求中解析更新数据
    let update_data = req.parse_json::<UpdateCustomerRequest>().await.map_err(|e| {
        // 标注依赖: salvo::Request::parse_json, core::error::AppError
        AppError::BusinessError { code: 3001, message: format!("请求数据解析失败: {}", e) }
    })?;

    // 验证输入数据
    update_data.validate().map_err(|e| {
        // 标注依赖: validator::Validate, core::error::AppError
        AppError::BusinessError { code: 3002, message: format!("输入数据验证失败: {}", e) }
    })?;

    // 获取数据库连接
    let db = depot.get::<sea_orm::DatabaseConnection>().ok_or_else(|| {
        // 标注依赖: salvo::Depot, core::error::AppError
        AppError::SystemError { code: 1000, message: "无法获取数据库连接".to_string() }
    })?;

    // TODO: 根据 ID从数据库查询客户
    // 标注依赖: core::entities::customer, sea-orm
    // let customer = customer::Entity::find_by_id(customer_id).one(db).await?.ok_or_else(|| {
    //     AppError::DataError { code: 2004, message: format!("客户用户未找到，ID: {}", customer_id) }
    // })?;r
    // let mut active_model: customer::ActiveModel = customer.into();r

    // TODO: 更新字段
    // 标注依赖: sea-orm::Set
    // if let Some(nickname) = update_data.nickname {
    //     active_model.nickname = Set(nickname);
    // }r
    // if let Some(email) = update_data.email {
    //     active_model.email = Set(Some(email)); // 假设 email 是 Option<String>
    // }r
    // if let Some(phone) = update_data.phone {
    //      active_model.phone = Set(Some(phone)); // 假设 phone 是 Option<String>
    // }r
    // if let Some(status) = update_data.status {
    //      active_model.status = Set(status);
    // }r
    // active_model.updated_at = Set(chrono::NaiveDateTime::now());r


    // TODO: 保存到数据库
    // 标注依赖: sea-orm::ActiveModelTrait
    // let updated_customer = active_model.update(db).await.map_err(|e| {
    //     AppError::DataError { code: 2006, message: format!("更新客户用户失败: {}", e) }
    // })?;r

    // 占位符: 模拟更新后的结果
     let updated_customer = customer::Model {
            id: customer_id,
            account: "customer001".to_string(),
            password_hash: "".to_string(),
            nickname: update_data.nickname.unwrap_or("普通客户A".to_string()),
            avatar_url: None,
            email: update_data.email.or(Some("customer_a@example.com".to_string())),
            phone: update_data.phone.or(None),
            status: update_data.status.unwrap_or("active".to_string()),
            created_at: chrono::NaiveDateTime::now(), // 模拟旧的创建时间
            updated_at: chrono::NaiveDateTime::now(),
        };


    // 将实体模型转换为响应结构
    let response_customer = CustomerResponse {
        id: updated_customer.id,
        account: updated_customer.account,
        nickname: updated_customer.nickname,
        avatar_url: updated_customer.avatar_url,
        email: updated_customer.email,
        phone: updated_customer.phone,
        status: updated_customer.status,
        created_at: updated_customer.created_at,
        updated_at: updated_customer.updated_at,
    };

    Ok(Json(response_customer))
}

/// 处理删除客户用户的请求
#[salvo::handler]
pub async fn delete_customer(req: &mut Request, depot: &mut Depot) -> Result<Json<usize>, AppError> {
     // 从路径参数中获取客户 ID
    let customer_id: i64 = req.param("id").ok_or_else(|| {
        // 标注依赖: salvo::Request::param, core::error::AppError
        AppError::BusinessError { code: 3004, message: "客户 ID 不能为空".to_string() }
    })?;

    // 获取数据库连接
    let db = depot.get::<sea_orm::DatabaseConnection>().ok_or_else(|| {
        // 标注依赖: salvo::Depot, core::error::AppError
        AppError::SystemError { code: 1000, message: "无法获取数据库连接".to_string() }
    })?;

    // TODO: 根据 ID 从数据库删除客户
    // 标注依赖: core::entities::customer, sea-orm
    // let delete_result = customer::Entity::delete_by_id(customer_id).exec(db).await.map_err(|e| {
    //     AppError::DataError { code: 2007, message: format!("删除客户用户失败: {}", e) }
    // })?;r
    // let rows_affected = delete_result.rows_affected;r

    // TODO: 检查是否删除了记录
    // 标注依赖: sea-orm::DeleteResult
    // if rows_affected == 0 {
    //      return Err(AppError::DataError { code: 2004, message: format!("客户用户未找到，ID: {}", customer_id) });
    // }
r


    // 占位符: 模拟删除结果
    let rows_affected = 1;


    Ok(Json(rows_affected))
}

// TODO: 添加处理批量删除客户用户的函数 (delete_customers)
// TODO: 添加处理修改客户用户密码的函数 (change_customer_password)
// ...
