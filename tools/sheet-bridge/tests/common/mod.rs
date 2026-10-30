//! Общий помощник тестов: поднимает `surreal start memory` на свободном порту и грузит схему + сиды-справочники.
#![allow(dead_code)]

use sheet_bridge::config::DbConfig;
use sheet_bridge::db::Db;
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

pub fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Каталог gurps-sheet (только чтение): env GURPS_SHEET_DIR или соседний /workspace/gurps-sheet.
pub fn sheet_dir() -> Option<PathBuf> {
    let p = std::env::var("GURPS_SHEET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo().join("../gurps-sheet"));
    p.join("plugins").is_dir().then_some(p)
}

fn surreal_bin() -> String {
    if let Ok(b) = std::env::var("SURREAL_BIN") {
        return b;
    }
    let home = std::env::var("HOME").unwrap_or_default();
    let p = Path::new(&home).join(".local/bin/surreal");
    if p.exists() {
        p.display().to_string()
    } else {
        "surreal".into()
    }
}

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port()
}

fn sql_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut es: Vec<_> = std::fs::read_dir(dir).unwrap().flatten().map(|e| e.path()).collect();
    es.sort();
    for p in es {
        if p.is_dir() {
            sql_files(&p, out);
        } else if p.extension().is_some_and(|e| e == "surql") {
            out.push(p);
        }
    }
}

pub async fn start() -> (Server, Db) {
    let port = free_port();
    let child = Command::new(surreal_bin())
        .args([
            "start",
            "memory",
            "--user",
            "root",
            "--pass",
            "root",
            "--bind",
            &format!("127.0.0.1:{port}"),
            "--log",
            "error",
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("не запустился surreal (SURREAL_BIN?)");
    let cfg = DbConfig {
        url: format!("ws://127.0.0.1:{port}"),
        ns: "test".into(),
        db: "test".into(),
        user: "root".into(),
        pass: "root".into(),
    };
    let mut db = None;
    for _ in 0..100 {
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        if let Ok(d) = Db::connect(&cfg).await {
            db = Some(d);
            break;
        }
    }
    let db = db.expect("surreal не ответил");
    let base = repo().join("Data/database");
    let mut files = Vec::new();
    sql_files(&base.join("schema"), &mut files);
    // справочные сиды (00..09: паки, книги, условия ранений, refs, планы тела) — по порядку имён
    let mut seeds: Vec<PathBuf> = std::fs::read_dir(base.join("seed"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy();
            n.ends_with(".surql") && n.as_bytes()[0] == b'0'
        })
        .collect();
    seeds.sort();
    files.extend(seeds);
    files.push(Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/colleges.surql"));
    for f in files {
        let body = std::fs::read_to_string(&f).unwrap();
        db.query(&body).await.unwrap_or_else(|e| panic!("{}: {e:#}", f.display()));
    }
    (Server { child, cfg }, db)
}

pub fn tmp(name: &str) -> PathBuf {
    let p = std::env::temp_dir().join(format!("sheet-bridge-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

use serde_json::Value as J;

/// Нормализация для сравнения: убирает ключи `drop`, null, пустые строки/массивы/объекты;
/// массивы объектов с `id` сортируются по id.
pub fn norm(v: &J, drop: &[&str]) -> J {
    match v {
        J::Object(o) => {
            let mut m = serde_json::Map::new();
            let mut keys: Vec<_> = o.keys().collect();
            keys.sort();
            for k in keys {
                if drop.contains(&k.as_str()) {
                    continue;
                }
                let x = norm(&o[k], drop);
                let empty = matches!(&x, J::Null | J::Bool(false))
                    || x.as_str() == Some("")
                    || x.as_array().is_some_and(|a| a.is_empty())
                    || x.as_object().is_some_and(|a| a.is_empty());
                if !empty {
                    m.insert(k.clone(), x);
                }
            }
            J::Object(m)
        }
        J::Array(a) => {
            let mut a: Vec<J> = a.iter().map(|x| norm(x, drop)).collect();
            if a.iter().all(|x| x.get("id").is_some()) {
                a.sort_by_key(|x| x["id"].to_string());
            }
            J::Array(a)
        }
        J::Number(n) => n.as_f64().map(|f| serde_json::json!(f)).unwrap_or(v.clone()),
        _ => v.clone(),
    }
}

/// Первое расхождение (путь) — для читаемых сообщений.
pub fn diff(a: &J, b: &J, path: &str) -> Option<String> {
    match (a, b) {
        (J::Object(x), J::Object(y)) => {
            for k in x.keys().chain(y.keys()) {
                let p = format!("{path}.{k}");
                match (x.get(k), y.get(k)) {
                    (Some(l), Some(r)) => {
                        if let Some(d) = diff(l, r, &p) {
                            return Some(d);
                        }
                    }
                    (l, r) => return Some(format!("{p}: {l:?} != {r:?}")),
                }
            }
            None
        }
        (J::Array(x), J::Array(y)) if x.len() == y.len() => x
            .iter()
            .zip(y)
            .enumerate()
            .find_map(|(i, (l, r))| diff(l, r, &format!("{path}[{i}]"))),
        _ if a == b => None,
        _ => Some(format!("{path}: {a} != {b}")),
    }
}

pub fn catalog_opts() -> sheet_bridge::catalog::import::CatalogOpts {
    sheet_bridge::catalog::import::CatalogOpts {
        kind: "official".into(),
        abbrev: None,
        spell_system: "standard".into(),
        spell_class: "regular".into(),
    }
}

pub fn char_opts(id: Option<&str>) -> sheet_bridge::character::import::CharOpts {
    sheet_bridge::character::import::CharOpts {
        id: id.map(str::to_string),
        tl: "TL3".into(),
        spell_class: "regular".into(),
        spell_system: "standard".into(),
    }
}

pub fn read_toml(p: &Path) -> J {
    let t: toml::Value = toml::from_str(&std::fs::read_to_string(p).unwrap()).unwrap();
    serde_json::to_value(t).unwrap()
}

/// skill_bonus на навыки вне образца теряется при импорте (отчёт features.skill_bonus).
pub fn drop_skill_bonus(v: &mut J) {
    match v {
        J::Object(o) => {
            if let Some(fs) = o.get_mut("features").and_then(|f| f.as_array_mut()) {
                fs.retain(|f| f["type"] != "skill_bonus");
            }
            o.values_mut().for_each(drop_skill_bonus);
        }
        J::Array(a) => a.iter_mut().for_each(drop_skill_bonus),
        _ => {}
    }
}
