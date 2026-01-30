use regex::Regex;
use std::path::Path;

/// Format minutes as `Xhr##` format (e.g., "0hr25", "1hr58", "3hr10")
pub fn format_time(total_minutes: u32) -> String {
    let hours = total_minutes / 60;
    let mins = total_minutes % 60;
    format!("{}hr{:02}", hours, mins)
}

/// Parse time string `Xhr##` back to total minutes
pub fn parse_time(s: &str) -> Option<u32> {
    let re = Regex::new(r"(\d+)hr(\d+)").ok()?;
    let caps = re.captures(s)?;
    let hours: u32 = caps.get(1)?.as_str().parse().ok()?;
    let mins: u32 = caps.get(2)?.as_str().parse().ok()?;
    Some(hours * 60 + mins)
}

/// Add two time values and return formatted result
pub fn time_add(existing: &str, added_minutes: u32) -> String {
    let existing_mins = parse_time(existing).unwrap_or(0);
    format_time(existing_mins + added_minutes)
}

/// Get daily note path for a given date
pub fn daily_note_path(vault_path: &str, date: &str) -> String {
    // date format: YYYY-MM-DD
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return String::new();
    }
    let year = parts[0];
    let month = parts[1];

    Path::new(vault_path)
        .join("0. 周期笔记")
        .join(year)
        .join("Daily")
        .join(month)
        .join(format!("{}.md", date))
        .to_string_lossy()
        .to_string()
}

