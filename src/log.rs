use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{
    filter, fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer,
};

pub fn setup() -> WorkerGuard {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let console_layer = fmt::layer().pretty().with_writer(std::io::stderr);

    let file_appender = RollingFileAppender::builder()
        .max_log_files(5)
        .rotation(Rotation::DAILY)
        .filename_prefix("lette")
        .filename_suffix("log")
        .build("./logs")
        .expect("initializing rolling file appender failed");

    let (non_blocking_appender, file_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_target(true)
        .with_level(true)
        .with_ansi(false)
        .with_writer(non_blocking_appender)
        .with_filter(filter::LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    file_guard
}
