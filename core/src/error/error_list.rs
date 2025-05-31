use anyhow::Result;
use salvo::http::StatusCode;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

// Define constants for error code ranges
pub const SYSTEM_ERROR_BASE: u16 = 1000;
pub const DATA_ERROR_BASE: u16 = 2000;
pub const BUSINESS_ERROR_BASE: u16 = 3000;
pub const AUTH_ERROR_BASE: u16 = 4000;
pub const PAYMENT_ERROR_BASE: u16 = 5000;
pub const FILE_ERROR_BASE: u16 = 6000;
pub const EXTERNAL_SERVICE_ERROR_BASE: u16 = 7000;
pub const OTHER_ERROR_BASE: u16 = 8000;

// Define specific error codes and their descriptions
// System Errors (1000-1999)
pub const SYSTEM_INTERNAL_ERROR: u16 = SYSTEM_ERROR_BASE + 1;
pub const SYSTEM_INTERNAL_ERROR_MSG: &str = "内部系统错误。";

pub const CONFIG_LOAD_ERROR: u16 = SYSTEM_ERROR_BASE + 2;
pub const CONFIG_LOAD_ERROR_MSG: &str = "配置加载失败。";

pub const DB_CONNECT_ERROR: u16 = SYSTEM_ERROR_BASE + 3;
pub const DB_CONNECT_ERROR_MSG: &str = "数据库连接失败。";

pub const REDIS_CONNECT_ERROR: u16 = SYSTEM_ERROR_BASE + 4;
pub const REDIS_CONNECT_ERROR_MSG: &str = "Redis 连接失败。";

pub const SERVER_START_ERROR: u16 = SYSTEM_ERROR_BASE + 5;
pub const SERVER_START_ERROR_MSG: &str = "服务器启动失败。";

pub const MIDDLEWARE_INIT_ERROR: u16 = SYSTEM_ERROR_BASE + 6;
pub const MIDDLEWARE_INIT_ERROR_MSG: &str = "中间件初始化失败。";

pub const UTILS_INIT_ERROR: u16 = SYSTEM_ERROR_BASE + 7;
pub const UTILS_INIT_ERROR_MSG: &str = "工具函数初始化失败。";

pub const SHUTDOWN_ERROR: u16 = SYSTEM_ERROR_BASE + 8;
pub const SHUTDOWN_ERROR_MSG: &str = "系统关闭异常。";

pub const THREAD_POOL_ERROR: u16 = SYSTEM_ERROR_BASE + 9;
pub const THREAD_POOL_ERROR_MSG: &str = "线程池错误。";

pub const ROUTE_MATCH_ERROR: u16 = SYSTEM_ERROR_BASE + 10;
pub const ROUTE_MATCH_ERROR_MSG: &str = "路由匹配错误。";

pub const HANDLER_EXECUTION_ERROR: u16 = SYSTEM_ERROR_BASE + 11;
pub const HANDLER_EXECUTION_ERROR_MSG: &str = "请求处理执行错误。";

pub const RATE_LIMIT_CONFIG_ERROR: u16 = SYSTEM_ERROR_BASE + 12;
pub const RATE_LIMIT_CONFIG_ERROR_MSG: &str = "限流配置错误。";

pub const CORS_CONFIG_ERROR: u16 = SYSTEM_ERROR_BASE + 13;
pub const CORS_CONFIG_ERROR_MSG: &str = "CORS 配置错误。";

pub const CACHE_INIT_ERROR: u16 = SYSTEM_ERROR_BASE + 14;
pub const CACHE_INIT_ERROR_MSG: &str = "缓存初始化失败。";

pub const LOGGING_INIT_ERROR: u16 = SYSTEM_ERROR_BASE + 15;
pub const LOGGING_INIT_ERROR_MSG: &str = "日志系统初始化失败。";

// Data Errors (2000-2999)
pub const DB_OPERATION_ERROR: u16 = DATA_ERROR_BASE + 1;
pub const DB_OPERATION_ERROR_MSG: &str = "数据库操作失败。";

pub const RECORD_NOT_FOUND: u16 = DATA_ERROR_BASE + 2;
pub const RECORD_NOT_FOUND_MSG: &str = "记录未找到。";

pub const DUPLICATE_ENTRY: u16 = DATA_ERROR_BASE + 3;
pub const DUPLICATE_ENTRY_MSG: &str = "重复条目。";

