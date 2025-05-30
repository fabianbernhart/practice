use std::{env, process};

use minigrep::Config;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for '{}'", { &config.query });
    println!("In file '{}'", &config.file_path);

    println!();

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    Ok(())
}
