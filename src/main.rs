use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Arg, Command};
use echo_python::{CheckResult, check_paths};

fn cli() -> Command {
    Command::new("echo-python")
        .version(env!("CARGO_PKG_VERSION"))
        .about("an extremely fast python linter with custom rules.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("check").arg(
                Arg::new("paths")
                    .default_value(".")
                    .num_args(1..)
                    .value_parser(clap::value_parser!(PathBuf)),
            ),
        )
}

fn main() -> ExitCode {
    let matches = cli().get_matches();
    let Some(("check", check_matches)) = matches.subcommand() else {
        return ExitCode::from(2);
    };
    let paths = paths_from_matches(check_matches);
    match check_paths(&paths) {
        Ok(result) => print_result(&result),
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(2)
        }
    }
}

fn paths_from_matches(matches: &clap::ArgMatches) -> Vec<PathBuf> {
    matches.get_many::<PathBuf>("paths").map_or_else(
        || vec![PathBuf::from(".")],
        |values| values.cloned().collect(),
    )
}

fn print_result(result: &CheckResult) -> ExitCode {
    for diagnostic in &result.diagnostics {
        println!("{diagnostic}");
    }
    let count = result.diagnostics.len();
    if count == 0 {
        println!("all checks passed!");
        return ExitCode::SUCCESS;
    }
    let noun = if count == 1 { "error" } else { "errors" };
    println!("found {count} {noun}.");
    ExitCode::from(1)
}
