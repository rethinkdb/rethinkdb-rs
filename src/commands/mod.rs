//! The ReQL command reference

pub mod run;

commands! {
    db_list,            changes,            db_create,          db_drop,            table_create,       table_drop,
    table_list,         index_create,       index_drop,         index_list,         index_rename,       index_status,
    index_wait,         insert,             update,             replace,            delete,             sync,
    db,                 table,              get,                get_all,            between,            filter,
    inner_join,         outer_join,         eq_join,            zip,                map,                with_fields,
    concat_map,         order_by,           skip,               limit,              slice,              nth,
    offsets_of,         is_empty,           union,              sample,             group,              ungroup,
    reduce,             fold,               count,              sum,                avg,                min,
    max,                distinct,           contains,           pluck,              without,            merge,
    append,             prepend,            difference,         set_insert,         set_union,          set_intersection,
    set_difference,     get_field,          has_fields,         insert_at,          splice_at,          delete_at,
    change_at,          keys,               values,             literal,            expr,               match_,
    split,              upcase,             downcase,           add,                sub,                mul,
    div,                mod_,               and,                or,                 eq,                 ne,
    gt,                 ge,                 lt,                 le,                 not,                random,
    round,              ceil,               floor,              now,                time,               epoch_time,
    iso8601,            in_timezone,        timezone,           during,             date,               time_of_day,
    year,               month,              day,                day_of_week,        day_of_year,        hours,
    minutes,            seconds,            to_iso8601,         to_epoch_time,      wait,               binary,
    do_,                branch,             for_each,           range,              error,              default,
    status,             js,                 coerce_to,          type_of,            info,               json,
    to_json,            http,               uuid,               circle,             distance,           fill,
    geojson,            to_geojson,         get_intersecting,   get_nearest,        includes,           intersects,
    line,               point,              polygon,            polygon_sub,        grant,              config,
    rebalance,          reconfigure,        with_args,          connection,
}

use ql2::proto::{Term, Term_AssocPair as TermPair};

/// The type returned by every error
#[must_use = "command results are moved from one command to another so you must either catch a command's result using a let binding or chain the command all the way through"]
#[derive(Debug, Clone)]
pub struct Command {
    term: Term,
}

impl Command {
    /// Creates a new command
    ///
    /// This is typically called `r`.
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate reql;
    /// # use reql::commands::*;
    /// # use reql::commands::run::Dummy;
    /// # struct Users;
    /// # fn main() {
    /// # let conn = ();
    /// let r = Command::new();
    /// r.table("users").run::<Users>(&conn);
    /// # }
    /// ```
    pub fn new() -> Command {
        Command {
            term: Term::new(),
        }
    }

    #[doc(hidden)]
    pub fn term(&self) -> &Term {
        &self.term
    }

    #[doc(hidden)]
    pub fn mut_term(&mut self) -> &mut Term {
        &mut self.term
    }

    #[doc(hidden)]
    pub fn set_term(&mut self, term: Term) {
        self.term = term;
    }
}

/// The return type of the `args!()` macro
#[derive(Debug, Clone)]
pub struct Args {
    term: Term,
}

impl Args {
    #[doc(hidden)]
    pub fn new() -> Args {
        Args {
            term: Term::new(),
        }
    }

    #[doc(hidden)]
    pub fn term(&self) -> &Term {
        &self.term
    }

    #[doc(hidden)]
    pub fn mut_term(&mut self) -> &mut Term {
        &mut self.term
    }

    #[doc(hidden)]
    pub fn set_term(&mut self, term: Term) {
        self.term = term;
    }

    #[doc(hidden)]
    pub fn create_term_pair<T: ::ToArg>(key: &str, val: T) -> TermPair {
        let mut temp = Term::new();
        temp.mut_args().push(val.to_arg());
        let mut temp_pair = TermPair::new();
        temp_pair.set_key(key.into());
        temp_pair.set_val(temp);
        temp_pair
    }
}
