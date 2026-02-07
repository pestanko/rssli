mod interactive;

use std::fs;
use std::sync::Once;

use clap::{Parser, Subcommand};

use crate::{cli::interactive::process_interactive, corelib::ProgramExitError, parser::Value, Runtime};

static INIT: Once = Once::new();

/**
 * Main function for the CLI
 * This will parse the arguments and process the input
 * It will return the result of the evaluation
 */
pub fn cli_main() -> anyhow::Result<()> {
    INIT.call_once(|| {
        env_logger::init();
    });

    let args = Args::parse();

    log::debug!("Parsed arguments: {:?}", args);

    match args.command {
        Commands::Eval { expression } => {
            let result = process_expression(&expression)?;
            log::info!("result for '{}': {:?}", expression, result);
        }
        Commands::File { file } => {
            let result = process_file(&file)?;
            log::info!("result for '{}': {:?}", file, result);
        }
        Commands::Interactive => {
            process_interactive()?;
        }
    };

    Ok(())
}

fn process_file(file: &str) -> anyhow::Result<Value> {
    log::info!("Processing file {}", file);
    let content = fs::read_to_string(file)?;
    let mut runtime = Runtime::new_default();
    match runtime.eval_file(file, &content) {
        Ok(result) => Ok(result),
        Err(err) => {
            if let Some(exit_err) = err.downcast_ref::<ProgramExitError>() {
                log::info!("Program exited with code {}", exit_err.code);
                std::process::exit(exit_err.code);
            }
            Err(err)
        }
    }
}

fn process_expression(expression: &str) -> anyhow::Result<Value> {
    log::info!("Processing expression {}", expression);
    let mut runtime = Runtime::new_default();
    match runtime.eval_string(expression) {
        Ok(result) => Ok(result),
        Err(err) => {
            if let Some(exit_err) = err.downcast_ref::<ProgramExitError>() {
                log::info!("Program exited with code {}", exit_err.code);
                std::process::exit(exit_err.code);
            }
            Err(err)
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Evaluate a string
    Eval {
        /// The expression to evaluate
        expression: String,
    },
    /// Evaluate a file
    File {
        /// The file to evaluate
        file: String,
    },
    /// Start an interactive session
    Interactive,
}
