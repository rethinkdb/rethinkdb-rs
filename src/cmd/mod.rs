//! ReQL command reference

pub mod connect;
pub mod db;
pub mod expr;
pub mod merge;
pub mod run;
pub mod table;

#[doc(hidden)]
macro make_builder() {
    /// Start building the options
    pub fn builder() -> Self {
        Default::default()
    }

    /// Finalise the options
    pub fn build(&self) -> Self {
        *self
    }
}
