use clap::{arg, ArgMatches, Command};

pub fn cli() -> ArgMatches {
    Command::new("utils_cli")
        .about("A cli for basic unix utils")
        .subcommand_required(true)
        .arg_required_else_help(true)
        // .allow_external_subcommands(true)
        .subcommand(
            Command::new("ls")
                .about("List directory contents")
                .arg(arg!(--full "Show full details"))
                .arg(arg!(<DIR> "Directory to list").default_missing_value("."))
                .arg_required_else_help(true),
        )
        .get_matches()
}
