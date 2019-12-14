#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
/// RGBA color structure represented by `f32`s. Within
/// Peach, this structure is actually computed as SRGBA
/// because of [`wgpu`].
///
/// Each component is in the range [0.0, 1.0].
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
    pub const TRANSPARENT: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    };
    pub const BLACK: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const WHITE: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const YELLOW: Color = Color {
        r: 1.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
    pub const CYAN: Color = Color {
        r: 0.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    pub const BLUE: Color = Color {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const MAGENTA: Color = Color {
        r: 1.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };

    /// Create a new color, given individual RGBA values.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        let r = r.min(1.0).max(0.0);
        let g = g.min(1.0).max(0.0);
        let b = b.min(1.0).max(0.0);
        let a = a.min(1.0).max(0.0);

        Color { r, g, b, a }
    }

    /// Create a new color, given RGB values. For this
    /// method, alpha is set to `1.0`.
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Color {
        Color::new(r, g, b, 1.0)
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
    /// Create the RGBA from HSLA, where hue is in degrees
    /// from `0.0` to `360.0`. Saturation, lightness, and
    /// alpha are all from `0.0` to `1.0`.
    ///
    /// If the hue is greater than `360.0`, then modulo is
    /// applied.
    pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Color {
        // https://en.wikipedia.org/wiki/HSL_and_HSV#Alternative_HSL_to_RGB
        //
        // Replace `a` with `x` so it doesn't inerfere with alpha.
        let h = h.rem_euclid(360.0);

        let x = s * l.min(1.0 - l);
        let f = |n: f32| {
            let k = (n + h / 30.0).rem_euclid(12.0);
            l - x * (k - 3.0).min(9.0 - k).min(1.0).max(-1.0)
        };

        Color::new(f(0.0), f(8.0), f(4.0), a)
    }

    /// Same as calling `Color::hsla` with an alpha of 1.0.
    pub fn hsl(h: f32, s: f32, l: f32) -> Color {
        Color::hsla(h, s, l, 1.0)
    }
}

impl From<u32> for Color {
    fn from(hex: u32) -> Color {
        Color::hex(hex)
    }
}

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
