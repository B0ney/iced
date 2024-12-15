//! Access the clipboard.
use crate::core::clipboard::Kind;


/// A clipboard action to be performed by some [`Task`].
///
/// [`Task`]: crate::Task
#[derive(Debug)]
pub enum Action {
    /// Read the clipboard and produce `T` with the result.
    Read {
        /// The clipboard target.
        target: Kind,
    },

    /// Write the given contents to the clipboard.
    Write {
        /// The clipboard target.
        target: Kind,
        /// The contents to be written.
        contents: String,
    },
}
