//! A renderer-agnostic native GUI runtime.
//!
//! ![The native path of the Iced ecosystem](https://github.com/iced-rs/iced/blob/master/docs/graphs/native.png?raw=true)
//!
//! `iced_runtime` takes [`iced_core`] and builds a native runtime on top of it.
//!
//! [`iced_core`]: https://github.com/iced-rs/iced/tree/master/core
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod clipboard;
pub mod font;
pub mod image;
pub mod keyboard;
pub mod system;
pub mod task;
pub mod user_interface;
pub mod widget;
pub mod window;

pub use iced_core as core;
pub use iced_futures as futures;

pub use task::Task;
pub use user_interface::UserInterface;
pub use window::Window;

use crate::core::Event;
use crate::futures::futures::channel::oneshot;

use std::borrow::Cow;
use std::fmt;

/// An action that the iced runtime can perform.
pub enum Action<T> {
    /// Output some value.
    Output(T),

    /// Load a font from its bytes.
    LoadFont {
        /// The bytes of the font to load.
        bytes: Cow<'static, [u8]>,
        /// The channel to send back the load result.
        channel: oneshot::Sender<Result<(), font::Error>>,
    },

    /// Run a widget operation.
    Widget(Box<dyn core::widget::Operation>),

    /// Run a clipboard action.
    Clipboard(clipboard::Action),

    /// Run a window action.
    Window(window::Action),

    /// Run a system action.
    System(system::Action),

    /// Run an image action.
    Image(image::Action),

    /// Produce an event.
    Event {
        /// The [`window::Id`](core::window::Id) of the event.
        window: core::window::Id,
        /// The [`Event`] to be produced.
        event: Event,
    },

    /// Poll any resources that may have pending computations.
    Tick,

    /// Recreate all user interfaces and redraw all windows.
    Reload,

    /// Exits the runtime.
    ///
    /// This will normally close any application windows and
    /// terminate the runtime loop.
    Exit,

    /// A type-erased custom action.
    ///
    /// Useful for custom runtimes.
    Custom(custom::Action),
}

/// A custom runtime event
pub mod custom {
    use std::any::Any;

    use crate::core::Maybe;

    /// A custom runtime event
    pub struct Action {
        inner: Box<dyn Any + Send + 'static>,
    }

    impl Action {
        /// Construct a new type erased custom Action
        pub fn new<T: Clone + Send + 'static>(data: T) -> Self {
            Self {
                inner: Box::new(data),
            }
        }
    }

    impl std::fmt::Debug for Action {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Action").field("inner", &"...").finish()
        }
    }

    impl Action {
        /// Attempt to obtain the value of the custom Action.
        pub fn get<T: Clone + 'static>(&self) -> Maybe<&T> {
            match self.inner.downcast_ref() {
                Some(value) => Maybe::Some(value),
                None => Maybe::None,
            }
        }

        // /// Attempt to take the value of the custom Action.
        // pub fn take<T: Clone + 'static>(self) -> Result<T, Self> {
        //     match self.inner.downcast() {
        //         Ok(value) => Ok(*value),
        //         Err(inner) => Err(Self { inner }),
        //     }
        // }
    }
}

impl<T> Action<T> {
    /// Creates a new [`Action::Widget`] with the given [`widget::Operation`](core::widget::Operation).
    pub fn widget(operation: impl core::widget::Operation + 'static) -> Self {
        Self::Widget(Box::new(operation))
    }

    fn output<O>(self) -> Result<T, Action<O>> {
        match self {
            Action::Output(output) => Ok(output),
            Action::LoadFont { bytes, channel } => Err(Action::LoadFont { bytes, channel }),
            Action::Widget(operation) => Err(Action::Widget(operation)),
            Action::Clipboard(action) => Err(Action::Clipboard(action)),
            Action::Window(action) => Err(Action::Window(action)),
            Action::System(action) => Err(Action::System(action)),
            Action::Image(action) => Err(Action::Image(action)),
            Action::Event { window, event } => Err(Action::Event { window, event }),
            Action::Custom(action) => Err(Action::Custom(action)),
            Action::Tick => Err(Action::Tick),
            Action::Reload => Err(Action::Reload),
            Action::Exit => Err(Action::Exit),
        }
    }
}

impl<T> fmt::Debug for Action<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Output(output) => write!(f, "Action::Output({output:?})"),
            Action::LoadFont { .. } => {
                write!(f, "Action::LoadFont")
            }
            Action::Widget { .. } => {
                write!(f, "Action::Widget")
            }
            Action::Clipboard(action) => {
                write!(f, "Action::Clipboard({action:?})")
            }
            Action::Window(_) => write!(f, "Action::Window"),
            Action::System(action) => write!(f, "Action::System({action:?})"),
            Action::Image(_) => write!(f, "Action::Image"),
            Action::Event { window, event } => write!(
                f,
                "Action::Event {{ window: {window:?}, event: {event:?} }}"
            ),
            Action::Tick => write!(f, "Action::Tick"),
            Action::Reload => write!(f, "Action::Reload"),
            Action::Exit => write!(f, "Action::Exit"),
            Action::Custom(_) => write!(f, "Action::Custom"),
        }
    }
}

/// Creates a [`Task`] that exits the iced runtime.
///
/// This will normally close any application windows and
/// terminate the runtime loop.
pub fn exit<T>() -> Task<T> {
    task::effect(Action::Exit)
}