/// Update the project time in the "项目列表" section of a daily note.
/// Returns Ok(true) if updated, Ok(false) if daily note doesn't exist.
pub fn update_project_time(
    vault_path: &str,
    date: &str,
    project_path: &str,
    display_name: &str,
    added_minutes: u32,
) -> Result<bool, String> {
    let note_path = daily_note_path(vault_path, date);
    let path = Path::new(&note_path);

    if !path.exists() {
        return Ok(false);
    }

    // Retry logic with mtime check
    for attempt in 0..3 {
        let mtime_before = std::fs::metadata(&note_path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?
            .modified()
            .map_err(|e| format!("Failed to read mtime: {}", e))?;

        let content = std::fs::read_to_string(&note_path)
            .map_err(|e| format!("Failed to read daily note: {}", e))?;

        let updated = update_project_section(&content, project_path, display_name, added_minutes)?;

        // Check mtime hasn't changed
        let mtime_after = std::fs::metadata(&note_path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?
            .modified()
            .map_err(|e| format!("Failed to read mtime: {}", e))?;

        if mtime_before != mtime_after && attempt < 2 {
            continue; // File changed, retry
        }

        std::fs::write(&note_path, updated)
            .map_err(|e| format!("Failed to write daily note: {}", e))?;

        return Ok(true);
    }

    Err("Failed to update daily note after 3 attempts".to_string())
}

fn update_project_section(
    content: &str,
    project_path: &str,
    display_name: &str,
    added_minutes: u32,
) -> Result<String, String> {
    let lines: Vec<&str> = content.lines().collect();

    // Find "## 项目列表" section
    let section_start = lines
        .iter()
        .position(|l| l.trim() == "## 项目列表")
        .ok_or("Could not find '## 项目列表' section")?;

    // Find next ## section
    let section_end = lines
        .iter()
        .enumerate()
        .skip(section_start + 1)
        .find(|(_, l)| l.starts_with("## "))
        .map(|(i, _)| i)
        .unwrap_or(lines.len());

    // Extract section lines
    let section_lines: Vec<&str> = lines[section_start + 1..section_end].to_vec();

    // Try to match existing entries in two formats:
    // 1. Short format: `N. [[ShortName.README|DisplayName]]` (existing entries)
    // 2. Long format: `N. [[projectPath|displayName]] XhrYY` (time entries)

    // Extract short name from project path (e.g., "DualBasic" from "1. 项目/科学研究-DualBasic/DualBasic.README.md")
    let short_name = project_path
        .split('/')
        .last()
        .and_then(|s| s.strip_suffix(".md"))
        .and_then(|s| s.strip_suffix(".README"))
        .unwrap_or("");

    // Regex for existing short format entry: `N. [[ShortName.README|DisplayName]]`
    let short_entry_re = Regex::new(&format!(
        r"^(\d+)\.\s+\[\[{}\.README\|{}\]\]$",
        regex::escape(short_name),
        regex::escape(display_name)
    ))
    .map_err(|e| format!("Regex error: {}", e))?;

    // Regex for existing time entry: `N. [[projectPath|displayName]] XhrYY`
    let time_entry_re = Regex::new(&format!(
        r"^(\d+)\.\s+\[\[{}[|]{}\]\]\s+(\d+hr\d+)",
        regex::escape(project_path),
        regex::escape(display_name)
    ))
    .map_err(|e| format!("Regex error: {}", e))?;

    let mut new_section_lines: Vec<String> = Vec::new();
    let mut found_existing = false;
    let mut total_line_idx: Option<usize> = None;
    let mut max_num = 0;

    for (i, line) in section_lines.iter().enumerate() {
        // Check for numbered list items
        if let Some(num) = line
            .trim()
            .split('.')
            .next()
            .and_then(|n| n.parse::<u32>().ok())
        {
            if num > max_num {
                max_num = num;
            }
        }

        // Check if this is an existing time entry for this project
        if let Some(caps) = time_entry_re.captures(line) {
            // Found existing time entry, update time
            let old_time = caps.get(2).unwrap().as_str();
            let new_time = time_add(old_time, added_minutes);
            let updated_line = line.replace(old_time, &new_time);
            new_section_lines.push(updated_line);
            found_existing = true;
        }
        // Check if this is an existing short format entry for this project
        else if let Some(caps) = short_entry_re.captures(line) {
            // Found existing short entry, add time to it
            let num = caps.get(1).unwrap().as_str();
            let time_str = format_time(added_minutes);
            let updated_line = format!(
                "{}. [[{}|{}]] {}",
                num, project_path, display_name, time_str
            );
            new_section_lines.push(updated_line);
            found_existing = true;
        } else {
            // Check if this is the total time line (standalone time like "1hr58")
            let trimmed = line.trim();
            if !trimmed.is_empty()
                && parse_time(trimmed).is_some()
                && !trimmed.starts_with('[')
                && !trimmed.starts_with('#')
                && !trimmed.contains("[[")
            {
                total_line_idx = Some(i);
            }
            new_section_lines.push(line.to_string());
        }
    }

    if !found_existing {
        // Add new project time entry before the total line
        let new_entry = format!(
            "{}. [[{}|{}]] {}",
            max_num + 1,
            project_path,
            display_name,
            format_time(added_minutes)
        );

        if let Some(total_idx) = total_line_idx {
            // Insert before total line, after any empty lines before total
            let mut insert_at = total_idx;
            // Go back past empty lines
            while insert_at > 0 && new_section_lines[insert_at - 1].trim().is_empty() {
                insert_at -= 1;
            }
            new_section_lines.insert(insert_at, new_entry);
            // total_line_idx shifted by 1
            total_line_idx = Some(total_idx + 1);
        } else {
            // No total line, append after last numbered item
            new_section_lines.push(new_entry);
        }
    }

    // Calculate new total from all project time entries
    let time_entry_re_all = Regex::new(r"\]\]\s+(\d+hr\d+)").unwrap();
    let mut total_mins: u32 = 0;
    for line in &new_section_lines {
        if let Some(caps) = time_entry_re_all.captures(line) {
            if let Some(mins) = parse_time(caps.get(1).unwrap().as_str()) {
                total_mins += mins;
            }
        }
    }

    // Update or add total line
    if let Some(total_idx) = total_line_idx {
        if total_idx < new_section_lines.len() {
            new_section_lines[total_idx] = format_time(total_mins);
        }
    } else {
        // Add empty line then total
        new_section_lines.push(String::new());
        new_section_lines.push(format_time(total_mins));
    }

    // Rebuild full content
    let mut result_lines: Vec<String> = Vec::new();
    for line in &lines[..=section_start] {
        result_lines.push(line.to_string());
    }
    for line in &new_section_lines {
        result_lines.push(line.to_string());
    }
    for line in &lines[section_end..] {
        result_lines.push(line.to_string());
    }

    Ok(result_lines.join("\n"))
}

/// Check the "使用番茄钟" habit checkbox in the daily note
pub fn check_pomodoro_habit(vault_path: &str, date: &str) -> Result<bool, String> {
    let note_path = daily_note_path(vault_path, date);
    let path = Path::new(&note_path);

    if !path.exists() {
        return Ok(false);
    }

    let content = std::fs::read_to_string(&note_path)
        .map_err(|e| format!("Failed to read daily note: {}", e))?;

    // Match both `* [ ] 使用番茄钟` and `- [ ] 使用番茄钟`
    let re = Regex::new(r"([-*])\s+\[ \]\s+使用番茄钟")
        .map_err(|e| format!("Regex error: {}", e))?;

    if let Some(m) = re.find(&content) {
        let replacement = format!(
            "{} [x] 使用番茄钟 ✅ {}",
            &content[m.start()..m.start() + 1], // preserve - or *
            date
        );
        let updated = content[..m.start()].to_string() + &replacement + &content[m.end()..];
        std::fs::write(&note_path, updated)
            .map_err(|e| format!("Failed to write daily note: {}", e))?;
        Ok(true)
    } else {
        // Already checked or not found
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time() {
        assert_eq!(format_time(25), "0hr25");
        assert_eq!(format_time(60), "1hr00");
        assert_eq!(format_time(118), "1hr58");
        assert_eq!(format_time(190), "3hr10");
    }

    #[test]
    fn test_parse_time() {
        assert_eq!(parse_time("0hr25"), Some(25));
        assert_eq!(parse_time("1hr58"), Some(118));
        assert_eq!(parse_time("3hr10"), Some(190));
    }

    #[test]
    fn test_time_add() {
        assert_eq!(time_add("0hr25", 25), "0hr50");
        assert_eq!(time_add("0hr50", 25), "1hr15");
        assert_eq!(time_add("1hr58", 13), "2hr11");
    }
}
