use tauri::State;
use ext_api::AppState;
use ext_core::Project;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to ETAB Extension.", name)
}

#[tauri::command]
pub async fn create_project(
    name: String,
    description: String,
    state: State<'_, AppState>,
) -> Result<Project, String> {
    state.create_project(name, description).await
}

#[tauri::command]
pub async fn get_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state.get_projects().await
}