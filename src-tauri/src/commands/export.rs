use tauri::command;
use crate::github::Repository;
use std::path::PathBuf;
use std::fs::File;
use std::io::Write;
use chrono::Local;
use anyhow::Result;

#[command]
pub async fn export_repositories(repos: Vec<Repository>, format: String) -> Result<String, String> {
    match perform_export(repos, &format) {
        Ok(path) => Ok(path),
        Err(e) => Err(e.to_string()),
    }
}

fn perform_export(repos: Vec<Repository>, format: &str) -> Result<String> {
    let mut path = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("gittop_export_{}.{}", timestamp, format);
    path.push(&filename);

    match format {
        "csv" => {
            let mut wtr = csv::Writer::from_path(&path)?;
            for repo in repos {
                wtr.serialize(repo)?;
            }
            wtr.flush()?;
        },
        "json" => {
            let file = File::create(&path)?;
            serde_json::to_writer_pretty(file, &repos)?;
        },
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
    }

    Ok(path.to_string_lossy().to_string())
}
