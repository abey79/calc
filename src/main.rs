#![allow(dead_code)]

use crate::states::InputState;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod context;
mod data;
mod errors;
mod pipeline;
mod states;

/// the dabbit compiler
#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Tokenize the input and display the tokens
    #[clap(aliases = &["tok"])]
    Tokenize {
        /// Path to wabbit source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Wabbit source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Parses the input and display the AST
    #[clap(aliases = &["ast"])]
    Parse {
        /// Path to wabbit source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Wabbit source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Parses the input and display the AST
    #[clap(aliases = &["fmt", "formatter"])]
    Format {
        /// Path to wabbit source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Wabbit source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Parses the input and display the AST
    #[clap(aliases = &["chk", "checker"])]
    Check {
        /// Path to wabbit source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Wabbit source code
        #[arg(short)]
        code: Option<String>,
    },
}

fn get_input(path: Option<PathBuf>, code: Option<String>) -> anyhow::Result<InputState> {
    if let Some(code) = code {
        Ok(InputState::from(code))
    } else if let Some(path) = path {
        Ok(InputState::from_file(path)?)
    } else {
        Ok(InputState::from_stdin()?)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut dump = String::new();

    match cli.command {
        Commands::Tokenize { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            tokenized_input.dump(&mut dump)?;
        }

        Commands::Parse { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            let ast = tokenized_input.parse()?;
            ast.ast.dump(&mut dump)?;
        }
        Commands::Format { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            let ast = tokenized_input.parse()?;
            ast.format(&mut dump)?;
        }
        Commands::Check { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            let parsed = tokenized_input.parse()?;
            let checked = parsed.check()?;
            checked.checked_ast.dump(&mut dump)?;
        }
    }

    println!("{}", dump);

    Ok(())
}
