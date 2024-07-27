use iced_graphics::core::Color;

pub trait ColorConversion {
    fn convert(color: Color) -> tiny_skia::Color;
}

#[derive(Debug)]
pub struct BGRA;

impl ColorConversion for BGRA {
    fn convert(color: Color) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(color.b, color.g, color.r, color.a)
            .expect("Convert color from iced to tiny_skia")
    }
}

#[derive(Debug)]
pub struct RGBA;

impl ColorConversion for RGBA {
    fn convert(color: Color) -> tiny_skia::Color {
        tiny_skia::Color::from_rgba(color.r, color.g, color.b, color.a)
            .expect("Convert color from iced to tiny_skia")
    }
}
