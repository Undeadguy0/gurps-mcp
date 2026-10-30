//! Отчёт о потерях: что не перенеслось в каком направлении.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dir {
    /// gurps-sheet -> БД
    Import,
    /// БД -> gurps-sheet
    Export,
}

#[derive(Debug, Default, Clone)]
pub struct Report {
    /// (направление, тема) -> список записей
    pub gaps: BTreeMap<(Dir, String), Vec<String>>,
    pub info: Vec<String>,
}

impl Report {
    pub fn gap(&mut self, dir: Dir, topic: impl Into<String>, what: impl Into<String>) {
        let v = self.gaps.entry((dir, topic.into())).or_default();
        let w = what.into();
        if !v.contains(&w) {
            v.push(w);
        }
    }
    pub fn info(&mut self, s: impl Into<String>) {
        self.info.push(s.into());
    }
    pub fn is_clean(&self) -> bool {
        self.gaps.is_empty()
    }
    pub fn topics(&self, dir: Dir) -> Vec<String> {
        self.gaps.keys().filter(|(d, _)| *d == dir).map(|(_, t)| t.clone()).collect()
    }
    pub fn has(&self, dir: Dir, topic: &str) -> bool {
        self.gaps.contains_key(&(dir, topic.to_string()))
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in &self.info {
            writeln!(f, "{i}")?;
        }
        if self.gaps.is_empty() {
            return writeln!(f, "Потерь нет.");
        }
        writeln!(f, "Потери/пробелы ({}):", self.gaps.len())?;
        for ((d, topic), items) in &self.gaps {
            let dir = match d {
                Dir::Import => "лист→БД",
                Dir::Export => "БД→лист",
            };
            let shown: Vec<&str> = items.iter().take(8).map(String::as_str).collect();
            let more = if items.len() > 8 {
                format!(" … (+{})", items.len() - 8)
            } else {
                String::new()
            };
            writeln!(f, "  [{dir}] {topic}: {}{more}", shown.join(", "))?;
        }
        Ok(())
    }
}
