use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use std::env::var as env_var;
use std::time::Duration;


pub struct PgSettings {
    pub url: String,
    pub conn_timeout: u64,
    pub max_pool_size: usize,
    pub wait_timeout: u64,
    pub new_connection_timeout: u64,
    pub recycle_timeout: u64,
    pub warm_pool: bool,
    pub warm_pool_size: usize,
    pub ssl_accept_invalid_certs: bool,
    pub ssl_accept_invalid_hostnames: bool,
    pub ssl_root_cert_path: Option<String>,
}


pub struct MokaSettings {
    pub cache_size: u64,
    pub expiration_time: Duration,
}


pub struct AppSettings {
    pub pg_settings: PgSettings,
    pub cache_settings: MokaSettings,
    pub redis_url: String,
}


#[derive(Clone)]
pub struct RmqSettings {
    pub domain: String,
    pub auth_token: String, // Base64 encoded string of "username:password"
    pub virtual_host: String,
    pub exchange_name: String,
    pub routing_key: String,
}


pub struct ApiSettings {
    pub allowed_origins: Vec<String>,
    pub self_api_key: String,
    // Additional API settings can be added here as needed
}


// ------- Implementations ------- //


impl PgSettings {
    fn from_env() -> Self {
        let url = env_var("POSTGRES_DB_URL").expect("POSTGRES_DB_URL must be set");
        let conn_timeout = env_var("PG_CONN_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_CONN_TIMEOUT must be a positive integer of type u64");
        let max_pool_size = env_var("PG_POOL_MAX_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_MAX_SIZE must be a positive integer of type usize");
        let wait_timeout = env_var("PG_POOL_WAIT_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_WAIT_TIMEOUT must be a positive integer of type u64");
        let new_connection_timeout = env_var("PG_POOL_NEW_CONNECTION_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_NEW_CONNECTION_TIMEOUT must be a positive integer of type u64");
        let recycle_timeout = env_var("PG_POOL_RECYCLE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_RECYCLE_TIMEOUT must be a positive integer of type u64");
        let warm_pool = env_var("PG_POOL_WARM_POOL").expect("PG_POOL_WARM_POOL must be set as true or false");
        let warm_pool = match warm_pool.to_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => panic!("PG_POOL_WARM_POOL must be set as true or false"),
        };
        let warm_pool_size = env_var("PG_POOL_WARM_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("PG_POOL_WARM_POOL_SIZE must be a positive integer of type usize");
        let ssl_accept_invalid_certs = env_var("PG_SSL_ACCEPT_INVALID_CERTS")
            .ok()
            .map(|s| s.to_lowercase())
            .map(|s| match s.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("PG_SSL_ACCEPT_INVALID_CERTS must be true or false"),
            })
            .unwrap_or(false);
        let ssl_accept_invalid_hostnames = env_var("PG_SSL_ACCEPT_INVALID_HOSTNAMES")
            .ok()
            .map(|s| s.to_lowercase())
            .map(|s| match s.as_str() {
                "true" => true,
                "false" => false,
                _ => panic!("PG_SSL_ACCEPT_INVALID_HOSTNAMES must be true or false"),
            })
            .unwrap_or(false);
        let ssl_root_cert_path = env_var("PG_SSL_ROOT_CERT_PATH")
            .ok()
            .filter(|s| !s.trim().is_empty());

        // Warm pool size can not go above 128 (if warm pool is enabled)
        if warm_pool_size > max_pool_size {
            panic!("PG_POOL_WARM_POOL_SIZE must be at most PG_POOL_MAX_SIZE, it can not go more than {}", max_pool_size);
        }
        if warm_pool && warm_pool_size > 128 {
            panic!("PG_POOL_WARM_POOL_SIZE must be at most 128, and the optimal size is 64");
        }

        PgSettings {
            url,
            conn_timeout,
            max_pool_size,
            wait_timeout,
            new_connection_timeout,
            recycle_timeout,
            warm_pool,
            warm_pool_size,
            ssl_accept_invalid_certs,
            ssl_accept_invalid_hostnames,
            ssl_root_cert_path,
        }
    }
}


