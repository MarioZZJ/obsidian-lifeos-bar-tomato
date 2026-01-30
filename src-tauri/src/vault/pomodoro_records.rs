use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PomodoroRecord {
    pub id: String,
    pub date: String,
    pub start_time: u64,
    pub end_time: u64,
    pub duration: u32,
    pub mode: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pomodoro_index: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordsFile {
    pub version: u32,
    pub records: Vec<PomodoroRecord>,
}

impl RecordsFile {
    pub fn new() -> Self {
        Self {
            version: 1,
            records: Vec::new(),
        }
    }
}

pub fn get_records_file_path(vault_path: &str, device_hash: &str) -> String {
    let vault_name = Path::new(vault_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("vault");

    let storage_dir = Path::new(vault_path)
        .join(".obsidian")
        .join("plugins")
        .join("lifeos-pro")
        .join("storage");

    storage_dir
        .join(format!(
            "pomodoro-records-{}.{}.json",
            vault_name, device_hash
        ))
        .to_string_lossy()
        .to_string()
}

pub fn read_records(file_path: &str) -> Result<RecordsFile, String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Ok(RecordsFile::new());
    }

    let content =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read records: {}", e))?;

    serde_json::from_str(&content).map_err(|e| format!("Failed to parse records: {}", e))
}

pub fn write_records(file_path: &str, records: &RecordsFile) -> Result<(), String> {
    use fs2::FileExt;

    let path = Path::new(file_path);

    // Ensure storage directory exists
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create storage dir: {}", e))?;
    }

    let content = serde_json::to_string_pretty(records)
        .map_err(|e| format!("Failed to serialize records: {}", e))?;

    // Write atomically: write to temp file, then rename
    let tmp_path = format!("{}.tmp", file_path);
    let tmp_file = std::fs::File::create(&tmp_path)
        .map_err(|e| format!("Failed to create temp file: {}", e))?;

    // Lock the temp file
    tmp_file
        .lock_exclusive()
        .map_err(|e| format!("Failed to lock file: {}", e))?;

    std::fs::write(&tmp_path, &content)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;

    tmp_file
        .unlock()
        .map_err(|e| format!("Failed to unlock file: {}", e))?;

    std::fs::rename(&tmp_path, file_path)
        .map_err(|e| format!("Failed to rename temp file: {}", e))?;

    Ok(())
}

pub fn append_record(file_path: &str, record: PomodoroRecord) -> Result<(), String> {
    let mut records = read_records(file_path)?;
    records.records.push(record);
    write_records(file_path, &records)
}
