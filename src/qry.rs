// TODO remove after using these types
#![allow(dead_code)]

mod cmd {
    #[derive(Debug, Clone)]
    pub(crate) struct Command(Type, Argument, Option<Opt>);

    #[derive(Debug, Clone)]
    pub(crate) enum Argument {
        List(Vec<Argument>),
        Datum(Vec<u8>),
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Opt(Vec<u8>);

    #[derive(Debug, Clone)]
    pub(crate) enum Type {}
}

mod request {
    use super::cmd::Command;
    use crate::opt::Run;

    #[derive(Debug, Clone)]
    pub(crate) struct Request(Type, Option<Command>, Option<Run>);

    #[derive(Debug, Clone)]
    pub(crate) enum Type {
        Start = 1,
        Continue = 2,
        Stop = 3,
        NoreplyWait = 4,
        ServerInfo = 5,
    }
}

mod response {
    pub(crate) enum Type {
        SuccessAtom = 1,
        SuccessSequence = 2,
        SuccessPartial = 3,
        WaitComplete = 4,
        ServerInfo = 5,
        ClientError = 16,
        CompileError = 17,
        RuntimeError = 18,
    }

    pub(crate) enum Note {
        SequenceFeed = 1,
        AtomFeed = 2,
        OrderByLimitFeed = 3,
        UnionFeed = 4,
        IncludesStates = 5,
    }
}
