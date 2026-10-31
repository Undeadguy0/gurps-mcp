//! Точка входа: логи в stderr, MCP по stdio.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    gurps_mcp::run_stdio().await
}