pub const DATA_VALIDATION_ERROR: u16 = DATA_ERROR_BASE + 4;
pub const DATA_VALIDATION_ERROR_MSG: &str = "数据验证失败。";

pub const DATA_PARSE_ERROR: u16 = DATA_ERROR_BASE + 5;
pub const DATA_PARSE_ERROR_MSG: &str = "数据解析失败。";

pub const INVALID_ID_FORMAT: u16 = DATA_ERROR_BASE + 6;
pub const INVALID_ID_FORMAT_MSG: &str = "无效的ID格式。";

pub const DATABASE_MIGRATION_ERROR: u16 = DATA_ERROR_BASE + 7;
pub const DATABASE_MIGRATION_ERROR_MSG: &str = "数据库迁移失败。";

pub const REDIS_OPERATION_ERROR: u16 = DATA_ERROR_BASE + 8;
pub const REDIS_OPERATION_ERROR_MSG: &str = "Redis 操作失败。";

pub const CACHE_OPERATION_ERROR: u16 = DATA_ERROR_BASE + 9;
pub const CACHE_OPERATION_ERROR_MSG: &str = "缓存操作失败。";

pub const INVALID_DATA_FORMAT: u16 = DATA_ERROR_BASE + 10;
pub const INVALID_DATA_FORMAT_MSG: &str = "数据格式无效。";

pub const TRANSACTION_COMMIT_FAILED: u16 = DATA_ERROR_BASE + 11;
pub const TRANSACTION_COMMIT_FAILED_MSG: &str = "事务提交失败。";

pub const TRANSACTION_ROLLBACK_FAILED: u16 = DATA_ERROR_BASE + 12;
pub const TRANSACTION_ROLLBACK_FAILED_MSG: &str = "事务回滚失败。";

pub const INVALID_QUERY_PARAMETER: u16 = DATA_ERROR_BASE + 13;
pub const INVALID_QUERY_PARAMETER_MSG: &str = "无效的查询参数。";

pub const DATA_INCONSISTENCY: u16 = DATA_ERROR_BASE + 14;
pub const DATA_INCONSISTENCY_MSG: &str = "数据不一致。";

// Business Errors (3000-3999)
pub const INVALID_INPUT: u16 = BUSINESS_ERROR_BASE + 1;
pub const INVALID_INPUT_MSG: &str = "无效的输入参数。";

pub const INSUFFICIENT_STOCK: u16 = BUSINESS_ERROR_BASE + 2;
pub const INSUFFICIENT_STOCK_MSG: &str = "库存不足。";

pub const UNSUPPORTED_OPERATION: u16 = BUSINESS_ERROR_BASE + 3;
pub const UNSUPPORTED_OPERATION_MSG: &str = "不支持的操作。";

pub const BUSINESS_LOGIC_ERROR: u16 = BUSINESS_ERROR_BASE + 4;
pub const BUSINESS_LOGIC_ERROR_MSG: &str = "业务逻辑错误。";

pub const ACCOUNT_DISABLED: u16 = BUSINESS_ERROR_BASE + 5;
pub const ACCOUNT_DISABLED_MSG: &str = "账号已被禁用。";

pub const ACCOUNT_LOCKED: u16 = BUSINESS_ERROR_BASE + 6;
pub const ACCOUNT_LOCKED_MSG: &str = "账号已被锁定。";

pub const INVALID_OLD_PASSWORD: u16 = BUSINESS_ERROR_BASE + 7;
pub const INVALID_OLD_PASSWORD_MSG: &str = "原密码不正确。";

pub const PASSWORD_TOO_WEAK: u16 = BUSINESS_ERROR_BASE + 8;
pub const PASSWORD_TOO_WEAK_MSG: &str = "密码强度不足。";

pub const EMAIL_SEND_FAILED: u16 = BUSINESS_ERROR_BASE + 9;
pub const EMAIL_SEND_FAILED_MSG: &str = "邮件发送失败。";

pub const INVALID_VERIFICATION_CODE: u16 = BUSINESS_ERROR_BASE + 10;
pub const INVALID_VERIFICATION_CODE_MSG: &str = "无效的验证码。";

pub const VERIFICATION_CODE_EXPIRED: u16 = BUSINESS_ERROR_BASE + 11;
pub const VERIFICATION_CODE_EXPIRED_MSG: &str = "验证码已过期。";

