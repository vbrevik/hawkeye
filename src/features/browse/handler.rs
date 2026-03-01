use crate::shared::state::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct BrowseQuery {
    pub path: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct BrowseResponse {
    pub path: String,
    pub parent: Option<String>,
    pub entries: Vec<String>,
    pub md_file_count: usize,
}

pub fn list_dir(dir: &std::path::Path) -> Result<BrowseResponse, String> {
    let path = dir
        .canonicalize()
        .map_err(|e| format!("Cannot resolve path: {e}"))?;

    let parent = path
        .parent()
        .map(|p| p.to_string_lossy().into_owned());

    let all: Vec<_> = std::fs::read_dir(&path)
        .map_err(|e| format!("Cannot read directory: {e}"))?
        .filter_map(|e| e.ok())
        .collect();

    let md_file_count = all.iter()
        .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .ends_with(".md")
        })
        .count();

    let mut entries: Vec<String> = all.iter()
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
        md_file_count,
    })
}

pub async fn handle_browse(
    State(_state): State<Arc<AppState>>,
    Query(q): Query<BrowseQuery>,
) -> Result<Json<BrowseResponse>, (StatusCode, String)> {
    let default_path = std::env::current_dir()
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "/".to_string());
    let path_str = q.path.unwrap_or(default_path);
    let dir = PathBuf::from(&path_str);

    if !dir.exists() {
        return Err((StatusCode::BAD_REQUEST, format!("Path does not exist: {path_str}")));
    }
    if !dir.is_dir() {
        return Err((StatusCode::BAD_REQUEST, format!("Not a directory: {path_str}")));
    }

    list_dir(&dir).map(Json).map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile;

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

    #[test]
    fn test_list_dir_counts_md_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.md"), "").unwrap();
        std::fs::write(dir.path().join("b.md"), "").unwrap();
        std::fs::write(dir.path().join("c.txt"), "").unwrap();
        std::fs::create_dir(dir.path().join("subdir")).unwrap();

        let result = list_dir(&dir.path().to_path_buf()).unwrap();
        assert_eq!(result.md_file_count, 2, "should count only .md files, not .txt or dirs");
        assert_eq!(result.entries, vec!["subdir"]);
    }

    #[test]
    fn test_list_dir_md_count_zero_when_no_md_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("readme.txt"), "").unwrap();

        let result = list_dir(&dir.path().to_path_buf()).unwrap();
        assert_eq!(result.md_file_count, 0);
    }
}
