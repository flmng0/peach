#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
/// RGBA color structure represented by `f32`s. Within
/// Peach, this structure is actually computed as SRGBA
/// because of [`wgpu`].
///
/// [`wgpu`]: https://crates.io/crates/wgpu
pub struct Color {
    /// Red component.
    pub r: f32,
    /// Green component.
    pub g: f32,
    /// Blue component.
    pub b: f32,
    /// Alpha component.
    pub a: f32,
}

impl Color {
    pub const TRANSPARENT: Color = Color::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);

    pub const RED: Color = Color::new(1.0, 0.0, 0.0, 1.0);
    pub const YELLOW: Color = Color::new(1.0, 1.0, 0.0, 1.0);
    pub const GREEN: Color = Color::new(0.0, 1.0, 0.0, 1.0);
    pub const CYAN: Color = Color::new(0.0, 1.0, 1.0, 1.0);
    pub const BLUE: Color = Color::new(0.0, 0.0, 1.0, 1.0);
    pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);

    /// Create a new color, given individual RGBA values.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    /// Create a new color, given RGB values. For this
    /// method, alpha is set to `1.0`.
    pub const fn new_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    /// Create the RGBA color from hex, provided as a `u32`.
    ///
    /// # Usage
    /// ```
    /// let transparent_firebrick = Color::hex(0xB2222280);
    /// ```
    pub fn hex(hex: u32) -> Color {
        let r = (hex & 0xFF000000) >> 24;
        let g = (hex & 0x00FF0000) >> 16;
        let b = (hex & 0x0000FF00) >> 8;
        let a = hex & 0x000000FF;

        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Create the RGBA color from hex, with a constant
    /// alpha value of `1.0`.
    ///
    /// # Usage
    /// ```
    /// let cornflower_blue = Color::hex_rgb(0x6495ED);
    /// ```
    pub fn hex_rgb(hex: u32) -> Color {
        let r = (hex & 0xFF0000) >> 16;
        let g = (hex & 0x00FF00) >> 8;
        let b = hex & 0x0000FF;

        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Color {
        Color::hex(hex)
    }
}

// impl Into<u32> for Color {
//     fn into(self) -> u32 {

//     }
// }

impl From<[f32; 4]> for Color {
    fn from(slice: [f32; 4]) -> Color {
        let [r, g, b, a] = slice;

        Color { r, g, b, a }
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        let Color { r, g, b, a } = self;

        [r, g, b, a]
    }
}

impl Into<wgpu_native::Color> for Color {
    fn into(self) -> wgpu_native::Color {
        wgpu_native::Color {
            r: self.r as _,
            g: self.g as _,
            b: self.b as _,
            a: self.a as _,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color;
    use approx::assert_relative_eq;

    #[test]
    fn hex() {
        assert_eq!(Color::hex(0x00000000), Color::new(0.0, 0.0, 0.0, 0.0));
        assert_eq!(Color::hex(0xFFFFFFFF), Color::new(1.0, 1.0, 1.0, 1.0));
        assert_relative_eq!(Color::hex(0x40000000).r, 0.25, max_relative = 0.01);
    }

    #[test]
    fn hex_rgb() {
        assert_eq!(Color::hex_rgb(0x000000), Color::new(0.0, 0.0, 0.0, 1.0));
        assert_eq!(Color::hex_rgb(0xFFFFFF), Color::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(Color::hex_rgb(0xFF00FF), Color::new(1.0, 0.0, 1.0, 1.0));
    }
}
