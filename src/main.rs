fn main() {
    let subcommand = subcommand();

    match subcommand.as_str() {
        _ => panic!("unknown subcommand: {subcommand}"),
    }
}

fn subcommand() -> String {
    std::env::args().nth(1).expect("subcommand is not given")
}
