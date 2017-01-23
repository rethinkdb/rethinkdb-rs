use commands::Command;

/// Specify an optional argument to a ReQL command
///
/// # Example
///
/// Pass the `right_bound` optional argument to [between](trait.Between.html).
///
/// ```
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # struct Heroes;
/// # let r = Command::new();
/// r.table("marvel").between(10, 20).opt_arg("right_bound", "closed").run::<Heroes>();
/// ```
///
/// To pass more than one optional argument, chain `opt_arg` once for each argument.
///
/// # Example
///
/// ```
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # struct Heroes;
/// # let r = Command::new();
/// r.table("marvel").between(10, 20)
///     .opt_arg("right_bound", "closed")
///     .opt_arg("index", "power")
///     .run::<Heroes>();
/// ```
pub trait OptArg {
    fn opt_arg<T>(&self, option: &str, value: T) -> Command
        where T: ::ToArg;
}

impl OptArg for Command {
    fn opt_arg<T>(&self, option: &str, value: T) -> Command
        where T: ::ToArg
    {
        let mut cmd = self.clone();
        {
            let term = cmd.mut_term();
            let temp_pair = Command::create_term_pair(option, value);
            term.mut_optargs().push(temp_pair);
        }
        cmd
    }
}
