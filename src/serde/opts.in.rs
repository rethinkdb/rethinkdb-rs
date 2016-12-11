#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableOpts {
    read_mode: ReadMode,
    identifier_format: IdentifierFormat,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ReadMode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "majority")]
    Majority,
    #[serde(rename = "outdated")]
    Outdated,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IdentifierFormat {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "uuid")]
    Uuid,
}

impl Default for TableOpts {
    fn default() -> TableOpts {
        TableOpts {
            read_mode: ReadMode::Single,
            identifier_format: IdentifierFormat::Name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangesOpts<T> {
    squash: T,
    changefeed_queue_size: u64,
    include_initial: bool,
    include_states: bool,
    include_offsets: bool,
    include_types: bool,
}

macro_rules! default_changes_opts {
    ($T:ident is $V:expr) => {
        impl Default for ChangesOpts<$T> {
            fn default() -> ChangesOpts<$T> {
                ChangesOpts {
                    squash: $V,
                    changefeed_queue_size: 100_000,
                    include_initial: true,
                    include_states: false,
                    include_offsets: false,
                    include_types: false,
                }
            }
        }
    }
}

default_changes_opts!{ bool is false }
default_changes_opts!{ f32 is 0.0 }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAllOpts {
    index: StdString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Format {
    #[serde(rename = "native")]
    Native,
    #[serde(rename = "raw")]
    Raw,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Durability {
    #[serde(rename = "hard")]
    Hard,
    #[serde(rename = "soft")]
    Soft,
}
