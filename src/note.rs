use std::path::PathBuf;

fn sessions_path() -> PathBuf {
    let sessions_path = std::env::var("SOUL_SACRIFICE_SESSIONS_DIR")
        .expect("SOUL_SACRIFICE_SESSIONS_DIR is not set");

    PathBuf::from(sessions_path)
}
