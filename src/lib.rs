#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::multiple_crate_versions)]
#![allow(clippy::missing_errors_doc)]
pub use error::{Error, Result};
pub use protocol::{responses::card_punch::CardPunch, CardOwnerData, CardReadout};
pub use reader::Reader;
mod error;
mod protocol;
mod reader;
