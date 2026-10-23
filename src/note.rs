use std::path::PathBuf;

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
