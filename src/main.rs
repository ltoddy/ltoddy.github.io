pub mod cli;
pub mod ops;

use std::io;

use clap::{CommandFactory, Parser};
use log::LevelFilter::Debug;
use simplelog::{ColorChoice, TermLogger, TerminalMode};

use crate::cli::{Cli, Posts, SubCommands};
use crate::ops::new_post;

pub fn main() -> anyhow::Result<()> {
    TermLogger::init(Debug, Default::default(), TerminalMode::Mixed, ColorChoice::Auto).unwrap();

    let cli: Cli = Cli::parse();
    match cli.subcommand {
        SubCommands::Post(subcommand) => match subcommand {
            Posts::New(args) => new_post(args)?,
        },
        SubCommands::Completions(args) => {
            let mut cmd = Cli::command();
            args.print_completions(&mut cmd, &mut io::stdout().lock());
        }
    }

    Ok(())
}
