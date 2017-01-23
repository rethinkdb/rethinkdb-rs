//! Traits for executing ReQL commands

use commands::Command;

/// A `run` command that does nothing
///
/// Use this in examples and documentation where you don't want actual command execution to occur.
pub trait Dummy {
    fn run<T>(&self) { }
}

impl Dummy for Command { }
