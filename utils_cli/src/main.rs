use std::process;
use utils::{cat::cat, cli::cli, ls::ls};

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli();

    match matches.subcommand() {
        Some(("ls", sub_matches)) => {
            let path = sub_matches.get_one::<String>("DIR").unwrap();
            let full = sub_matches.get_one::<bool>("full").unwrap();
            let entries = ls(path, *full)?;

            for entry in entries {
                println!("{}", entry);
            }
        }
        Some(("cat", sub_matches)) => {
            let path = sub_matches.get_one::<String>("FILE").unwrap();
            let content = cat(path)?;
            println!("{content}")
        }
        _ => (),
    };
    Ok(())
}
