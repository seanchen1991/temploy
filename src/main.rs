use temploy::cli_init;

fn main() {
    if let Err(err) = cli_init() {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
