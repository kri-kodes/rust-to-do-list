//! # Journal Actions
//!
//! A struct for modeling actions/entries for to-do journal
//!

use std::path::PathBuf;
use structopt::StructOpt;

// An enumeration `Action` to represent different actions that can be performed on the
// journal.

#[derive(Debug, StructOpt)]
pub enum Action {
    ///Add an action to the journal file.
    Add {
        #[structopt()]
        task: String,
    },
    ///Remove a task with given position from the journal file.
    Done {
        #[structopt()]
        position: usize,
    },
    ///List all the tasks in the journal file.
    List,
}

// Define a struct `CommandLineArgs` to represent the command line arguments.

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Rust Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    /// The action to be performed, represented by the `Action` enum.
    #[structopt(subcommand)]
    pub action: Action,
    /// Optional journal file path, represented by `Option<PathBuf>`.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
