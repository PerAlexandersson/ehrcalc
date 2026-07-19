use ehrcalc_mcp::EhrcalcServer;
use rmcp::{transport::stdio, ServiceExt};

const BIN_NAME: &str = "ehrcalc-mcp";

#[derive(Debug, Eq, PartialEq)]
enum StartupMode {
    Serve,
    Help,
    Version,
}

fn parse_args(args: &[String]) -> Result<StartupMode, String> {
    match args {
        [_program] => Ok(StartupMode::Serve),
        [_program, flag] if flag == "--help" || flag == "-h" => Ok(StartupMode::Help),
        [_program, flag] if flag == "--version" || flag == "-V" => Ok(StartupMode::Version),
        [_program, flag] => Err(format!("unknown option: {flag}")),
        [_program, flag, ..] => Err(format!("unexpected extra arguments after: {flag}")),
        [] => Ok(StartupMode::Serve),
    }
}

fn help_text() -> String {
    format!(
        "{BIN_NAME} {}\n\nUsage:\n  {BIN_NAME}\n  {BIN_NAME} --help\n  {BIN_NAME} --version\n\nRun without arguments from an MCP client. The server uses stdio transport and exposes exact Ehrcalc tools only.\n",
        env!("CARGO_PKG_VERSION")
    )
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    match parse_args(&args) {
        Ok(StartupMode::Help) => {
            print!("{}", help_text());
            return Ok(());
        }
        Ok(StartupMode::Version) => {
            println!("{BIN_NAME} {}", env!("CARGO_PKG_VERSION"));
            return Ok(());
        }
        Ok(StartupMode::Serve) => {}
        Err(error) => {
            eprintln!("error: {error}");
            eprintln!("Try `{BIN_NAME} --help` for usage.");
            std::process::exit(2);
        }
    }

    let service = EhrcalcServer::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_local_server_flags() {
        assert_eq!(parse_args(&["ehrcalc-mcp".to_string()]), Ok(StartupMode::Serve));
        assert_eq!(parse_args(&["ehrcalc-mcp".to_string(), "--help".to_string()]), Ok(StartupMode::Help));
        assert!(parse_args(&["ehrcalc-mcp".to_string(), "--bad".to_string()]).is_err());
    }
}
