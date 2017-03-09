use {Client, Result, Pool, Response, ToArg, slog};
use ql2::proto::Term;
use protobuf::repeated::RepeatedField;
use ql2::proto::Term_TermType;

pub fn new_client() -> Client {
    Client {
        term: Term::new(),
        query: String::from("r"),
        logger: slog::Logger::root(slog::Discard, o!()),
    }
}

pub fn make_cmd<A: ToArg>(client: &Client,
                          name: &'static str,
                          cmd_type: Option<Term_TermType>,
                          args: Option<A>)
                          -> Client {
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

pub fn with_logger(client: &Client, logger: slog::Logger) -> Client {
    let mut cmd = client.clone();
    cmd.logger = logger;
    cmd
}

pub fn with_args<A: ToArg>(client: &Client, args: A) -> Client {
    let args = args.to_arg();
    let mut cmd = client.clone();
    cmd.query += &format!(".with_args({})", args.string);
    let logger = cmd.logger.new(o!("command" => "with_args"));
    with_args!(cmd, args);
    debug!(logger, "{}", cmd.query);
    debug!(logger, "{:?}", cmd.term);
    cmd.with_logger(logger)
}
