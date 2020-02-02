mod args;
mod io;
mod util;

use crate::{Client, Config, Connection, IntoArg, Result};
use ql2::proto::{Term, Term_TermType as Type};

include!(concat!(env!("OUT_DIR"), "/commands.rs"));
