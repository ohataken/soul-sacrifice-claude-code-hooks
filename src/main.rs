fn main() {
    let subcommand = subcommand();
}

fn subcommand() -> String {
    std::env::args().nth(1).expect("subcommand is not given")
}
