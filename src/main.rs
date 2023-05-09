#![allow(dead_code)]

use crate::states::InputState;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod context;
mod data;
mod errors;
mod pipeline;
mod states;

/// calc -- a complex compiler for a simple language
#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Run the optimizer
    #[arg(short, long)]
    optimize: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Tokenize the input and display the tokens
    #[clap(aliases = &["tok"])]
    Tokenize {
        /// Path to source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Parse the input and display the AST
    #[clap(aliases = &["ast"])]
    Parse {
        /// Path to source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Parse the input and print a formatted version of it
    #[clap(aliases = &["fmt", "formatter"])]
    Format {
        /// Path to source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Run the type checker on the input
    #[clap(aliases = &["chk", "checker"])]
    Check {
        /// Path to source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Run the interpreter on the input
    #[clap(aliases = &["interp"])]
    Run {
        /// Path to source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Source code
        #[arg(short)]
        code: Option<String>,
    },

    /// Compile input to LLVM IR
    #[clap(aliases = &["codegen"])]
    Llvm {
        /// Path to source file (or stdin if not present)
        path: Option<PathBuf>,

        /// Source code
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

            if cli.optimize {
                let checked = ast.check()?;
                let optimized = checked.optimize();
                optimized.ast.dump(&mut dump)?;
            } else {
                ast.raw_ast.dump(&mut dump)?;
            }
        }
        Commands::Format { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            let ast = tokenized_input.parse()?;

            if cli.optimize {
                let checked = ast.check()?;
                let optimized = checked.optimize();
                optimized.ast.format(&mut dump)?;
            } else {
                ast.raw_ast.format(&mut dump)?;
            }
        }
        Commands::Check { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            let parsed = tokenized_input.parse()?;
            let checked = parsed.check()?;

            if cli.optimize {
                let optimized = checked.optimize();
                optimized.ast.dump(&mut dump)?;
            } else {
                checked.ast.dump(&mut dump)?;
            }
        }
        Commands::Run { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            let parsed = tokenized_input.parse()?;
            let checked = parsed.check()?;

            if cli.optimize {
                let optimized = checked.optimize();
                optimized.interpret(&mut dump)?;
            } else {
                checked.interpret(&mut dump)?;
            }
        }
        Commands::Llvm { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;
            let parsed = tokenized_input.parse()?;
            let checked = parsed.check()?;

            if cli.optimize {
                let optimized = checked.optimize();
                optimized.llvm_codegen(&mut dump)?;
            } else {
                checked.llvm_codegen(&mut dump)?;
            }
        }
    }

    println!("{}", dump);

    Ok(())
}
