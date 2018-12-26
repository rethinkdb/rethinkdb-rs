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

impl Default for super::Run {
    fn default() -> Self {
        super::Run {
            read_mode: ReadMode::Single,
            time_format: Format::Native,
            profile: false,
            durability: Durability::Soft,
            group_format: Format::Native,
        }
    }
}
