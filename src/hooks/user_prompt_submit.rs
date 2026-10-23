use serde::Deserialize;

#[derive(Deserialize)]
struct UserPromptSubmit {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    prompt_id: String,
    prompt: String,
}

struct Frontmatter {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    prompt_id: String,
}

pub fn run() {
    let _payload: UserPromptSubmit =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");
}
