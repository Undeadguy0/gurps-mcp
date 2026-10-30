//! Помощник тестов: `surreal start memory` на свободном порту + схема и сиды из Data/database.
//! Каталог схемы: env GURPS_DB_DIR (по умолчанию ../mcp-gurps/Data/database). Нет схемы или surreal — тест пропускается.
#![allow(dead_code)]

use gurps_mcp::config::DbConfig;
use gurps_mcp::db::Db;
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

pub fn db_dir() -> Option<PathBuf> {
    let p = std::env::var("GURPS_DB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../mcp-gurps/Data/database")
        });
    p.join("schema").is_dir().then_some(p)
}

fn surreal_bin() -> String {
    if let Ok(b) = std::env::var("SURREAL_BIN") {
        return b;
    }
    let p = Path::new(&std::env::var("HOME").unwrap_or_default()).join(".local/bin/surreal");
    if p.exists() {
        p.display().to_string()
    } else {
        "surreal".into()
    }
}

fn sql_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut es: Vec<_> = std::fs::read_dir(dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .collect();
    es.sort();
    for p in es {
        if p.is_dir() {
            sql_files(&p, out);
        } else if p.extension().is_some_and(|e| e == "surql") {
            out.push(p);
        }
    }
}

/// Поднять БД со схемой и сидами. None — окружение не готово (пропуск).
pub async fn start() -> Option<(Server, Db)> {
    let Some(dir) = db_dir() else {
        eprintln!("нет схемы (GURPS_DB_DIR), пропуск");
        return None;
    };
    let port = std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let child = match Command::new(surreal_bin())
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
    let mut files = vec![];
    sql_files(&dir.join("schema"), &mut files);
    sql_files(&dir.join("seed"), &mut files);
    for f in files {
        let sql = std::fs::read_to_string(&f).unwrap();
        db.query(&sql, vec![])
            .await
            .unwrap_or_else(|e| panic!("{}: {e}", f.display()));
    }
    Some((server, db))
}
