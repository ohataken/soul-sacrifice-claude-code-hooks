mod hooks;

fn main() {
    let subcommand = subcommand();

    match subcommand.as_str() {
        "session_start" => hooks::session_start::run(),
        _ => panic!("unknown subcommand: {subcommand}"),
    }
}

fn subcommand() -> String {
    std::env::args().nth(1).expect("subcommand is not given")
}
