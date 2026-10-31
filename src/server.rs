//! MCP-обработчик: маршрутизатор инструментов и сведения о сервере.
use crate::db::Db;
use crate::tools::{
    self,
    character::GetCharacterParams,
    compute::{CheckPrereqsParams, ComputeCharacterParams},
    entry::GetEntryParams,
    search::SearchParams,
};
use rmcp::handler::server::{router::tool::ToolRouter, wrapper::Parameters};
use rmcp::model::{CallToolResult, Implementation, ServerCapabilities, ServerConfig};
use rmcp::{ErrorData, ServerHandler, tool, tool_handler, tool_router};

#[derive(Clone)]
pub struct GurpsServer {
    db: Db,
    tool_router: ToolRouter<Self>,
}

fn ok(v: serde_json::Value) -> Result<CallToolResult, ErrorData> {
    // structured() кладёт JSON и в structured_content, и текстом в content
    Ok(CallToolResult::structured(v))
}

#[tool_router]
impl GurpsServer {
    pub fn new(db: Db) -> Self {
        Self {
            db,
            tool_router: Self::tool_router(),
        }
    }

    /// Полнотекстовый поиск по названиям/описаниям (таблица translation).
    #[tool(annotations(read_only_hint = true))]
    pub async fn search(
        &self,
        Parameters(p): Parameters<SearchParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ok(tools::search::search(&self.db, p).await?)
    }

    /// Каталожная запись по sid: действующая с учётом приоритета паков (или из указанного пака) + все переводы.
    #[tool(annotations(read_only_hint = true))]
    pub async fn get_entry(
        &self,
        Parameters(p): Parameters<GetEntryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ok(tools::entry::get_entry(&self.db, p).await?)
    }

    /// Список паков (источников данных) по убыванию приоритета.
    #[tool(annotations(read_only_hint = true))]
    pub async fn list_packs(&self) -> Result<CallToolResult, ErrorData> {
        ok(tools::packs::list_packs(&self.db).await?)
    }

    /// Персонаж со всеми рёбрами: навыки, техники, заклинания, черты, предметы.
    #[tool(annotations(read_only_hint = true))]
    pub async fn get_character(
        &self,
        Parameters(p): Parameters<GetCharacterParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ok(tools::character::get_character(&self.db, p).await?)
    }

    /// Расчёт персонажа: атрибуты и производные, уровни навыков/техник/заклинаний, атаки,
    /// сумма очков против журнала, нарушения пререквизитов, недостающие константы справочников.
    #[tool(annotations(read_only_hint = true))]
    pub async fn compute_character(
        &self,
        Parameters(p): Parameters<ComputeCharacterParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ok(tools::compute::compute_character(&self.db, p).await?)
    }

    /// Проверка пререквизитов записи каталога (действующей по приоритету паков) для персонажа.
    #[tool(annotations(read_only_hint = true))]
    pub async fn check_prereqs(
        &self,
        Parameters(p): Parameters<CheckPrereqsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        ok(tools::compute::check_prereqs(&self.db, p).await?)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for GurpsServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new(
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions(
                "База GURPS 4e (SurrealDB). id каталога — ['pack','sid']; тексты — в translation.",
            )
    }
}