impl MokaSettings {
    fn from_env() -> Self {
        let cache_size = env_var("CACHE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("CACHE_SIZE must be a positive integer of type u64");
        let expiration_time = env_var("CACHE_EXPIRATION_TIME")
            .ok()
            .and_then(|s| s.parse().ok())
            .expect("CACHE_EXPIRATION_TIME must be a positive integer of type u64");

        MokaSettings {
            cache_size,
            expiration_time: Duration::from_secs(expiration_time),
        }
    }
}


impl AppSettings {
    pub fn from_env() -> Self {
        let redis_url = env_var("REDIS_URL").expect("REDIS_URL must be set");

        AppSettings {
            pg_settings: PgSettings::from_env(),
            cache_settings: MokaSettings::from_env(),
            redis_url,
        }
    }
}


impl RmqSettings {
    pub fn from_env() -> Self {
        let domain = env_var("RABBITMQ_DOMAIN").expect("RABBITMQ_DOMAIN must be set");
        let user_name = env_var("RABBITMQ_USER_NAME").expect("RABBITMQ_USER_NAME must be set");
        let password = env_var("RABBITMQ_PASSWORD").expect("RABBITMQ_PASSWORD must be set");
        let virtual_host = env_var("RABBITMQ_VIRTUAL_HOST").expect("RABBITMQ_VIRTUAL_HOST must be set");
        let exchange_name = env_var("RABBITMQ_EXCHANGE_NAME").expect("RABBITMQ_EXCHANGE_NAME must be set");
        let routing_key = env_var("RABBITMQ_ROUTING_KEY").expect("RABBITMQ_ROUTING_KEY must be set");

        let auth_token = BASE64_STANDARD.encode(format!("{}:{}", user_name, password));

        RmqSettings {
            domain,
            auth_token,
            virtual_host,
            exchange_name,
            routing_key,
        }
    }
}


impl ApiSettings {
    pub fn from_env() -> Self {
        let allowed_origins = env_var("ALLOWED_ORIGINS")
            .ok()
            .map(|s| {
                s.split(',')
                    .map(|entry| entry.trim())
                    .filter(|entry| !entry.is_empty())
                    .map(|entry| entry.to_string())
                    .collect::<Vec<String>>()
            })
            .filter(|items| !items.is_empty())
            .expect("ALLOWED_ORIGINS must be set as a comma-separated list of allowed origins or '*' for allowing all origins");

        let self_api_key = env_var("SELF_API_KEY").expect("SELF_API_KEY must be set");

        ApiSettings {
            allowed_origins,
            self_api_key
        }
    }
}


// ------- Tests ------- //


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    // Prevent tests in this module from modifying environment
    // variables at the same time.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    const ENV_VARS: &[&str] = &[
        "POSTGRES_DB_URL",
        "PG_CONN_TIMEOUT",
        "PG_POOL_MAX_SIZE",
        "PG_POOL_WAIT_TIMEOUT",
        "PG_POOL_NEW_CONNECTION_TIMEOUT",
        "PG_POOL_RECYCLE_TIMEOUT",
        "PG_POOL_WARM_POOL",
        "PG_POOL_WARM_POOL_SIZE",
        "PG_SSL_ACCEPT_INVALID_CERTS",
        "PG_SSL_ACCEPT_INVALID_HOSTNAMES",
        "PG_SSL_ROOT_CERT_PATH",
        "CACHE_SIZE",
        "CACHE_EXPIRATION_TIME",
        "REDIS_URL",
        "RABBITMQ_DOMAIN",
        "RABBITMQ_USER_NAME",
        "RABBITMQ_PASSWORD",
        "RABBITMQ_VIRTUAL_HOST",
        "RABBITMQ_EXCHANGE_NAME",
        "RABBITMQ_ROUTING_KEY",
        "ALLOWED_ORIGINS",
        "SELF_API_KEY",
    ];

    struct TestEnv {
        _lock: MutexGuard<'static, ()>,
        original: Vec<(&'static str, Option<String>)>,
    }

    impl TestEnv {
        fn new() -> Self {
            let lock = ENV_LOCK.lock().unwrap();

            let original = ENV_VARS
                .iter()
                .map(|&key| (key, env_var(key).ok()))
                .collect();

            let test_env = Self {
                _lock: lock,
                original,
            };

            test_env.set_defaults();
            test_env
        }

        fn set(&self, key: &str, value: &str) {
            // Required because this crate uses Rust edition 2024.
            unsafe {
                std::env::set_var(key, value);
            }
        }

        fn remove(&self, key: &str) {
            unsafe {
                std::env::remove_var(key);
            }
        }

        fn set_defaults(&self) {
            self.set("POSTGRES_DB_URL", "postgres://test:test@localhost/test");
            self.set("PG_CONN_TIMEOUT", "10");
            self.set("PG_POOL_MAX_SIZE", "32");
            self.set("PG_POOL_WAIT_TIMEOUT", "5");
            self.set("PG_POOL_NEW_CONNECTION_TIMEOUT", "10");
            self.set("PG_POOL_RECYCLE_TIMEOUT", "30");
            self.set("PG_POOL_WARM_POOL", "true");
            self.set("PG_POOL_WARM_POOL_SIZE", "4");

            // Remove SSL-related environment variables to ensure default behavior
            self.remove("PG_SSL_ACCEPT_INVALID_CERTS");
            self.remove("PG_SSL_ACCEPT_INVALID_HOSTNAMES");
            self.remove("PG_SSL_ROOT_CERT_PATH");

            self.set("CACHE_SIZE", "1000");
            self.set("CACHE_EXPIRATION_TIME", "300");

            self.set("REDIS_URL", "redis://localhost:6379");

            self.set("RABBITMQ_DOMAIN", "localhost:5672");
            self.set("RABBITMQ_USER_NAME", "test_user");
            self.set("RABBITMQ_PASSWORD", "test_password");
            self.set("RABBITMQ_VIRTUAL_HOST", "/");
            self.set("RABBITMQ_EXCHANGE_NAME", "test_exchange");
            self.set("RABBITMQ_ROUTING_KEY", "test.routing.key");

            self.set(
                "ALLOWED_ORIGINS",
                "http://localhost:3000, http://127.0.0.1:3000",
            );
            self.set("SELF_API_KEY", "test-api-key");
        }
    }

    impl Drop for TestEnv {
        fn drop(&mut self) {
            for (key, value) in &self.original {
                unsafe {
                    match value {
                        Some(value) => std::env::set_var(key, value),
                        None => std::env::remove_var(key),
                    }
                }
            }
        }
    }

    #[test]
    fn pg_settings_parse_valid_values() {
        let _env = TestEnv::new();

        let settings = PgSettings::from_env();

        assert_eq!(settings.url, "postgres://test:test@localhost/test");
        assert_eq!(settings.conn_timeout, 10);
        assert_eq!(settings.max_pool_size, 32);
        assert_eq!(settings.wait_timeout, 5);
        assert_eq!(settings.new_connection_timeout, 10);
        assert_eq!(settings.recycle_timeout, 30);
        assert!(settings.warm_pool);
        assert_eq!(settings.warm_pool_size, 4);
    }

    #[test]
    fn pg_ssl_options_default_to_false() {
        let _env = TestEnv::new();

        let settings = PgSettings::from_env();

        assert!(!settings.ssl_accept_invalid_certs);
        assert!(!settings.ssl_accept_invalid_hostnames);
        assert_eq!(settings.ssl_root_cert_path, None);
    }

    #[test]
    fn pg_ssl_options_parse_values() {
        let env = TestEnv::new();

        env.set("PG_SSL_ACCEPT_INVALID_CERTS", "TRUE");
        env.set("PG_SSL_ACCEPT_INVALID_HOSTNAMES", "true");
        env.set("PG_SSL_ROOT_CERT_PATH", "/tmp/root.crt");

        let settings = PgSettings::from_env();

        assert!(settings.ssl_accept_invalid_certs);
        assert!(settings.ssl_accept_invalid_hostnames);
        assert_eq!(
            settings.ssl_root_cert_path.as_deref(),
            Some("/tmp/root.crt")
        );
    }

    #[test]
    fn pg_warm_pool_cannot_exceed_max_pool_size() {
        let env = TestEnv::new();

        env.set("PG_POOL_MAX_SIZE", "4");
        env.set("PG_POOL_WARM_POOL_SIZE", "5");

        let result = std::panic::catch_unwind(PgSettings::from_env);

        assert!(result.is_err());
    }

    #[test]
    fn pg_warm_pool_cannot_exceed_128() {
        let env = TestEnv::new();

        env.set("PG_POOL_MAX_SIZE", "256");
        env.set("PG_POOL_WARM_POOL_SIZE", "129");

        let result = std::panic::catch_unwind(PgSettings::from_env);

        assert!(result.is_err());
    }

    #[test]
    fn pg_invalid_boolean_is_rejected() {
        let env = TestEnv::new();

        env.set("PG_POOL_WARM_POOL", "yes");

        let result = std::panic::catch_unwind(PgSettings::from_env);

        assert!(result.is_err());
    }

    #[test]
    fn moka_settings_parse_values() {
        let _env = TestEnv::new();

        let settings = MokaSettings::from_env();

        assert_eq!(settings.cache_size, 1000);
        assert_eq!(settings.expiration_time, Duration::from_secs(300));
    }

    #[test]
    fn app_settings_parse_values() {
        let _env = TestEnv::new();

        let settings = AppSettings::from_env();

        assert_eq!(settings.redis_url, "redis://localhost:6379");
        assert_eq!(settings.pg_settings.max_pool_size, 32);
        assert_eq!(settings.cache_settings.cache_size, 1000);
    }

    #[test]
    fn rmq_settings_encode_credentials() {
        let _env = TestEnv::new();

        let settings = RmqSettings::from_env();

        let expected = BASE64_STANDARD.encode("test_user:test_password");

        assert_eq!(settings.auth_token, expected);
        assert_eq!(settings.domain, "localhost:5672");
        assert_eq!(settings.virtual_host, "/");
        assert_eq!(settings.exchange_name, "test_exchange");
        assert_eq!(settings.routing_key, "test.routing.key");
    }

    #[test]
    fn api_settings_parse_allowed_origins() {
        let _env = TestEnv::new();

        let settings = ApiSettings::from_env();

        assert_eq!(
            settings.allowed_origins,
            vec![
                "http://localhost:3000",
                "http://127.0.0.1:3000",
            ]
        );
        assert_eq!(settings.self_api_key, "test-api-key");
    }

    #[test]
    fn api_settings_reject_empty_allowed_origins() {
        let env = TestEnv::new();

        env.set("ALLOWED_ORIGINS", " , , ");

        let result = std::panic::catch_unwind(ApiSettings::from_env);

        assert!(result.is_err());
    }
}
