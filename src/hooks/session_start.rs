use crate::note;
use chrono::{DateTime, Local};
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
    timestamp: DateTime<Local>,
}

impl Frontmatter {
    fn render(&self) -> String {
        format!(
            "---\nsession_id: {}\ntranscript_path: {}\ncwd: {}\nhook_event_name: {}\nsource: {}\ntimestamp: {}\n---\n",
            self.session_id,
            serde_json::json!(self.transcript_path),
            serde_json::json!(self.cwd),
            self.hook_event_name,
            self.source,
            self.timestamp.format("%Y-%m-%dT%H:%M:%S"),
        )
    }
}

struct Base {
    session_id: String,
}

impl Base {
    fn render(&self) -> String {
        let mut base = String::new();

        base += "views:\n";
        base += "  - type: table\n";
        base += "    name: Session\n";

        base
    }
}

pub fn run() {
    let now = Local::now();

    let payload: SessionStart =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");

    let frontmatter = Frontmatter {
        session_id: payload.session_id,
        transcript_path: payload.transcript_path,
        cwd: payload.cwd,
        hook_event_name: payload.hook_event_name,
        source: payload.source,
        timestamp: now,
    };

    note::create_session_dir(&frontmatter.session_id);

    note::write(&now, &frontmatter.session_id, frontmatter.render());
}
