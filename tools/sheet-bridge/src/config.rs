//! Параметры подключения (аргументы CLI или переменные окружения, как в Data/.env).

use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct DbConfig {
    /// Адрес сервера: ws://, http://, mem:// …
    #[arg(long, env = "SURREALDB_HOST", default_value = "ws://127.0.0.1:8000")]
    pub url: String,
    #[arg(long, env = "SURREALDB_NAMESPACE", default_value = "gurps")]
    pub ns: String,
    #[arg(long, env = "SURREALDB_NAME", default_value = "gurps")]
    pub db: String,
    #[arg(long, env = "SURREALDB_USER", default_value = "root")]
    pub user: String,
    #[arg(long, env = "SURREALDB_PASSWORD", default_value = "root", hide_env_values = true)]
    pub pass: String,
}
