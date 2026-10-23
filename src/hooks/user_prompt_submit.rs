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

pub fn run() {}
