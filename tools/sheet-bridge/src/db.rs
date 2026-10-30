//! Подключение к SurrealDB и выполнение сгенерированного SurrealQL.

use crate::config::DbConfig;
use crate::sql::Rec;
use anyhow::Context;
use serde_json::Value as J;
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

pub struct Db {
    pub conn: Surreal<Any>,
}

impl Db {
    pub async fn connect(cfg: &DbConfig) -> anyhow::Result<Self> {
        let conn = surrealdb::engine::any::connect(cfg.url.as_str())
            .await
            .with_context(|| format!("подключение к {}", cfg.url))?;
        conn.signin(surrealdb::opt::auth::Root {
            username: cfg.user.clone(),
            password: cfg.pass.clone(),
        })
        .await
        .context("вход (root)")?;
        conn.use_ns(&cfg.ns).use_db(&cfg.db).await.context("use ns/db")?;
        Ok(Db { conn })
    }

    /// Выполнить запрос, вернуть результаты всех операторов как JSON.
    pub async fn query(&self, sql: &str) -> anyhow::Result<Vec<J>> {
        let mut resp = self.conn.query(sql).await.context("отправка запроса")?;
        let n = resp.num_statements();
        let errs = resp.take_errors();
        if !errs.is_empty() {
            let mut v: Vec<_> = errs.into_iter().collect();
            v.sort_by_key(|(i, _)| *i);
            let msg = v.iter().map(|(i, e)| format!("#{i}: {e}")).collect::<Vec<_>>().join("\n");
            anyhow::bail!("ошибка SurrealDB:\n{msg}");
        }
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let v: surrealdb::types::Value = resp.take(i).with_context(|| format!("результат #{i}"))?;
            out.push(v.into_json_value());
        }
        Ok(out)
    }

    pub async fn one(&self, sql: &str) -> anyhow::Result<J> {
        Ok(self.query(sql).await?.pop().unwrap_or(J::Null))
    }

    /// Выполнить пакет записи в транзакции. `dry_run` — откатить (`CANCEL`):
    /// сервер полностью проверяет схему/ASSERT/события, но ничего не сохраняет.
    pub async fn write(&self, body: &str, dry_run: bool) -> anyhow::Result<()> {
        let end = if dry_run { "CANCEL" } else { "COMMIT" };
        let sql = format!("BEGIN TRANSACTION;\n{body}\n{end} TRANSACTION;");
        let mut resp = self.conn.query(sql).await.context("отправка запроса")?;
        let errs = resp.take_errors();
        let real: Vec<String> = {
            let mut v: Vec<_> = errs.into_iter().collect();
            v.sort_by_key(|(i, _)| *i);
            v.into_iter()
                .map(|(i, e)| (i, e.to_string()))
                .filter(|(_, e)| !(dry_run && is_cancel_noise(e)))
                .filter(|(_, e)| !e.contains("not executed due to a failed transaction"))
                .map(|(i, e)| format!("#{i}: {e}"))
                .collect()
        };
        if !real.is_empty() {
            anyhow::bail!("ошибка SurrealDB (транзакция отменена):\n{}", real.join("\n"));
        }
        Ok(())
    }

    /// Какие из ссылок отсутствуют в БД.
    pub async fn missing(&self, recs: &[Rec]) -> anyhow::Result<Vec<Rec>> {
        if recs.is_empty() {
            return Ok(vec![]);
        }
        let list = recs.iter().map(|r| format!("record::exists({r})")).collect::<Vec<_>>().join(", ");
        let v = self.one(&format!("RETURN [{list}];")).await?;
        let flags = v.as_array().cloned().unwrap_or_default();
        Ok(recs
            .iter()
            .zip(flags)
            .filter(|(_, f)| f.as_bool() != Some(true))
            .map(|(r, _)| r.clone())
            .collect())
    }
}

fn is_cancel_noise(e: &str) -> bool {
    e.contains("cancelled transaction") || e.contains("canceled transaction")
}
