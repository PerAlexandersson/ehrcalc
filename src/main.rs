use clap::{CommandFactory, Parser};
use ehrcalc::cli::{run, Cli};

fn main() {
    let cli = Cli::parse();

    match run(cli) {
        Ok(Some(output)) => print!("{output}"),
        Ok(None) => {
            let mut command = Cli::command();
            command
                .print_long_help()
                .expect("writing help to stdout should succeed");
            println!();
        }
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("Try `ehrcalc --help` for usage.");
            std::process::exit(2);
        }
    }
}
