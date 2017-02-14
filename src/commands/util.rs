use {Client, Result, Pool, Response, ToArg, slog};
use ql2::proto::Term;
use protobuf::repeated::RepeatedField;
use ql2::proto::Term_TermType;
#[cfg(feature = "with_io")]
use reql_io::serde::Deserialize;

// #[cfg(feature = "with_io")]
// mod io;
// #[cfg(feature = "with_io")]
// pub use self::io::*;
//

pub fn new_client<A: ToArg>() -> Client<A> {
    Client {
        term: Term::new(),
        query: String::from("r"),
        logger: slog::Logger::root(slog::Discard, o!()),
        phantom: ::std::marker::PhantomData,
    }
}

pub fn make_cmd<A: ToArg>(client: &Client<A>,
                          name: &'static str,
                          cmd_type: Option<Term_TermType>,
                          args: Option<A>)
                          -> Client<A> {
    let logger = client.logger.new(o!("command" => name));
    let mut term = Term::new();
    if let Some(cmd_type) = cmd_type {
        term.set_field_type(cmd_type);
    }
    if client.term != Term::new() {
        let prev_cmd = RepeatedField::from_vec(vec![client.term.clone()]);
        term.set_args(prev_cmd);
    }
    let mut cmd = Client::new();
    cmd.term = term;
    match args {
        Some(args) => {
            let arg = args.to_arg();
            with_args!(cmd, arg);
            cmd.query = format!("{}.{}({})", client.query, name, arg.string);
        }
        None => {
            cmd.query = format!("{}.{}()", client.query, name);
        }
    }
    debug!(logger, "{}", cmd.query);
    debug!(logger, "{:?}", cmd.term);
    cmd.with_logger(logger)
}

pub fn with_logger<A: ToArg>(client: &Client<A>, logger: slog::Logger) -> Client<A> {
    // let mut cmd: () = client.clone();
    //
    // cmd.logger = logger;
    // cmd
    //

    Client::new()
}

pub fn connect<A: ToArg>(client: &Client<A>, args: A) -> Result<Pool> {
    unimplemented!();
}

pub fn run<A: ToArg, T: Deserialize>(client: &Client<A>, args: A) -> Result<Response<T>> {
    unimplemented!();
}
