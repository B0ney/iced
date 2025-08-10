use crate::backend::Backend;
use crate::core::{self, Font, Pixels};

/// The settings of a renderer.
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    /// The default [`Font`] to use.
    pub default_font: Font,

    /// The default size of text.
    ///
    /// By default, it will be set to `16.0`.
    pub default_text_size: Pixels,

    /// It defaults to [`Backend::Best`].
    pub backend: Backend,

    /// Whether or not to synchronize frames.
    ///
    /// By default, it is `true`.
    pub vsync: bool,

    /// Enabling it can produce a smoother result in some widgets, like the
    /// `Canvas`, at a performance cost.
    ///
    /// By default, it is disabled.
    pub antialiasing: bool,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            default_font: Font::default(),
            default_text_size: Pixels(16.0),
            vsync: true,
            antialiasing: false,
            backend: Backend::default(),
        }
    }
}

impl From<core::Settings> for Settings {
    fn from(settings: core::Settings) -> Self {
        Self {
            default_font: if cfg!(all(
                target_arch = "wasm32",
                feature = "fira-sans"
            )) && settings.default_font == Font::default()
            {
                Font::with_name("Fira Sans")
            } else {
                settings.default_font
            },
            default_text_size: settings.default_text_size,
            vsync: settings.vsync,
            antialiasing: settings.antialiasing,
            backend: Backend::default(),
        }
    }
}
