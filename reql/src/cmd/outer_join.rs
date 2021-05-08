use crate::cmd;

pub trait Arg {
    fn arg(self) -> cmd::Arg<()>;
}
