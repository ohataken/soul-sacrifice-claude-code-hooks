use crate::note;
use chrono::{DateTime, Local};
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
    timestamp: DateTime<Local>,
}

impl Frontmatter {
    fn render(&self) -> String {
        format!(
            "---\nsession_id: {}\ntranscript_path: {}\ncwd: {}\nhook_event_name: {}\nprompt_id: {}\ntimestamp: {}\n---\n",
            self.session_id,
            serde_json::json!(self.transcript_path),
            serde_json::json!(self.cwd),
            self.hook_event_name,
            self.prompt_id,
            self.timestamp.format("%Y-%m-%dT%H:%M:%S"),
        )
    }
}

pub fn run() {
    let now = Local::now();

    let payload: Stop =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");

    let frontmatter = Frontmatter {
        session_id: payload.session_id,
        transcript_path: payload.transcript_path,
        cwd: payload.cwd,
        hook_event_name: payload.hook_event_name,
        prompt_id: payload.prompt_id,
        timestamp: now,
    };

    note::write(
        &now,
        &frontmatter.session_id,
        format!(
            "{}\n{}\n",
            frontmatter.render(),
            payload.last_assistant_message
        ),
    );
}