pub const PRODUCT_NOT_OWNED_BY_MERCHANT: u16 = BUSINESS_ERROR_BASE + 12;
pub const PRODUCT_NOT_OWNED_BY_MERCHANT_MSG: &str = "商品不属于该商家。";

pub const ORDER_STATUS_TRANSITION_ERROR: u16 = BUSINESS_ERROR_BASE + 13;
pub const ORDER_STATUS_TRANSITION_ERROR_MSG: &str = "订单状态转换错误。";

pub const USER_STATUS_UPDATE_FAILED: u16 = BUSINESS_ERROR_BASE + 14;
pub const USER_STATUS_UPDATE_FAILED_MSG: &str = "用户状态更新失败。";

pub const PERMISSION_DENIED: u16 = BUSINESS_ERROR_BASE + 15;
pub const PERMISSION_DENIED_MSG: &str = "权限被拒绝。";

pub const SETTING_UPDATE_FAILED: u16 = BUSINESS_ERROR_BASE + 16;
pub const SETTING_UPDATE_FAILED_MSG: &str = "设置更新失败。";

pub const ACCOUNT_ALREADY_EXISTS: u16 = BUSINESS_ERROR_BASE + 17;
pub const ACCOUNT_ALREADY_EXISTS_MSG: &str = "账号已存在。";

pub const PASSWORD_RESET_FAILED: u16 = BUSINESS_ERROR_BASE + 18;
pub const PASSWORD_RESET_FAILED_MSG: &str = "密码重置失败。";

pub const PROFILE_UPDATE_FAILED: u16 = BUSINESS_ERROR_BASE + 19;
pub const PROFILE_UPDATE_FAILED_MSG: &str = "个人信息更新失败。";

pub const INVALID_PRODUCT_STATUS: u16 = BUSINESS_ERROR_BASE + 20;
pub const INVALID_PRODUCT_STATUS_MSG: &str = "无效的商品状态。";

pub const LOG_DELETION_FAILED: u16 = BUSINESS_ERROR_BASE + 21;
pub const LOG_DELETION_FAILED_MSG: &str = "日志删除失败。";

pub const REPORT_GENERATION_FAILED: u16 = BUSINESS_ERROR_BASE + 22;
pub const REPORT_GENERATION_FAILED_MSG: &str = "报表生成失败。";

pub const INVALID_EMAIL_FORMAT: u16 = BUSINESS_ERROR_BASE + 23;
pub const INVALID_EMAIL_FORMAT_MSG: &str = "无效的邮箱格式。";

pub const INVALID_PHONE_FORMAT: u16 = BUSINESS_ERROR_BASE + 24;
pub const INVALID_PHONE_FORMAT_MSG: &str = "无效的手机号格式。";

pub const IMAGE_PROCESSING_FAILED: u16 = BUSINESS_ERROR_BASE + 25;
pub const IMAGE_PROCESSING_FAILED_MSG: &str = "图片处理失败。";

pub const AVATAR_UPDATE_FAILED: u16 = BUSINESS_ERROR_BASE + 26;
pub const AVATAR_UPDATE_FAILED_MSG: &str = "头像更新失败。";

pub const WEBSITE_INFO_UPDATE_FAILED: u16 = BUSINESS_ERROR_BASE + 27;
pub const WEBSITE_INFO_UPDATE_FAILED_MSG: &str = "网站信息更新失败。";

pub const INVALID_DATE_RANGE: u16 = BUSINESS_ERROR_BASE + 28;
pub const INVALID_DATE_RANGE_MSG: &str = "无效的日期范围。";

pub const USER_ACCOUNT_NOT_FOUND: u16 = BUSINESS_ERROR_BASE + 29;
pub const USER_ACCOUNT_NOT_FOUND_MSG: &str = "用户账号未找到。";

pub const MERCHANT_ACCOUNT_NOT_FOUND: u16 = BUSINESS_ERROR_BASE + 30;
pub const MERCHANT_ACCOUNT_NOT_FOUND_MSG: &str = "商家账号未找到。";

pub const ADMIN_ACCOUNT_NOT_FOUND: u16 = BUSINESS_ERROR_BASE + 31;
pub const ADMIN_ACCOUNT_NOT_FOUND_MSG: &str = "管理员账号未找到。";

// Authentication Errors (4000-4999)
pub const UNAUTHORIZED: u16 = AUTH_ERROR_BASE + 1;
pub const UNAUTHORIZED_MSG: &str = "需要身份验证。";

