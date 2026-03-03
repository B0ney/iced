//! Handle events of a user interface.
use crate::clipboard;
use crate::input_method;
use crate::keyboard;
use crate::mouse;
use crate::touch;
use crate::window;

/// A user interface event.
///
/// _**Note:** This type is largely incomplete! If you need to track
/// additional events, feel free to [open an issue] and share your use case!_
///
/// [open an issue]: https://github.com/iced-rs/iced/issues
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// A keyboard event
    Keyboard(keyboard::Event),

    /// A mouse event
    Mouse(mouse::Event),

    /// A window event
    Window(window::Event),

    /// A touch event
    Touch(touch::Event),

    /// An input method event
    InputMethod(input_method::Event),

    /// A clipboard event
    Clipboard(clipboard::Event),

    /// A custom event
    Custom(custom::Event),
}

/// The status of an [`Event`] after being processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Event`] was **NOT** handled by any widget.
    Ignored,

    /// The [`Event`] was handled and processed by a widget.
    Captured,
}

impl Status {
    /// Merges two [`Status`] into one.
    ///
    /// `Captured` takes precedence over `Ignored`:
    ///
    /// ```
    /// use iced_core::event::Status;
    ///
    /// assert_eq!(Status::Ignored.merge(Status::Ignored), Status::Ignored);
    /// assert_eq!(Status::Ignored.merge(Status::Captured), Status::Captured);
    /// assert_eq!(Status::Captured.merge(Status::Ignored), Status::Captured);
    /// assert_eq!(Status::Captured.merge(Status::Captured), Status::Captured);
    /// ```
    pub fn merge(self, b: Self) -> Self {
        match self {
            Status::Ignored => b,
            Status::Captured => Status::Captured,
        }
    }
}

/// A custom runtime event
pub mod custom {
    use std::{any::Any, sync::Arc};

    use crate::Maybe;

    /// A custom runtime event
    #[derive(Clone)]
    pub struct Event {
        inner: Arc<dyn Any + Send + Sync + 'static>,
    }

    impl Event {
        /// Construct a new type erased custom event
        pub fn new<T: Send + Sync + Clone + PartialEq + 'static>(data: T) -> Self {
            Self {
                inner: Arc::new(data),
            }
        }
    }

    impl PartialEq for Event {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }

    impl std::fmt::Debug for Event {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Event").field("inner", &"...").finish()
        }
    }

    impl Event {
        /// Attempt to obtain the value of the custom event.
        pub fn get<T: Clone + PartialEq + 'static>(&self) -> Maybe<&T> {
            match self.inner.downcast_ref() {
                Some(value) => Maybe::Some(value),
                None => Maybe::None,
            }
        }
    }
}
