use chrono::Local;
use config::Config;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{format::Writer, time::FormatTime, writer::MakeWriterExt},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

pub fn init(config: &Config) -> WorkerGuard {
    let max_level = if config.get_bool("logger.debug").unwrap_or(false) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    let directory = config
        .get_string("logger.directory")
        .unwrap_or("./log".to_string());
    let file_name_prefix = config
        .get_string("logger.file_name_prefix")
        .unwrap_or("pantheon".to_string());

    let file_appender = tracing_appender::rolling::daily(directory, file_name_prefix);
    let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);
    let layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        .with_timer(LocalTimer)
        .with_writer(non_blocking.with_max_level(max_level));
    tracing_subscriber::registry().with(layer).init();
    worker_guard
}
