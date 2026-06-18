#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use chrono::Local;

#[tauri::command]
fn capture_chat(clipboard_text: Option<String>) -> Result<String, String> {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let filename = format!("Quiki6_Chat_{}.md", timestamp);

    let chat_content = clipboard_text.unwrap_or_else(|| "[No text copied]".to_string());

    let content = format!(
        "# Quiki6 Chat Capture\n\n**Saved on:** {}\n\n---\n\n{}\n\n---\n\nSaved by Quiki6 Desktop • {}",
        now.format("%A, %B %d, %Y at %I:%M %p"),
        chat_content,
        now.format("%Y-%m-%d %H:%M:%S")
    );

    let local_dir = PathBuf::from(r"D:\AI CHAT BACKUPS - drive\Quiki6 Backups");
    let _ = fs::create_dir_all(&local_dir);
    let _ = fs::write(local_dir.join(&filename), &content);

    let cloud_dir = PathBuf::from(r"C:\Users\geoff\My Drive (geoffreyetd@gmail.com)\AI CHAT BACKUPS - cloud\Quiki6 Backups");
    let _ = fs::create_dir_all(&cloud_dir);
    let _ = fs::write(cloud_dir.join(&filename), &content);

    Ok(format!("Backup file: \"{}\"", filename))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture_chat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}