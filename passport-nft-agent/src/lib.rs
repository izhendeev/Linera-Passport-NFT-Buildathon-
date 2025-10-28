pub mod chain_client;
pub mod config;
pub mod llm;
pub mod scoring;
pub mod updater;

use anyhow::{anyhow, Result};
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_logging(level: &str) -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", level);
    }

    fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .with_ansi(false)
        .try_init()
        .map_err(|err| anyhow!("failed to initialize tracing subscriber: {err}"))?;

    Ok(())
}
pub mod scoring_llm;
