mod io;
mod util;
mod args;

use crate::{Config, Client, IntoArg, Result, Connection};
use ql2::proto::{Term, Term_TermType as Type};

include!(concat!(env!("OUT_DIR"), "/commands.rs"));
