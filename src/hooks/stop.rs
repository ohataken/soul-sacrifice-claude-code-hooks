use serde::Deserialize;

#[derive(Deserialize)]
struct Stop {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    prompt_id: String,
    last_assistant_message: String,
}

pub fn run() {
    let _payload: Stop =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");
}
