use Command;

/// Specify an optional argument to a ReQL command
///
/// # Example
///
/// Pass the `right_bound` optional argument to [between](trait.Between.html).
///
/// ```
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # use reql::r;
/// # struct Heroes;
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
/// # use reql::r;
/// # struct Heroes;
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
        // Squash the value into a single term
        let mut term = ::ql2::proto::Term::new();
        term.mut_args().push(value.to_arg());
        // Create a term pair to hold our option and value
        let mut term_pair = ::ql2::proto::Term_AssocPair::new();
        term_pair.set_key(option.into());
        term_pair.set_val(term);
        // Push it into our term
        cmd.term.mut_optargs().push(term_pair);
        cmd
    }
}
