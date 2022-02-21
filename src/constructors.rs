use crate::types::{Color, Scalar};

pub const fn rgb(r: Scalar, g: Scalar, b: Scalar) -> Color {
    rgba(r, g, b, 1.0)
}

pub const fn rgba(r: Scalar, g: Scalar, b: Scalar, a: Scalar) -> Color {
    Color { r, g, b, a }
}

pub fn hex_bytes(hex: u32) -> [u8; 4] {
    [
        ((hex & 0x00FF0000) >> 16) as u8,
        ((hex & 0x0000FF00) >> 8) as u8,
        (hex & 0x000000FF) as u8,
        ((hex & 0xFF000000) >> 24) as u8,
    ]
}

pub fn hex(hex: u32) -> Color {
    let bytes = hex_bytes(hex);

    Color {
        r: bytes[0] as f64 / 255.0,
        g: bytes[1] as f64 / 255.0,
        b: bytes[2] as f64 / 255.0,
        a: bytes[3] as f64 / 255.0,
    }
}
