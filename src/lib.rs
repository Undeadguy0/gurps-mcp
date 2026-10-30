//! MCP-сервер для базы GURPS (SurrealDB).
pub mod config;
pub mod db;
pub mod error;
pub mod server;
pub mod tools;

use rmcp::ServiceExt;

/// Запуск сервера на stdio. stdout занят протоколом, поэтому логи — только в stderr.
pub async fn run_stdio() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();
    let cfg = config::DbConfig::from_env();
    tracing::info!(host = %cfg.host, ns = %cfg.namespace, db = %cfg.database, "подключение к SurrealDB");
    let db = db::Db::connect(&cfg).await?;
    let service = server::GurpsServer::new(db)
        .serve(rmcp::transport::stdio())
        .await?;
    tracing::info!("MCP-сервер запущен (stdio)");
    service.waiting().await?;
    Ok(())
}