pub const FORBIDDEN: u16 = AUTH_ERROR_BASE + 2;
pub const FORBIDDEN_MSG: &str = "权限不足。";

pub const INVALID_CREDENTIALS: u16 = AUTH_ERROR_BASE + 3;
pub const INVALID_CREDENTIALS_MSG: &str = "无效的凭据。";

pub const SESSION_EXPIRED: u16 = AUTH_ERROR_BASE + 4;
pub const SESSION_EXPIRED_MSG: &str = "会话已过期。";

pub const TOKEN_VALIDATION_FAILED: u16 = AUTH_ERROR_BASE + 5;
pub const TOKEN_VALIDATION_FAILED_MSG: &str = "Token 验证失败。";

pub const MISSING_CREDENTIALS: u16 = AUTH_ERROR_BASE + 6;
pub const MISSING_CREDENTIALS_MSG: &str = "缺少身份验证凭据。";

pub const USER_NOT_AUTHENTICATED: u16 = AUTH_ERROR_BASE + 7;
pub const USER_NOT_AUTHENTICATED_MSG: &str = "用户未认证。";

pub const INVALID_SIGNATURE: u16 = AUTH_ERROR_BASE + 8;
pub const INVALID_SIGNATURE_MSG: &str = "无效的签名。";

pub const ACCOUNT_NOT_ACTIVATED: u16 = AUTH_ERROR_BASE + 9;
pub const ACCOUNT_NOT_ACTIVATED_MSG: &str = "账号未激活。";

pub const CSRF_TOKEN_VALIDATION_FAILED: u16 = AUTH_ERROR_BASE + 10;
pub const CSRF_TOKEN_VALIDATION_FAILED_MSG: &str = "CSRF Token 验证失败。";

// Payment Errors (5000-5999)
pub const PAYMENT_FAILED: u16 = PAYMENT_ERROR_BASE + 1;
pub const PAYMENT_FAILED_MSG: &str = "支付处理失败。";

pub const INVALID_PAYMENT_METHOD: u16 = PAYMENT_ERROR_BASE + 2;
pub const INVALID_PAYMENT_METHOD_MSG: &str = "无效的支付方法。";

pub const PAYMENT_GATEWAY_ERROR: u16 = PAYMENT_ERROR_BASE + 3;
pub const PAYMENT_GATEWAY_ERROR_MSG: &str = "支付网关错误。";

pub const PAYMENT_CALLBACK_VERIFY_FAILED: u16 = PAYMENT_ERROR_BASE + 4;
pub const PAYMENT_CALLBACK_VERIFY_FAILED_MSG: &str = "支付回调验证失败。";

pub const CURRENCY_MISMATCH: u16 = PAYMENT_ERROR_BASE + 5;
pub const CURRENCY_MISMATCH_MSG: &str = "货币不匹配。";

pub const AMOUNT_MISMATCH: u16 = PAYMENT_ERROR_BASE + 6;
pub const AMOUNT_MISMATCH_MSG: &str = "金额不匹配。";

pub const ORDER_ALREADY_PAID: u16 = PAYMENT_ERROR_BASE + 7;
pub const ORDER_ALREADY_PAID_MSG: &str = "订单已支付。";

pub const INVALID_WALLET_ADDRESS: u16 = PAYMENT_ERROR_BASE + 8;
pub const INVALID_WALLET_ADDRESS_MSG: &str = "无效的钱包地址。";

pub const INSUFFICIENT_BALANCE: u16 = PAYMENT_ERROR_BASE + 9;
pub const INSUFFICIENT_BALANCE_MSG: &str = "余额不足。";

pub const TRANSACTION_NOT_FOUND: u16 = PAYMENT_ERROR_BASE + 10;
pub const TRANSACTION_NOT_FOUND_MSG: &str = "交易未找到。";

pub const WITHDRAWAL_FAILED: u16 = PAYMENT_ERROR_BASE + 11;
pub const WITHDRAWAL_FAILED_MSG: &str = "提现失败。";

pub const REFUND_FAILED: u16 = PAYMENT_ERROR_BASE + 12;
pub const REFUND_FAILED_MSG: &str = "退款失败。";

pub const PAYMENT_METHOD_NOT_SUPPORTED: u16 = PAYMENT_ERROR_BASE + 13;
pub const PAYMENT_METHOD_NOT_SUPPORTED_MSG: &str = "不支持的支付方式。";

