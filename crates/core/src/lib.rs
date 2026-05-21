pub mod error;
pub mod scanner;
pub mod cleaner;
pub mod duplicates;
pub mod types;

pub use error::{CoreError, Result};
pub use types::{FileEntry, ScanCategory, ScanResult, CleanResult};
