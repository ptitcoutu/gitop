#![allow(unused)]
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::fmt;
use std::fmt::{Debug, Formatter};

use clap::{Parser, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::{Level, LevelFilter};
use pretty_env_logger::env_logger;
use pretty_env_logger::env_logger::Builder;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: Verbosity,
    command: Command,
    operation: String,
    source_branch: String,
    dependencies: Vec<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum Command {
    Tag,
    Check,
}

trait Runnable {
    fn run(&self);
}
impl fmt::Display for Cli {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command: {}, operation: {}, source_branch: {}, dependencies : {} ",
            self.command,
            self.operation,
            self.source_branch,
            self.dependencies.join(",")
        )
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Command::Tag => write!(f, "tag"),
            Command::Check => write!(f, "check"),
        }
    }
}

impl Runnable for Command {
    fn run(&self) {
        match self {
            Command::Tag => {
                info!("tag command")
            }
            Command::Check => {
                info!("check command")
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let filter_level = args.verbose.log_level_filter();
    Builder::new().filter_level(filter_level).init();
    info!("gitop with args {}", args);
    args.command.run();
    Ok(())
}
