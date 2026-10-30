//! Плагин gurps-sheet: `plugin.toml` + `data/*.toml` + `locales/<lang>/*.ftl`.

use super::defs::DataFile;
use super::ftl::{parse_ftl, LocEntry};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manifest {
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub depends: Vec<String>,
}

/// lang -> key -> перевод (позже загруженные файлы перекрывают более ранние).
pub type Locales = BTreeMap<String, BTreeMap<String, LocEntry>>;

#[derive(Debug, Clone)]
pub struct Plugin {
    pub manifest: Manifest,
    pub data: DataFile,
    pub locales: Locales,
    pub path: PathBuf,
}

impl Plugin {
    /// Языковой пак: нет данных, только переводы.
    pub fn is_locale_only(&self) -> bool {
        self.data.is_empty()
    }
}

fn sorted_entries(dir: &Path, pred: impl Fn(&Path) -> bool) -> anyhow::Result<Vec<PathBuf>> {
    let mut v: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| anyhow::anyhow!("{}: {e}", dir.display()))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| pred(p))
        .collect();
    v.sort();
    Ok(v)
}

fn read(p: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(p).map_err(|e| anyhow::anyhow!("{}: {e}", p.display()))
}

pub fn load_plugin(dir: &Path) -> anyhow::Result<Plugin> {
    let mp = dir.join("plugin.toml");
    let manifest: Manifest = toml::from_str(&read(&mp)?).map_err(|e| anyhow::anyhow!("{}: {e}", mp.display()))?;
    let mut data = DataFile::default();
    let dd = dir.join("data");
    if dd.is_dir() {
        for f in sorted_entries(&dd, |p| p.extension().is_some_and(|x| x == "toml"))? {
            let df: DataFile = toml::from_str(&read(&f)?).map_err(|e| anyhow::anyhow!("{}: {e}", f.display()))?;
            data.extend(df);
        }
    }
    let mut locales = Locales::new();
    let ld = dir.join("locales");
    if ld.is_dir() {
        for lang_dir in sorted_entries(&ld, |p| p.is_dir())? {
            let lang = lang_dir.file_name().unwrap().to_string_lossy().to_string();
            let map = locales.entry(lang).or_default();
            for f in sorted_entries(&lang_dir, |p| p.extension().is_some_and(|x| x == "ftl"))? {
                map.extend(parse_ftl(&read(&f)?, &f.display().to_string())?);
            }
        }
    }
    Ok(Plugin {
        manifest,
        data,
        locales,
        path: dir.to_path_buf(),
    })
}

/// Загружает плагин или все плагины каталога (порядок — топологический по depends).
pub fn load_plugins(paths: &[PathBuf]) -> anyhow::Result<Vec<Plugin>> {
    let mut all = Vec::new();
    for p in paths {
        if p.join("plugin.toml").is_file() {
            all.push(load_plugin(p)?);
        } else {
            for d in sorted_entries(p, |d| d.join("plugin.toml").is_file())? {
                all.push(load_plugin(&d)?);
            }
        }
    }
    let order = resolve_order(&all.iter().map(|p| p.manifest.clone()).collect::<Vec<_>>())?;
    let mut by_id: BTreeMap<String, Plugin> = all.into_iter().map(|p| (p.manifest.id.clone(), p)).collect();
    Ok(order.into_iter().filter_map(|id| by_id.remove(&id)).collect())
}

/// Топологическая сортировка; зависимости вне набора допускаются (считаются уже в БД).
pub fn resolve_order(ms: &[Manifest]) -> anyhow::Result<Vec<String>> {
    let map: BTreeMap<&str, &Manifest> = ms.iter().map(|m| (m.id.as_str(), m)).collect();
    if map.len() != ms.len() {
        anyhow::bail!("повторяющийся id плагина");
    }
    let mut done = std::collections::BTreeSet::new();
    let mut out = Vec::new();
    fn visit<'a>(
        id: &'a str,
        map: &BTreeMap<&'a str, &'a Manifest>,
        stack: &mut Vec<&'a str>,
        done: &mut std::collections::BTreeSet<&'a str>,
        out: &mut Vec<String>,
    ) -> anyhow::Result<()> {
        if done.contains(id) || !map.contains_key(id) {
            return Ok(());
        }
        if stack.contains(&id) {
            anyhow::bail!("цикл зависимостей: {} -> {id}", stack.join(" -> "));
        }
        stack.push(id);
        let mut deps: Vec<&str> = map[id].depends.iter().map(String::as_str).collect();
        deps.sort();
        for d in deps {
            visit(d, map, stack, done, out)?;
        }
        stack.pop();
        done.insert(id);
        out.push(id.to_string());
        Ok(())
    }
    for id in map.keys().copied().collect::<Vec<_>>() {
        visit(id, &map, &mut Vec::new(), &mut done, &mut out)?;
    }
    Ok(out)
}

/// Пишет плагин в `root/<id>/`: plugin.toml, data/<вид>.toml, locales/<lang>/<id>.ftl.
pub fn write_plugin(root: &Path, p: &Plugin) -> anyhow::Result<PathBuf> {
    let dir = root.join(&p.manifest.id);
    std::fs::create_dir_all(dir.join("data"))?;
    std::fs::write(dir.join("plugin.toml"), toml::to_string(&p.manifest)?)?;
    let d = &p.data;
    let parts: [(&str, DataFile); 6] = [
        (
            "traits",
            DataFile {
                traits: d.traits.clone(),
                ..Default::default()
            },
        ),
        (
            "skills",
            DataFile {
                skills: d.skills.clone(),
                techniques: d.techniques.clone(),
                ..Default::default()
            },
        ),
        (
            "spells",
            DataFile {
                spells: d.spells.clone(),
                ..Default::default()
            },
        ),
        (
            "equipment",
            DataFile {
                equipment: d.equipment.clone(),
                ..Default::default()
            },
        ),
        (
            "weapons",
            DataFile {
                weapons: d.weapons.clone(),
                ..Default::default()
            },
        ),
        (
            "modifiers",
            DataFile {
                modifiers: d.modifiers.clone(),
                ..Default::default()
            },
        ),
    ];
    for (name, df) in parts {
        if !df.is_empty() {
            let header = "# Экспортировано sheet-bridge из SurrealDB. Не редактировать вручную —\n# правьте БД и экспортируйте заново.\n\n";
            std::fs::write(
                dir.join("data").join(format!("{name}.toml")),
                format!("{header}{}", toml::to_string_pretty(&df)?),
            )?;
        }
    }
    for (lang, entries) in &p.locales {
        if entries.is_empty() {
            continue;
        }
        let ld = dir.join("locales").join(lang);
        std::fs::create_dir_all(&ld)?;
        let list: Vec<(String, LocEntry)> = entries.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        let text = super::ftl::write_ftl(
            &format!(
                "Переводы ({lang}) пака {}; сгенерировано sheet-bridge из таблицы translation.",
                p.manifest.id
            ),
            &list,
        );
        std::fs::write(ld.join(format!("{}.ftl", p.manifest.id)), text)?;
    }
    Ok(dir)
}
