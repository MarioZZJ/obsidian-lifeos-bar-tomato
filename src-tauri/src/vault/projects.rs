use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
    pub display_name: String,
    pub path: String,
    pub readme_path: String,
}

pub fn scan_projects(vault_path: &str) -> Result<Vec<Project>, String> {
    let projects_dir = Path::new(vault_path).join("1. 项目");
    if !projects_dir.exists() {
        return Ok(vec![]);
    }

    let mut projects = Vec::new();

    let entries = std::fs::read_dir(&projects_dir)
        .map_err(|e| format!("Failed to read projects dir: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read dir entry: {}", e))?;
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let folder_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        if folder_name.starts_with('.') {
            continue;
        }

        // Look for README file: {ProjectShortName}.README.md
        let short_name = folder_name
            .split('-')
            .last()
            .unwrap_or(&folder_name)
            .to_string();
        let readme_filename = format!("{}.README.md", short_name);
        let readme_path = path.join(&readme_filename);

        let relative_readme = if readme_path.exists() {
            format!("1. 项目/{}/{}", folder_name, readme_filename)
        } else {
            // Fallback: look for any .README.md file
            let mut found = String::new();
            if let Ok(files) = std::fs::read_dir(&path) {
                for f in files.flatten() {
                    let fname = f.file_name().to_string_lossy().to_string();
                    if fname.ends_with(".README.md") {
                        found = format!("1. 项目/{}/{}", folder_name, fname);
                        break;
                    }
                }
            }
            if found.is_empty() {
                format!("1. 项目/{}/", folder_name)
            } else {
                found
            }
        };

        projects.push(Project {
            name: folder_name.clone(),
            display_name: folder_name.clone(),
            path: format!("1. 项目/{}", entry.file_name().to_string_lossy()),
            readme_path: relative_readme,
        });
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(projects)
}
