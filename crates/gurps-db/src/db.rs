//! Подключение к SurrealDB и запросы, возвращающие JSON.
use crate::config::DbConfig;
use crate::error::{DbError, DbResult};
use serde_json::Value as J;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

#[derive(Clone)]
pub struct Db {
    pub conn: Surreal<Any>,
}

impl std::fmt::Debug for Db {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Db")
    }
}

/// Переменные запроса: имя -> строка.
pub type Vars = Vec<(&'static str, String)>;

impl Db {
    pub async fn connect(cfg: &DbConfig) -> DbResult<Self> {
        let err = |e: surrealdb::Error, what: &str| DbError::Connect {
            host: cfg.host.clone(),
            msg: format!("{what}: {e}"),
        };
        let conn = surrealdb::engine::any::connect(cfg.host.as_str())
            .await
            .map_err(|e| err(e, "соединение"))?;
        conn.signin(surrealdb::opt::auth::Root {
            username: cfg.user.clone(),
            password: cfg.password.clone(),
        })
        .await
        .map_err(|e| err(e, "вход (root)"))?;
        conn.use_ns(&cfg.namespace)
            .use_db(&cfg.database)
            .await
            .map_err(|e| err(e, "use ns/db"))?;
        Ok(Self { conn })
    }

    /// Выполнить запрос с переменными; результаты всех операторов как JSON.
    pub async fn query(&self, sql: &str, vars: Vars) -> DbResult<Vec<J>> {
        let mut q = self.conn.query(sql);
        for (k, v) in vars {
            q = q.bind((k, v));
        }
        let mut resp = q.await?;
        let errs = resp.take_errors();
        if !errs.is_empty() {
            let mut v: Vec<_> = errs.into_iter().collect();
            v.sort_by_key(|(i, _)| *i);
            let msg = v
                .iter()
                .map(|(i, e)| format!("#{i}: {e}"))
                .collect::<Vec<_>>()
                .join("; ");
            return Err(DbError::Query(msg));
        }
        let mut out = Vec::new();
        for i in 0..resp.num_statements() {
            let v: surrealdb::types::Value = resp.take(i)?;
            out.push(v.into_json_value());
        }
        Ok(out)
    }

    /// Результат последнего оператора.
    pub async fn one(&self, sql: &str, vars: Vars) -> DbResult<J> {
        Ok(self.query(sql, vars).await?.pop().unwrap_or(J::Null))
    }
}
