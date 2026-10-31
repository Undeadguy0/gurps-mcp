//! Тестовый стенд: `surreal start memory` на свободном порту, схема и сиды через
//! `surrealkit sync` + `surrealkit seed` (каталог Data/database). Процесс гасится в Drop.
//!
//! Нет каталога схемы, `surreal` или `surrealkit` — `start` возвращает None (тест пропускается).
//! Переменные: GURPS_DB_DIR (каталог `database`), SURREAL_BIN, SURREALKIT_BIN.
use crate::{Db, DbConfig};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

pub struct Server {
    child: Child,
    pub cfg: DbConfig,
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// Каталог `database` (schema/seed): GURPS_DB_DIR или Data/database репозитория gurps-mcp.
pub fn db_dir() -> Option<PathBuf> {
    let p = std::env::var("GURPS_DB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../Data/database"));
    p.join("schema").is_dir().then_some(p)
}

fn bin(env: &str, name: &str) -> String {
    std::env::var(env).unwrap_or_else(|_| name.into())
}

fn surrealkit(dir: &Path, cfg: &DbConfig, cmd: &str) -> Result<(), String> {
    let folder = dir.parent().unwrap_or(dir);
    let out = Command::new(bin("SURREALKIT_BIN", "surrealkit"))
        .current_dir(folder)
        .arg(cmd)
        .args([
            "--host",
            &cfg.host,
            "--ns",
            &cfg.namespace,
            "--db",
            &cfg.database,
        ])
        .args(["--user", &cfg.user, "--pass", &cfg.password])
        .args(["--folder", &dir.display().to_string()])
        .output()
        .map_err(|e| format!("surrealkit {cmd}: {e}"))?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "surrealkit {cmd}: {}\n{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ))
    }
}

/// Поднять БД со схемой и сидами. None — окружение не готово (пропуск).
pub async fn start() -> Option<(Server, Db)> {
    let Some(dir) = db_dir() else {
        eprintln!("нет схемы (GURPS_DB_DIR), пропуск");
        return None;
    };
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .ok()?
        .local_addr()
        .ok()?
        .port();
    let child = match Command::new(bin("SURREAL_BIN", "surreal"))
        .args([
            "start", "memory", "--user", "root", "--pass", "root", "--log", "error",
        ])
        .args(["--bind", &format!("127.0.0.1:{port}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("surreal не запустился ({e}), пропуск");
            return None;
        }
    };
    let cfg = DbConfig {
        host: format!("ws://127.0.0.1:{port}"),
        namespace: "test".into(),
        database: "test".into(),
        user: "root".into(),
        password: "root".into(),
    };
    let server = Server {
        child,
        cfg: cfg.clone(),
    };
    let mut db = None;
    for _ in 0..100 {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        if let Ok(d) = Db::connect(&cfg).await {
            db = Some(d);
            break;
        }
    }
    let db = db.expect("surreal не поднялся");
    let kit_cfg = DbConfig {
        host: format!("http://127.0.0.1:{port}"),
        ..cfg
    };
    for cmd in ["sync", "seed"] {
        if let Err(e) = surrealkit(&dir, &kit_cfg, cmd) {
            if e.contains("program not found")
                || e.contains("cannot find")
                || e.contains("os error 2")
            {
                eprintln!("{e}; пропуск");
                return None;
            }
            panic!("{e}");
        }
    }
    Some((server, db))
}
