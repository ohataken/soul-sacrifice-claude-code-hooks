mod hooks;

fn main() {
    let subcommand = subcommand();

    match subcommand.as_str() {
        "session_start" => hooks::session_start::run(),
        "session_end" => hooks::session_end::run(),
        "user_prompt_submit" => hooks::user_prompt_submit::run(),
        "stop" => hooks::stop::run(),
        _ => panic!("unknown subcommand: {subcommand}"),
    }
}

fn subcommand() -> String {
    std::env::args().nth(1).expect("subcommand is not given")
}
