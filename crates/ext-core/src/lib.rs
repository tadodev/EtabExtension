use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(
    export,
    export_to = concat!(env!("CARGO_MANIFEST_DIR"), "/../../packages/shared/src/types/")
)]
pub struct Project {
     #[ts(type = "string")]
    pub id: Uuid,

    pub name: String,

    pub description: String,

     #[ts(type = "string")]
    pub created_at: DateTime<Utc>,

     #[ts(type = "string")]
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: String, description: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project() {
        let project = Project::new(
            "Test Project".to_string(),
            "A test project".to_string()
        );
        assert_eq!(project.name, "Test Project");
    }
    
    #[test]
    fn test_export_typescript_bindings() {
        Project::export().expect("Failed to export Project");
    }
}