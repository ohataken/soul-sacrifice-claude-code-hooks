use serde::Deserialize;

#[derive(Deserialize)]
struct SessionEnd {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    reason: String,
}

struct Frontmatter {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    reason: String,
}

pub fn run() {
    let _payload: SessionEnd =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");
}
