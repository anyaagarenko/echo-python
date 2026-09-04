use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Arg, Command};
use echo_python::check_paths;

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

    let paths = check_matches.get_many::<PathBuf>("paths").map_or_else(
        || vec![PathBuf::from(".")],
        |values| values.cloned().collect(),
    );

    match check_paths(&paths) {
        Ok(result) => {
            for diagnostic in &result.diagnostics {
                println!("{diagnostic}");
            }

            let count = result.diagnostics.len();
            if count == 0 {
                println!("all checks passed!");
                ExitCode::SUCCESS
            } else {
                let noun = if count == 1 { "error" } else { "errors" };
                println!("found {count} {noun}.");
                ExitCode::from(1)
            }
        }
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(2)
        }
    }
}
