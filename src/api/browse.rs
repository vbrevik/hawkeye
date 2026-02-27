use crate::api::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct BrowseQuery {
    pub path: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct BrowseResponse {
    pub path: String,
    pub parent: Option<String>,
    pub entries: Vec<String>,
}

pub fn list_dir(dir: &PathBuf) -> Result<BrowseResponse, String> {
    let path = dir
        .canonicalize()
        .map_err(|e| format!("Cannot resolve path: {e}"))?;

    let parent = path
        .parent()
        .filter(|p| *p != path.as_path()) // root's parent == itself
        .map(|p| p.to_string_lossy().into_owned());

    let mut entries: Vec<String> = std::fs::read_dir(&path)
        .map_err(|e| format!("Cannot read directory: {e}"))?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') { None } else { Some(name) }
        })
        .collect();
    entries.sort();

    Ok(BrowseResponse {
        path: path.to_string_lossy().into_owned(),
        parent,
        entries,
    })
}

pub async fn handle_browse(
    State(_state): State<Arc<AppState>>,
    Query(q): Query<BrowseQuery>,
) -> Result<Json<BrowseResponse>, (StatusCode, String)> {
    let default_home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    let path_str = q.path.unwrap_or(default_home);
    let dir = PathBuf::from(&path_str);

    list_dir(&dir).map(Json).map_err(|e| (StatusCode::BAD_REQUEST, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_list_dir_returns_only_dirs() {
        let home = env::var("HOME").unwrap();
        let result = list_dir(&std::path::PathBuf::from(&home)).unwrap();
        assert!(result.parent.is_some());
        let home_path = std::path::PathBuf::from(&home);
        for entry in &result.entries {
            assert!(home_path.join(entry).is_dir());
        }
    }

    #[test]
    fn test_list_dir_root_has_no_parent() {
        let result = list_dir(&std::path::PathBuf::from("/")).unwrap();
        assert_eq!(result.parent, None);
    }

    #[test]
    fn test_list_dir_hides_dotfiles() {
        let home = env::var("HOME").unwrap();
        let result = list_dir(&std::path::PathBuf::from(&home)).unwrap();
        for entry in &result.entries {
            assert!(!entry.starts_with('.'), "dotfile should be hidden: {entry}");
        }
    }
}
