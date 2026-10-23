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

impl Frontmatter {
    fn render(&self) -> String {
        format!(
            "---\nsession_id: {}\ntranscript_path: {}\ncwd: {}\nhook_event_name: {}\nprompt_id: {}\n---\n",
            self.session_id,
            serde_json::json!(self.transcript_path),
            serde_json::json!(self.cwd),
            self.hook_event_name,
            self.prompt_id,
        )
    }
}

pub fn run() {
    let payload: UserPromptSubmit =
        serde_json::from_reader(std::io::stdin()).expect("failed to parse JSON from stdin");

    let frontmatter = Frontmatter {
        session_id: payload.session_id,
        transcript_path: payload.transcript_path,
        cwd: payload.cwd,
        hook_event_name: payload.hook_event_name,
        prompt_id: payload.prompt_id,
    };

    print!("{}", frontmatter.render());
}
