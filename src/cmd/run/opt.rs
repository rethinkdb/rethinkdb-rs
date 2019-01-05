#[derive(Debug, Clone, Copy)]
pub struct Opts<'a> {
    pub(super) read_mode: ReadMode,
    pub(super) time_format: Format,
    pub(super) profile: bool,
    pub(super) durability: Durability,
    pub(super) group_format: Format,
    pub(super) db: &'a str,
}

#[derive(Debug, Clone, Copy)]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

#[derive(Debug, Clone, Copy)]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Native,
    Raw,
}

impl<'a> Default for Opts<'a> {
    fn default() -> Self {
        Self {
            read_mode: ReadMode::Single,
            time_format: Format::Native,
            profile: false,
            durability: Durability::Soft,
            group_format: Format::Native,
            db: "test",
        }
    }
}
