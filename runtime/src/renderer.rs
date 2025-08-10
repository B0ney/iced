//! A renderer draws the different primitives of your user interface.

pub use crate::graphics::backend;

pub use crate::graphics::Error;

pub use backend::Backend;
pub use iced_graphics::compositor::Settings;

use crate::task;

use crate::{Action, Task};

/// Changes the [`Settings`] of the current renderer.
pub fn change(settings: Settings) -> Task<Result<(), Error>> {
    task::oneshot(|channel| Action::ChangeRenderer { settings, channel })
}
