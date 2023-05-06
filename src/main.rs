#![allow(dead_code)]

use crate::states::RawInput;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
}

//TODO: recreate this

// /// Newtype wrapper over a [`Token`] for the purpose of pretty printing the token stream.
// struct DisplayToken(Token);
//
// impl fmt::Display for DisplayToken {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let span = format!("{}", self.0.span);
//         let kind = match self.0.kind {
//             TokenKind::Name(ref s) => format!("{:10} {:?}", "Name", s),
//             TokenKind::Int(i) => format!("{:10} {}", "Int", i),
//             TokenKind::Float(fl) => format!("{:10} {:?}", "Float", fl),
//             TokenKind::Bool(b) => format!("{:10} {}", "Bool", b),
//             _ => format!("{:?}", self.0.kind),
//         };
//
//         write!(f, "{:15} {}", span, kind)
//     }
// }

fn get_input(path: Option<PathBuf>, code: Option<String>) -> anyhow::Result<RawInput> {
    if let Some(code) = code {
        Ok(RawInput::from(code))
    } else if let Some(path) = path {
        Ok(RawInput::from_file(path)?)
    } else {
        Ok(RawInput::from_stdin()?)
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Tokenize { path, code } => {
            let input = get_input(path, code)?;
            let tokenized_input = input.tokenize()?;

            for token in tokenized_input.tokens() {
                println!("{:?}", token.kind);
            }
        }
    }

    Ok(())
}
