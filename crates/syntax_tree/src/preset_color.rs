use warpui_core::color::ColorU;

pub const fn hex(r: u8, g: u8, b: u8) -> ColorU {
    ColorU { r, g, b, a: 255 }
}
