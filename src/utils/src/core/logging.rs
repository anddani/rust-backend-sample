use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_subscriber::fmt::format::{DefaultFields, FmtSpan, Format};
use tracing_subscriber::fmt::Subscriber;

pub async fn install() {
    let log_filter: String = std::env::var("RUST_LOG")
        .expect("RUST_LOG environment variable must be set");

    let subscriber: Subscriber<DefaultFields, Format, EnvFilter> = FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting global tracing subscriber failed")
}