pub const PAYMENT_AMOUNT_TOO_SMALL: u16 = PAYMENT_ERROR_BASE + 14;
pub const PAYMENT_AMOUNT_TOO_SMALL_MSG: &str = "支付金额过小。";

pub const PAYMENT_TIMEOUT: u16 = PAYMENT_ERROR_BASE + 15;
pub const PAYMENT_TIMEOUT_MSG: &str = "支付超时。";

pub const INCONSISTENT_PAYMENT_STATE: u16 = PAYMENT_ERROR_BASE + 16;
pub const INCONSISTENT_PAYMENT_STATE_MSG: &str = "支付状态不一致。";

pub const WALLET_CREATION_FAILED: u16 = PAYMENT_ERROR_BASE + 17;
pub const WALLET_CREATION_FAILED_MSG: &str = "钱包创建失败。";

pub const ADDRESS_GENERATION_FAILED: u16 = PAYMENT_ERROR_BASE + 18;
pub const ADDRESS_GENERATION_FAILED_MSG: &str = "地址生成失败。";

// File Errors (6000-6999)
pub const FILE_UPLOAD_FAILED: u16 = FILE_ERROR_BASE + 1;
pub const FILE_UPLOAD_FAILED_MSG: &str = "文件上传失败。";

pub const FILE_TOO_LARGE: u16 = FILE_ERROR_BASE + 2;
pub const FILE_TOO_LARGE_MSG: &str = "文件太大。";

pub const UNSUPPORTED_FILE_TYPE: u16 = FILE_ERROR_BASE + 3;
pub const UNSUPPORTED_FILE_TYPE_MSG: &str = "不支持的文件类型。";

pub const FILE_NOT_FOUND: u16 = FILE_ERROR_BASE + 4;
pub const FILE_NOT_FOUND_MSG: &str = "文件未找到。";

pub const FILE_READ_ERROR: u16 = FILE_ERROR_BASE + 5;
pub const FILE_READ_ERROR_MSG: &str = "文件读取失败。";

pub const FILE_WRITE_ERROR: u16 = FILE_ERROR_BASE + 6;
pub const FILE_WRITE_ERROR_MSG: &str = "文件写入失败。";

pub const DIRECTORY_CREATION_FAILED: u16 = FILE_ERROR_BASE + 7;
pub const DIRECTORY_CREATION_FAILED_MSG: &str = "目录创建失败。";

pub const FILE_DELETE_FAILED: u16 = FILE_ERROR_BASE + 8;
pub const FILE_DELETE_FAILED_MSG: &str = "文件删除失败。";

// External Service Errors (7000-7999)
pub const EXTERNAL_SERVICE_CALL_FAILED: u16 = EXTERNAL_SERVICE_ERROR_BASE + 1;
pub const EXTERNAL_SERVICE_CALL_FAILED_MSG: &str = "调用外部服务失败。";

pub const SMTP_CONNECT_FAILED: u16 = EXTERNAL_SERVICE_ERROR_BASE + 2;
pub const SMTP_CONNECT_FAILED_MSG: &str = "SMTP 服务器连接失败。";

pub const CREGIS_API_ERROR: u16 = EXTERNAL_SERVICE_ERROR_BASE + 3;
pub const CREGIS_API_ERROR_MSG: &str = "Cregis API 错误。";

pub const ANYCHAIN_API_ERROR: u16 = EXTERNAL_SERVICE_ERROR_BASE + 4;
pub const ANYCHAIN_API_ERROR_MSG: &str = "Anychain API 错误。";

pub const SMS_SERVICE_ERROR: u16 = EXTERNAL_SERVICE_ERROR_BASE + 5;
pub const SMS_SERVICE_ERROR_MSG: &str = "短信服务错误。";

pub const GEOLOCATION_SERVICE_ERROR: u16 = EXTERNAL_SERVICE_ERROR_BASE + 6;
pub const GEOLOCATION_SERVICE_ERROR_MSG: &str = "地理位置服务错误。";

pub const EXTERNAL_AUTH_ERROR: u16 = EXTERNAL_SERVICE_ERROR_BASE + 7;
pub const EXTERNAL_AUTH_ERROR_MSG: &str = "外部认证服务错误。";

// Other Errors (8000-8999)
pub const UNEXPECTED_ERROR: u16 = OTHER_ERROR_BASE + 1;
pub const UNEXPECTED_ERROR_MSG: &str = "发生意外错误。";

