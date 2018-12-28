#[derive(Debug, Clone)]
pub struct Opts {
    pub(super) read_mode: ReadMode,
    pub(super) time_format: Format,
    pub(super) profile: bool,
    pub(super) durability: Durability,
    pub(super) group_format: Format,
}

#[derive(Debug, Clone)]
pub enum ReadMode {
    Single,
    Majority,
    Outdated,
}

#[derive(Debug, Clone)]
pub enum Durability {
    Hard,
    Soft,
}

#[derive(Debug, Clone)]
pub enum Format {
    Native,
    Raw,
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            read_mode: ReadMode::Single,
            time_format: Format::Native,
            profile: false,
            durability: Durability::Soft,
            group_format: Format::Native,
        }
    }
}
