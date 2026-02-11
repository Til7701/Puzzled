use adw::gdk::RGBA;

pub const WARNING_BG_LIGHT: RGBA = from_hex(0xe5a50a);
pub const WARNING_BG_DARK: RGBA = from_hex(0xcd9309);

pub const ERROR_BG_LIGHT: RGBA = from_hex(0xe01b24);
pub const ERROR_BG_DARK: RGBA = from_hex(0xc01c28);

pub const BLUE_1: RGBA = from_hex(0x99c1f1);
pub const BLUE_2: RGBA = from_hex(0x62a0ea);
pub const BLUE_3: RGBA = from_hex(0x3584e4);
pub const BLUE_4: RGBA = from_hex(0x1c71d8);
pub const BLUE_5: RGBA = from_hex(0x1a5fb4);
pub const GREEN_1: RGBA = from_hex(0x8ff0a4);
pub const GREEN_2: RGBA = from_hex(0x57e389);
pub const GREEN_3: RGBA = from_hex(0x33d17a);
pub const GREEN_4: RGBA = from_hex(0x2ec27e);
pub const GREEN_5: RGBA = from_hex(0x26a269);
pub const YELLOW_1: RGBA = from_hex(0xf9f06b);
pub const YELLOW_2: RGBA = from_hex(0xf8e45c);
pub const YELLOW_3: RGBA = from_hex(0xf6d32d);
pub const YELLOW_4: RGBA = from_hex(0xf5c211);
pub const YELLOW_5: RGBA = from_hex(0xe5a50a);
pub const ORANGE_1: RGBA = from_hex(0xffbe6f);
pub const ORANGE_2: RGBA = from_hex(0xffa348);
pub const ORANGE_3: RGBA = from_hex(0xff7800);
pub const ORANGE_4: RGBA = from_hex(0xe66100);
pub const ORANGE_5: RGBA = from_hex(0xc64600);
pub const RED_1: RGBA = from_hex(0xf66151);
pub const RED_2: RGBA = from_hex(0xed333b);
pub const RED_3: RGBA = from_hex(0xe01b24);
pub const RED_4: RGBA = from_hex(0xc01c28);
pub const RED_5: RGBA = from_hex(0xa51d2d);
pub const PURPLE_1: RGBA = from_hex(0xdc8add);
pub const PURPLE_2: RGBA = from_hex(0xc061cb);
pub const PURPLE_3: RGBA = from_hex(0x9141ac);
pub const PURPLE_4: RGBA = from_hex(0x813d9c);
pub const PURPLE_5: RGBA = from_hex(0x613583);
pub const BROWN_1: RGBA = from_hex(0xcdab8f);
pub const BROWN_2: RGBA = from_hex(0xb5835a);
pub const BROWN_3: RGBA = from_hex(0x986a44);
pub const BROWN_4: RGBA = from_hex(0x865e3c);
pub const BROWN_5: RGBA = from_hex(0x63452c);
pub const LIGHT_1: RGBA = from_hex(0xffffff);
pub const LIGHT_2: RGBA = from_hex(0xf6f5f4);
pub const LIGHT_3: RGBA = from_hex(0xdeddda);
pub const LIGHT_4: RGBA = from_hex(0xc0bfbc);
pub const LIGHT_5: RGBA = from_hex(0x9a9996);
pub const DARK_1: RGBA = from_hex(0x77767b);
pub const DARK_2: RGBA = from_hex(0x5e5c64);
pub const DARK_3: RGBA = from_hex(0x3d3846);
pub const DARK_4: RGBA = from_hex(0x241f31);
pub const DARK_5: RGBA = from_hex(0x000000);

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
