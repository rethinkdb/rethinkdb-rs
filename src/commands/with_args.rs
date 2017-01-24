use commands::Command;

/// Specify optional arguments to a ReQL command
///
/// Normally, you should use the `args!()` macro to pass arguments to a command that
/// also takes optional arguments. If the command takes at least one argument, you
/// don't need to call `with_args`. However, some commands like [get_all](trait.GetAll.html)
/// and [delete](trait.Delete.html) do not have any required arguments but yet they have
/// optional ones. That's when `with_args` comes in.
///
/// # Example
///
/// Secondary index keys are not guaranteed to be unique so we cannot query via
/// [get](trait.Get.html) when using a secondary index.
///
/// ```
/// # #[macro_use] extern crate reql;
/// # fn main() {
/// # use reql::commands::*;
/// # use reql::commands::run::Dummy;
/// # struct Heroe;
/// # let r = Command::new();
/// r.table("marvel").get_all().with_args(args!("man_of_steel", {index: "code_name"})).run::<Heroe>();
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
        let mut cmd = self.clone();
        {
            let term = cmd.mut_term();
            let mut tmp_args = args.to_arg();
            if tmp_args.has_field_type() { // did not come from the args macro
                term.mut_args().push(tmp_args);
            } else { // came from the args macro
                for arg in tmp_args.take_args().into_vec() {
                    term.mut_args().push(arg);
                }
                for pair in tmp_args.take_optargs().into_vec() {
                    term.mut_optargs().push(pair);
                }
            }
        }
        cmd
    }
}
