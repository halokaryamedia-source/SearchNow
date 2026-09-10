use super::error::CommandError;
use searchnow_core::{
    app_runtime::SearchNowBackendRuntime,
    catalog::{CatalogPage, CatalogRequest},
};
use tauri::State;

#[tauri::command]
pub async fn query_catalog(
    state: State<'_, SearchNowBackendRuntime>,
    request: CatalogRequest,
) -> Result<CatalogPage, CommandError> {
    let runtime = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || runtime.query_catalog(&request))
        .await
        .map_err(|error| {
            CommandError::new(
                "catalog_query_task_failed",
                format!("Catalog query task failed: {error}"),
            )
        })?
        .map_err(|error| CommandError::new(error.code, error.message))
}
