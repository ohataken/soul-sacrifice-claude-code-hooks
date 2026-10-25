use chrono::{DateTime, Local};
use std::io::Write;
use std::path::PathBuf;

pub fn write(now: &DateTime<Local>, session_id: &str, content: String) {
    let session_dir = session_dir(session_id);
    let name_format = name_format();

    let file_name = format!("{}.md", now.format(&name_format));
    let note_path = session_dir.join(file_name);

    std::fs::write(&note_path, content).expect("failed to write note");
}

pub fn create_session_dir(session_id: &str) {
    if let Err(error) = std::fs::create_dir(session_dir(session_id))
        && error.kind() != std::io::ErrorKind::AlreadyExists
    {
        panic!("failed to create session directory: {error}");
    }
}

pub fn create_base(session_id: &str, content: String) {
    let base_path = session_dir(session_id).join(format!("{session_id}.base"));

    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&base_path)
    {
        Ok(mut file) => file
            .write_all(content.as_bytes())
            .expect("failed to write base"),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(error) => panic!("failed to create base: {error}"),
    }
}

fn sessions_path() -> PathBuf {
    let sessions_path = std::env::var("SOUL_SACRIFICE_SESSIONS_DIR")
        .expect("SOUL_SACRIFICE_SESSIONS_DIR is not set");

    PathBuf::from(sessions_path)
}

fn name_format() -> String {
    std::env::args()
        .nth(2)
        .unwrap_or_else(|| "%Y%m%d%H%M".to_string())
}

fn session_dir(session_id: &str) -> PathBuf {
    sessions_path().join(session_id)
}
