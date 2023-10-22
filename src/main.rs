#![allow(unused)]
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::borrow::BorrowMut;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::io::Error;

use clap::{Parser, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use downcast_rs::{impl_downcast, DowncastSync};
use log::{Level, LevelFilter};
use pretty_env_logger::env_logger;
use pretty_env_logger::env_logger::Builder;

#[derive(Clone, Debug, Parser)]
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
    fn run(&self, cli: &Cli, repo: &mut dyn SourceRepository) -> Result<String, Error>;
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
    fn run(&self, args: &Cli, repo: &mut dyn SourceRepository) -> Result<String, Error> {
        match self {
            Command::Tag => Self::tag(args, repo),

            Command::Check => Self::check(args, repo),
        }
    }
}
trait SourceRepository: DowncastSync {
    fn get_current_branch(&self) -> String;
    fn push_tag(&mut self, tag: String);
}
impl_downcast!(sync SourceRepository);
struct GitSourceRepository {}

impl SourceRepository for GitSourceRepository {
    fn get_current_branch(&self) -> String {
        String::from("master")
    }

    fn push_tag(&mut self, tag: String) {
        info!("push tag {}", tag)
    }
}

impl Command {
    fn tag(cli: &Cli, repo: &mut dyn SourceRepository) -> Result<String, Error> {
        info!("tag command");
        let branch_name = repo.get_current_branch();
        let tag_name = format!("gitop/{}/{}", branch_name, cli.operation);
        repo.push_tag(tag_name.clone());
        Ok(tag_name)
    }
    fn check(cli: &Cli, repo: &dyn SourceRepository) -> Result<String, Error> {
        info!("check command");
        Ok(String::from(""))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let filter_level = args.verbose.log_level_filter();
    Builder::new().filter_level(filter_level).init();
    info!("gitop with args {}", args.clone());
    let mut git_repo = GitSourceRepository {};
    args.command.run(&args, &mut git_repo);
    Ok(())
}

#[test]
fn test_tag_command() {
    // given a fake repo
    #[derive(Clone, Debug)]
    struct FakeRepository {
        pushed_tag: String,
    }
    impl SourceRepository for FakeRepository {
        fn get_current_branch(&self) -> String {
            String::from("master")
        }

        fn push_tag(&mut self, tag: String) {
            self.pushed_tag = tag;
        }
    }
    trait TrackableCall {
        fn get_last_pushed_tag(&self) -> String;
    }
    impl TrackableCall for FakeRepository {
        fn get_last_pushed_tag(&self) -> String {
            self.pushed_tag.clone()
        }
    }

    let mut fake_repo = FakeRepository {
        pushed_tag: String::from("no"),
    };
    // and a tag command
    let cli = Cli {
        verbose: Verbosity::new(0, 0),
        command: Command::Tag,
        operation: String::from("test"),
        source_branch: String::from("master"),
        dependencies: vec![],
    };

    // when tag using defined arguments
    Command::tag(&cli, &mut fake_repo);
    // then a new tag with op name and branch name is pushed
    let last_pushed_tag = fake_repo.get_last_pushed_tag();
    assert_eq!(&last_pushed_tag, "gitop/master/test")
}
