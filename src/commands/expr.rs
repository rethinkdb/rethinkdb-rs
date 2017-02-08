use {ToArg, Command};

/// Construct a ReQL JSON object from a native object
pub trait Expr {
    fn expr<T>(&self, value: T) -> Command
        where T: ToArg;
}

impl Expr for Command {
    fn expr<T>(&self, value: T) -> Command
        where T: ToArg
    {
        let mut cmd = Command::new();
        let logger = cmd.logger.new(o!("command" => "expr"));
        let arg = value.to_arg();
        cmd.set_term(arg.term);
        cmd.query += &format!(".expr({})", arg.string);
        debug!(logger, "{}", cmd.query);
        debug!(logger, "{:?}", cmd.term);
        cmd
    }
}
