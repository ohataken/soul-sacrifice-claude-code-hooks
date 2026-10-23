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

struct Frontmatter {
    session_id: String,
    transcript_path: String,
    cwd: String,
    hook_event_name: String,
    prompt_id: String,
}

pub fn run() {
    let payload: Stop =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");

    let frontmatter = Frontmatter {
        session_id: payload.session_id,
        transcript_path: payload.transcript_path,
        cwd: payload.cwd,
        hook_event_name: payload.hook_event_name,
        prompt_id: payload.prompt_id,
    };
}
