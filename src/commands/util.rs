use {Client, IntoArg, slog};
use protobuf::repeated::RepeatedField;
use ql2::proto::Term;
use ql2::proto::Term_TermType;

pub fn new_client() -> Client {
    Client {
        term: Ok(Term::new()),
        query: String::from("r"),
        write: false,
        logger: slog::Logger::root(slog::Discard, o!()),
    }
}

pub fn make_cmd<A: IntoArg>(client: &Client,
                            name: &'static str,
                            cmd_type: Option<Term_TermType>,
                            args: Option<A>)
                            -> Client {
    let cterm = match client.term {
        Ok(ref term) => term.clone(),
        Err(_) => {
            return client.clone();
        }
    };
    let logger = client.logger.new(o!("command" => name));
    let mut term = Term::new();
    if let Some(cmd_type) = cmd_type {
        term.set_field_type(cmd_type);
    }
    if cterm != Term::new() {
        let prev_cmd = RepeatedField::from_vec(vec![cterm.clone()]);
        term.set_args(prev_cmd);
    }
    let mut cmd = Client::new();
    cmd.term = Ok(term);
    match args {
        Some(args) => {
            let arg = args.into_arg();
            cmd.query = format!("{}.{}({})", client.query, name, arg.string);
            let aterm = match arg.term {
                Ok(term) => term,
                Err(error) => {
                    cmd.term = Err(error);
                    return cmd;
                }
            };
            with_args!(cmd, aterm);
        }
        None => {
            cmd.query = format!("{}.{}()", client.query, name);
        }
    }
    debug!(cmd.logger, "{}", cmd.query);
    debug!(cmd.logger, "{:?}", cmd.term);
    cmd.with_logger(logger)
}

pub fn with_logger(client: &Client, logger: slog::Logger) -> Client {
    let mut cmd = client.clone();
    cmd.logger = logger;
    cmd
}

pub fn with_args<A: IntoArg>(client: &Client, args: A) -> Client {
    let mut cmd = client.clone();
    if let Err(_) = cmd.term {
        return cmd;
    }
    let args = args.into_arg();
    cmd.query += &format!(".with_args({})", args.string);
    let aterm = match args.term {
        Ok(term) => term,
        Err(error) => {
            cmd.term = Err(error);
            return cmd;
        }
    };
    let logger = cmd.logger.new(o!("command" => "with_args"));
    with_args!(cmd, aterm);
    debug!(logger, "{}", cmd.query);
    debug!(logger, "{:?}", cmd.term);
    cmd.with_logger(logger)
}
