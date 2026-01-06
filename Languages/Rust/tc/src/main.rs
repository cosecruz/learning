use tc::cli::Cli;
use tc::error::TCError;

fn main() -> Result<(), TCError> {
    let args: Vec<String> = std::env::args().collect();
    let args_str: Vec<&str> = args.iter().map(String::as_str).collect();

    if args_str.len() < 2 {
        return Err(TCError::CliError);
    }

    let cli = Cli::cmd(&args_str[1..])?;
    cli.command.run()
}
