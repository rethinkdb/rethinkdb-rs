use ToArg;
use commands::Command;

/// Construct a ReQL JSON object from a native object
pub trait Expr {
    fn expr<T>(&self, value: T) -> Command
        where T: ToArg;
}

impl Expr for Command {
    fn expr<T>(&self, value: T) -> Command
        where T: ToArg
    {
        args!(value)
    }
}
