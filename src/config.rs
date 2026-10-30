//! Параметры подключения из окружения (те же имена, что у sheet-bridge и Data/.env).

#[derive(Debug, Clone)]
pub struct DbConfig {
    /// ws://, http://, mem:// …
    pub host: String,
    pub namespace: String,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        let get = |k: &str, d: &str| std::env::var(k).unwrap_or_else(|_| d.to_string());
        Self {
            host: get("SURREALDB_HOST", "ws://127.0.0.1:8000"),
            namespace: get("SURREALDB_NAMESPACE", "gurps"),
            database: get("SURREALDB_NAME", "gurps"),
            user: get("SURREALDB_USER", "root"),
            password: get("SURREALDB_PASSWORD", "root"),
        }
    }
}
