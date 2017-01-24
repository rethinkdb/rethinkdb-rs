use commands::Command;

/// Specify an optional argument to a ReQL command
///
/// # Example
///
/// Pass the `right_bound` optional argument to [between](trait.Between.html).
///
/// ```
/// # #[macro_use] extern crate reql;
/// # fn main() {
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # struct Heroes;
/// # let r = Command::new();
/// r.table("marvel").between(args!(10, 20, {right_bound: "closed"})).run::<Heroes>();
/// # }
/// ```
///
/// To pass more than one optional argument, chain `opt_arg` once for each argument.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate reql;
/// # fn main() {
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # struct Heroes;
/// # let r = Command::new();
/// r.table("marvel").between(args!(10, 20, {
///     right_bound: "closed",
///     index: "power",
/// }))
/// .run::<Heroes>();
/// # }
/// ```
///
/// The key is optional because some commands (eg. `error`) have optional arguments that
/// that can only be specified via `opt_arg`. In such a case just pass in `None` as the key.
pub trait OptArg {
    fn opt_arg<'a, K, V>(&self, option: K, value: V) -> Command
        where K: Into<Option<&'a str>>, V: ::ToArg;
}

impl OptArg for Command {
    fn opt_arg<'a, K, V>(&self, key: K, value: V) -> Command
        where K: Into<Option<&'a str>>, V: ::ToArg
    {
        let mut cmd = self.clone();
        {
            let term = cmd.mut_term();
            match key.into() {
                Some(option) => {
                    let temp_pair = Command::create_term_pair(option, value);
                    term.mut_optargs().push(temp_pair);
                }
                None => {
                    term.mut_args().push(value.to_arg());
                }
            }
        }
        cmd
    }
}
