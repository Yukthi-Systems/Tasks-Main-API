use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

pub fn init() -> WorkerGuard {
    let (writer, guard) = tracing_appender::non_blocking(std::io::stdout());

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_writer(writer)
                .with_target(true)
                .with_thread_ids(true)
                .with_thread_names(true),
        )
        .init();

    guard
}


// ------- Tests ------- //


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_guard_is_created() {
        let (_writer, guard) =
            tracing_appender::non_blocking(std::io::stdout());

        // WorkerGuard doesn't expose an `is_guard()` API.
        // If it exists, it was successfully constructed.
        drop(guard);
    }

    #[test]
    fn test_env_filter_defaults_to_info() {
        // Remove RUST_LOG for this test so we exercise the fallback.
        unsafe {
            std::env::remove_var("RUST_LOG");
        }

        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        assert_eq!(filter.to_string(), "info");
    }

    #[test]
    fn test_env_filter_accepts_rust_log() {
        unsafe {
            std::env::set_var("RUST_LOG", "debug");
        }

        let filter = EnvFilter::try_from_default_env()
            .expect("RUST_LOG should produce a valid filter");

        assert_eq!(filter.to_string(), "debug");

        unsafe {
            std::env::remove_var("RUST_LOG");
        }
    }
}
