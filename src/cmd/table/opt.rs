#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Opts {
    read_mode: ReadMode,
    identifier_format: IdentifierFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

#[derive(Debug, Clone, Copy)]
pub enum IdentifierFormat {
    Name,
    Uuid,
}
