use clap_verbosity_flag::Verbosity;
use rolling_file::{BasicRollingFileAppender, RollingConditionBasic};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter::EnvFilter, prelude::*};

use chatapp::cli;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Server { address, cert, key } => {
            let _log_guard = configure_logging(cli.verbosity, LogMode::Server);

            chatapp::server::run_server(&address, cert, key)
                .await
                .unwrap_or_else(|e| eprintln!("Server error: {e}"));
        }
        cli::Commands::Client { server } => {
            let _log_guard = configure_logging(cli.verbosity, LogMode::Client);

            chatapp::client::run_client(server)
                .await
                .unwrap_or_else(|e| eprintln!("Client error: {e}"));
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum LogMode {
    Client,
    Server,
}

fn configure_logging(verbosity: Verbosity, mode: LogMode) -> WorkerGuard {
    const GIB: u64 = 1024 * 1024 * 1024;

    std::fs::create_dir_all("./logs").expect("failed to create log directory");

    let file_name = match mode {
        LogMode::Client => "./logs/client.log",
        LogMode::Server => "./logs/server.log",
    };

    let file_appender =
        BasicRollingFileAppender::new(file_name, RollingConditionBasic::new().max_size(GIB), 2)
            .expect("failed to create rolling log file appender");

    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let console_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_filter(verbosity.tracing_level_filter());

    let file_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("chatapp=debug"))
        .expect("failed to create tracing env filter");

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(true)
        .with_current_span(true)
        .with_span_list(true)
        .with_filter(file_filter);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
}
