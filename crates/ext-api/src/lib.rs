use ext_core::Project;
use ext_db::Database;
use ext_error::AppError;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
        }
    }

    pub async fn create_project(
        &self,
        name: String,
        description: String,
    ) -> Result<Project, String> {
        let project = Project::new(name, description);
        
        let db = self.db.lock().await;
        db.save_project(&project)
            .await
            .map_err(|e: AppError| e.to_string())?;

        Ok(project)
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>, String> {
        let db = self.db.lock().await;
        db.list_projects()
            .await
            .map_err(|e: AppError| e.to_string())
    }
}