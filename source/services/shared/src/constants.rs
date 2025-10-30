// API Routes
pub const API_PREFIX: &str = "/api/v1";
pub const HEALTH_CHECK_PATH: &str = "/health";
pub const METRICS_PATH: &str = "/metrics";

// Auth constants
pub const BEARER_PREFIX: &str = "Bearer ";
pub const DEFAULT_TOKEN_EXPIRY: i64 = 3600; // 1 hour
pub const DEFAULT_REFRESH_TOKEN_EXPIRY: i64 = 86400 * 7; // 7 days

// Pagination
pub const DEFAULT_PAGE: u32 = 1;
pub const DEFAULT_LIMIT: u32 = 50;
pub const MAX_LIMIT: u32 = 100;

// Cache
pub const DEFAULT_CACHE_CAPACITY: usize = 1000;
pub const CACHE_TTL_SECONDS: u64 = 3600; // 1 hour

// Validation
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 128;
pub const MIN_USERNAME_LENGTH: usize = 3;
pub const MAX_USERNAME_LENGTH: usize = 50;

// Roles
pub const ROLE_USER: &str = "user";
pub const ROLE_ADMIN: &str = "admin";
pub const ROLE_MODERATOR: &str = "moderator";

// Permissions
pub const PERMISSION_READ: &str = "read";
pub const PERMISSION_WRITE: &str = "write";
pub const PERMISSION_DELETE: &str = "delete";
pub const PERMISSION_MANAGE_USERS: &str = "manage_users";
