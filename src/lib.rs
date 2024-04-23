#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_errors_doc)]
pub use error::*;
pub use reader::*;

mod error;
mod protocol;
mod reader;
