use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultTask {
    pub text: String,
    pub file_path: String,
    pub line_number: usize,
    pub project_tag: Option<String>,
    pub project_name: Option<String>,
}

pub fn scan_tasks(vault_path: &str) -> Result<Vec<VaultTask>, String> {
    let vault = Path::new(vault_path);
    let mut tasks = Vec::new();

    // Regex for uncompleted tasks: - [ ] or * [ ] or - [/] or * [/]
    let task_re =
        Regex::new(r"^[\s]*[-*]\s+\[([ /])\]\s+(.+)$").map_err(|e| format!("Regex error: {}", e))?;
    // Regex for project tag: #领域/项目名
    let tag_re =
        Regex::new(r"#([^/\s]+/[^\s]+)").map_err(|e| format!("Regex error: {}", e))?;

    let scan_dirs = vec![
        vault.join("1. 项目"),
        vault.join("0. 周期笔记"),
        vault.join("2. 领域"),
    ];

    for scan_dir in scan_dirs {
        if !scan_dir.exists() {
            continue;
        }

        for entry in WalkDir::new(&scan_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map_or(false, |ext| ext == "md")
                    && !e
                        .path()
                        .to_string_lossy()
                        .contains("Templates")
            })
        {
            let path = entry.path();
            let content = match std::fs::read_to_string(path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let relative_path = path
                .strip_prefix(vault)
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();

            for (line_num, line) in content.lines().enumerate() {
                if let Some(caps) = task_re.captures(line) {
                    let task_text = caps.get(2).map_or("", |m| m.as_str()).to_string();

                    // Extract project tag
                    let project_tag = tag_re
                        .captures(&task_text)
                        .and_then(|c| c.get(1))
                        .map(|m| m.as_str().to_string());

                    let project_name = project_tag.as_ref().map(|tag| {
                        tag.split('/').last().unwrap_or(tag).to_string()
                    });

                    tasks.push(VaultTask {
                        text: task_text,
                        file_path: relative_path.clone(),
                        line_number: line_num + 1,
                        project_tag,
                        project_name,
                    });
                }
            }
        }
    }

    Ok(tasks)
}
