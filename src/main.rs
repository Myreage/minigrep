use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::new(&args) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
    run(config);
}
