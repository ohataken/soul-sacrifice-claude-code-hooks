use serde::Deserialize;

#[derive(Deserialize)]
struct SessionStart {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    source: String,
}

struct Frontmatter {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    source: String,
}

pub fn run() {
    let _payload: SessionStart =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");
}