pub const RATE_LIMIT_EXCEEDED: u16 = OTHER_ERROR_BASE + 2;
pub const RATE_LIMIT_EXCEEDED_MSG: &str = "请求频率过高。";

pub const CSRF_TOKEN_INVALID: u16 = OTHER_ERROR_BASE + 3;
pub const CSRF_TOKEN_INVALID_MSG: &str = "无效的 CSRF Token。";

pub const INVALID_REQUEST: u16 = OTHER_ERROR_BASE + 4;
pub const INVALID_REQUEST_MSG: &str = "无效的请求。";

pub const TIMEOUT_ERROR: u16 = OTHER_ERROR_BASE + 5;
pub const TIMEOUT_ERROR_MSG: &str = "请求超时。";

pub const UNKNOWN_ROUTE: u16 = OTHER_ERROR_BASE + 6;
pub const UNKNOWN_ROUTE_MSG: &str = "未知路由。";

pub const METHOD_NOT_ALLOWED: u16 = OTHER_ERROR_BASE + 7;
pub const METHOD_NOT_ALLOWED_MSG: &str = "方法不允许。";

pub const PAYLOAD_TOO_LARGE: u16 = OTHER_ERROR_BASE + 8;
pub const PAYLOAD_TOO_LARGE_MSG: &str = "请求体过大。";

pub const MEDIATYPE_NOT_SUPPORTED: u16 = OTHER_ERROR_BASE + 9;
pub const MEDIATYPE_NOT_SUPPORTED_MSG: &str = "媒体类型不支持。";

pub const TOO_MANY_REQUESTS: u16 = OTHER_ERROR_BASE + 10;
pub const TOO_MANY_REQUESTS_MSG: &str = "请求过多。";

// Helper function to create an anyhow error with code and message
pub fn anyhow_error(code: u16, message: impl Into<String>) -> anyhow::Error {
    anyhow::anyhow!("Error Code: {}. Message: {}", code, message.into())
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("System Error: {message} (Code: {code})")]
    SystemError { code: u16, message: String },

    #[error("Data Error: {message} (Code: {code})")]
    DataError { code: u16, message: String },

    #[error("Business Error: {message} (Code: {code})")]
    BusinessError { code: u16, message: String },

    #[error("Authentication Error: {message} (Code: {code})")]
    AuthError { code: u16, message: String },

    #[error("Payment Error: {message} (Code: {code})")]
    PaymentError { code: u16, message: String },

    #[error("Other Error: {message} (Code: {code})")]
    OtherError { code: u16, message: String },

    #[error(transparent)]
    SeaOrmError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),

    #[error(transparent)]
    RedisError(#[from] redis::RedisError),

    #[error(transparent)]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error(transparent)]
    LettreError(#[from] lettre::error::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    // Add other necessary error types
}

impl AppError {
    pub fn code(&self) -> u16 {
        match self {
            AppError::SystemError { code, .. } => *code,
            AppError::DataError { code, .. } => *code,
            AppError::BusinessError { code, .. } => *code,
            AppError::AuthError { code, .. } => *code,
            AppError::PaymentError { code, .. } => *code,
            AppError::OtherError { code, .. } => *code,
            AppError::SeaOrmError(_) => 2000, // Example code for SeaORM errors
            AppError::ConfigError(_) => 1001, // Example code for config errors
            AppError::RedisError(_) => 1002,  // Example code for Redis errors
            AppError::BcryptError(_) => 1003, // Example code for Bcrypt errors
            AppError::LettreError(_) => 1004, // Example code for Lettre errors
            AppError::ReqwestError(_) => 1005, // Example code for Reqwest errors
            AppError::AnyhowError(_) => 1000, // Example code for generic anyhow errors
                                               // Assign codes for other error types
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::AuthError { .. } => StatusCode::UNAUTHORIZED,
            AppError::SystemError { .. }
            | AppError::ConfigError(_)
            | AppError::RedisError(_)
            | AppError::BcryptError(_)
            | AppError::LettreError(_)
            | AppError::ReqwestError(_)
            | AppError::AnyhowError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DataError { .. } => StatusCode::BAD_REQUEST, // Data validation or not found
            AppError::BusinessError { .. } | AppError::PaymentError { .. } => {
                StatusCode::BAD_REQUEST
            } // Business logic errors
            AppError::OtherError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SeaOrmError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
