use adw::gdk::RGBA;

pub const WARNING_BG_LIGHT: RGBA = from_hex(0xe5a50a);
pub const WARNING_BG_DARK: RGBA = from_hex(0xcd9309);

pub const ERROR_BG_LIGHT: RGBA = from_hex(0xe01b24);
pub const ERROR_BG_DARK: RGBA = from_hex(0xc01c28);

const fn from_hex(hex: u32) -> RGBA {
    let r = ((hex >> 16) & 0xFF) as f64 / 255.0;
    let g = ((hex >> 8) & 0xFF) as f64 / 255.0;
    let b = (hex & 0xFF) as f64 / 255.0;
    RGBA::new(r as f32, g as f32, b as f32, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex_r() {
        let color = from_hex(0xFF0000);
        assert_eq!(color.red(), 1.0);
        assert_eq!(color.green(), 0.0);
        assert_eq!(color.blue(), 0.0);
        assert_eq!(color.alpha(), 1.0);
    }

    #[test]
    fn test_from_hex_g() {
        let color = from_hex(0x00FF00);
        assert_eq!(color.red(), 0.0);
        assert_eq!(color.green(), 1.0);
        assert_eq!(color.blue(), 0.0);
        assert_eq!(color.alpha(), 1.0);
    }

    #[test]
    fn test_from_hex_b() {
        let color = from_hex(0x0000FF);
        assert_eq!(color.red(), 0.0);
        assert_eq!(color.green(), 0.0);
        assert_eq!(color.blue(), 1.0);
        assert_eq!(color.alpha(), 1.0);
    }

    #[test]
    fn test_from_hex_grey() {
        let color = from_hex(0x808080);
        assert_eq!(color.red(), 128.0 / 255.0);
        assert_eq!(color.green(), 128.0 / 255.0);
        assert_eq!(color.blue(), 128.0 / 255.0);
        assert_eq!(color.alpha(), 1.0);
    }
}
