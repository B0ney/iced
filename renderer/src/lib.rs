//! The official renderer for iced.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub use iced_graphics as graphics;
pub use iced_graphics::core;

#[cfg(feature = "geometry")]
pub use iced_graphics::geometry;

/// The default graphics renderer for [`iced`].
///
/// [`iced`]: https://github.com/iced-rs/iced
pub type Renderer = iced_tiny_skia::Renderer;
