use Command;

/// Specify optional arguments to a ReQL command
///
/// Normally, you should use the `args!()` macro to pass arguments to a command that
/// also takes optional arguments. If the command takes at least one argument, you
/// don't need to call `with_args`. However, some commands like [error](trait.Error.html)
/// and [delete](trait.Delete.html) do not have any required arguments but yet they have
/// optional ones. That's when `with_args` comes in.
///
/// # Example
///
/// Delete all documents from the table `comments` without waiting for the operation to be flushed to
/// disk.
///
/// ```
/// # #![allow(unused_must_use)]
/// # #[macro_use] extern crate reql;
/// # fn main() {
/// # use reql::Command;
/// # use reql::commands::*;
/// # let r = Command::new();
/// r.table("comments").delete().with_args(args!({durability: "soft"}));
/// # }
/// ```
pub trait WithArgs {
    fn with_args<T>(&self, args: T) -> Command
        where T: ::ToArg;
}

impl WithArgs for Command {
    fn with_args<T>(&self, args: T) -> Command
        where T: ::ToArg
    {
        let args = args.to_arg();
        let mut cmd = self.clone();
        cmd.query += &format!(".with_args({})", args.string);
        let logger = cmd.logger.new(o!("command" => "with_args"));
        with_args!(cmd, args);
        debug!(logger, "{}", cmd.query);
        debug!(logger, "{:?}", cmd.term);
        cmd.with_logger(logger)
    }
}
