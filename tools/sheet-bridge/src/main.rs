//! CLI sheet-bridge.

use clap::{Parser, Subcommand};
use sheet_bridge::{catalog, character, config::DbConfig, db::Db, report::Report};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "sheet-bridge", about = "Конвертер gurps-sheet <-> SurrealDB")]
struct Cli {
    #[command(flatten)]
    db: DbConfig,
    /// Код возврата 2, если есть потери/пробелы.
    #[arg(long, global = true)]
    strict: bool,
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Плагины (TOML + .ftl) -> паки.
    ImportCatalog {
        paths: Vec<PathBuf>,
        #[arg(long, default_value = "official")]
        kind: String,
        #[arg(long)]
        abbrev: Option<String>,
        #[arg(long, default_value = "standard")]
        spell_system: String,
        #[arg(long, default_value = "regular")]
        spell_class: String,
        /// Проверка без записи (транзакция отменяется).
        #[arg(long)]
        dry_run: bool,
    },
    /// Паки -> плагины.
    ExportCatalog {
        #[arg(long = "pack")]
        packs: Vec<String>,
        #[arg(long, default_value = "out")]
        out: PathBuf,
    },
    /// Файл персонажа (.json) -> character + рёбра + custom-пак.
    ImportCharacter {
        file: PathBuf,
        #[arg(long)]
        id: Option<String>,
        #[arg(long, default_value = "TL3")]
        tl: String,
        #[arg(long, default_value = "regular")]
        spell_class: String,
        #[arg(long, default_value = "standard")]
        spell_system: String,
        #[arg(long)]
        dry_run: bool,
    },
    /// character -> файл персонажа.
    ExportCharacter {
        id: String,
        #[arg(long)]
        out: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match go(&cli).await {
        Ok(r) => {
            println!("{r}");
            if cli.strict && !r.is_clean() {
                std::process::exit(2);
            }
        }
        Err(e) => {
            eprintln!("ошибка: {e:#}");
            std::process::exit(1);
        }
    }
}

async fn go(cli: &Cli) -> anyhow::Result<Report> {
    let db = Db::connect(&cli.db).await?;
    match &cli.cmd {
        Cmd::ImportCatalog {
            paths,
            kind,
            abbrev,
            spell_system,
            spell_class,
            dry_run,
        } => {
            let o = catalog::import::CatalogOpts {
                kind: kind.clone(),
                abbrev: abbrev.clone(),
                spell_system: spell_system.clone(),
                spell_class: spell_class.clone(),
            };
            catalog::import::run(&db, paths, &o, *dry_run).await
        }
        Cmd::ExportCatalog { packs, out } => catalog::export::run(&db, packs, out).await,
        Cmd::ImportCharacter {
            file,
            id,
            tl,
            spell_class,
            spell_system,
            dry_run,
        } => {
            let o = character::import::CharOpts {
                id: id.clone(),
                tl: tl.clone(),
                spell_class: spell_class.clone(),
                spell_system: spell_system.clone(),
            };
            character::import::run(&db, file, &o, *dry_run).await
        }
        Cmd::ExportCharacter { id, out } => character::export::run(&db, id, out).await,
    }
}
