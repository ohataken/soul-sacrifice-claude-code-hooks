use serde::Deserialize;

#[derive(Deserialize)]
struct SessionEnd {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    reason: String,
}

pub fn run() {}
