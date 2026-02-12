const COLORS: [ColorConfig; 35] = [
    ColorConfig::from_rgb_hex(0x1c71d8), // Blue 4
    ColorConfig::from_rgb_hex(0x2ec27e), // Green 4
    ColorConfig::from_rgb_hex(0xf5c211), // Yellow 4
    ColorConfig::from_rgb_hex(0xe66100), // Orange 4
    ColorConfig::from_rgb_hex(0xc01c28), // Red 4
    ColorConfig::from_rgb_hex(0x813d9c), // Purple 4
    ColorConfig::from_rgb_hex(0x865e3c), // Brown 4
    ColorConfig::from_rgb_hex(0x62a0ea), // Blue 2
    ColorConfig::from_rgb_hex(0x57e389), // Green 2
    ColorConfig::from_rgb_hex(0xf8e45c), // Yellow 2
    ColorConfig::from_rgb_hex(0xffa348), // Orange 2
    ColorConfig::from_rgb_hex(0xed333b), // Red 2
    ColorConfig::from_rgb_hex(0xc061cb), // Purple 2
    ColorConfig::from_rgb_hex(0xb5835a), // Brown 2
    ColorConfig::from_rgb_hex(0x1a5fb4), // Blue 5
    ColorConfig::from_rgb_hex(0x26a269), // Green 5
    ColorConfig::from_rgb_hex(0xe5a50a), // Yellow 5
    ColorConfig::from_rgb_hex(0xc64600), // Orange 5
    ColorConfig::from_rgb_hex(0x613583), // Purple 5
    ColorConfig::from_rgb_hex(0xa51d2d), // Red 5
    ColorConfig::from_rgb_hex(0x63452c), // Brown 5
    ColorConfig::from_rgb_hex(0x99c1f1), // Blue 1
    ColorConfig::from_rgb_hex(0x8ff0a4), // Green 1
    ColorConfig::from_rgb_hex(0xf9f06b), // Yellow 1
    ColorConfig::from_rgb_hex(0xffbe6f), // Orange 1
    ColorConfig::from_rgb_hex(0xf66151), // Red 1
    ColorConfig::from_rgb_hex(0xdc8add), // Purple 1
    ColorConfig::from_rgb_hex(0xcdab8f), // Brown 1
    ColorConfig::from_rgb_hex(0x3584e4), // Blue 3
    ColorConfig::from_rgb_hex(0x33d17a), // Green 3
    ColorConfig::from_rgb_hex(0xf6d32d), // Yellow 3
    ColorConfig::from_rgb_hex(0xff7800), // Orange 3
    ColorConfig::from_rgb_hex(0xe01b24), // Red 3
    ColorConfig::from_rgb_hex(0x9141ac), // Purple 3
    ColorConfig::from_rgb_hex(0x986a44), // Brown 3
];

#[derive(Debug, Clone, Copy)]
pub struct ColorConfig {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl ColorConfig {
    /// Creates a new ColorConfig.
    ///
    /// # Arguments
    ///
    /// * `red`: Red component (0-255).
    /// * `green`: Green component (0-255).
    /// * `blue`: Blue component (0-255).
    /// * `alpha`: Alpha component (0-255), where 0 is fully transparent and 255 is fully opaque.
    ///
    /// returns: ColorConfig
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> ColorConfig {
        ColorConfig {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a ColorConfig from a 24-bit RGB hex value.
    ///
    /// # Arguments
    ///
    /// * `hex`: A 24-bit RGB hex value in the format 0xRRGGBB.
    ///
    /// returns: ColorConfig with the specified RGB values and alpha set to 255 (fully opaque).
    pub const fn from_rgb_hex(hex: u32) -> ColorConfig {
        let red = ((hex >> 16) & 0xFF) as u8;
        let green = ((hex >> 8) & 0xFF) as u8;
        let blue = (hex & 0xFF) as u8;
        ColorConfig::new(red, green, blue, 255)
    }

    /// Returns a default ColorConfig based on the provided index.
    ///
    /// The index is used to select a color from a predefined list of colors.
    ///
    /// # Arguments
    ///
    /// * `index`: The index to select the color from the predefined list.
    ///
    /// returns: A ColorConfig from the predefined list based on the index.
    pub const fn default_with_index(index: usize) -> ColorConfig {
        COLORS[index % COLORS.len()]
    }

    pub const fn red(&self) -> u8 {
        self.red
    }

    pub const fn green(&self) -> u8 {
        self.green
    }

    pub const fn blue(&self) -> u8 {
        self.blue
    }

    pub const fn alpha(&self) -> u8 {
        self.alpha
    }
}

impl TryFrom<String> for ColorConfig {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(hex) = value.strip_prefix('#') {
            if hex.len() == 6 {
                let red = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| "Invalid hex color".to_string())?;
                let green = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| "Invalid hex color".to_string())?;
                let blue = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| "Invalid hex color".to_string())?;
                Ok(ColorConfig::new(red, green, blue, 255))
            } else if hex.len() == 8 {
                let red = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| "Invalid hex color".to_string())?;
                let green = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| "Invalid hex color".to_string())?;
                let blue = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| "Invalid hex color".to_string())?;
                let alpha = u8::from_str_radix(&hex[6..8], 16)
                    .map_err(|_| "Invalid hex color".to_string())?;
                Ok(ColorConfig::new(red, green, blue, alpha))
            } else {
                Err("Hex color must be in the format #RRGGBB or #RRGGBBAA".to_string())
            }
        } else {
            Err("Color string must start with '#'".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_config_from_rgb_hex() {
        let color = ColorConfig::from_rgb_hex(0x1c71d8);
        assert_eq!(color.red(), 28);
        assert_eq!(color.green(), 113);
        assert_eq!(color.blue(), 216);
        assert_eq!(color.alpha(), 255);
    }

    #[test]
    fn test_color_config_try_from_string() {
        let color = ColorConfig::try_from("#1c71d8".to_string()).unwrap();
        assert_eq!(color.red(), 28);
        assert_eq!(color.green(), 113);
        assert_eq!(color.blue(), 216);
        assert_eq!(color.alpha(), 255);

        let color_with_alpha = ColorConfig::try_from("#1c71d880".to_string()).unwrap();
        assert_eq!(color_with_alpha.red(), 28);
        assert_eq!(color_with_alpha.green(), 113);
        assert_eq!(color_with_alpha.blue(), 216);
        assert_eq!(color_with_alpha.alpha(), 128);

        let invalid_color = ColorConfig::try_from("1c71d8".to_string());
        assert!(invalid_color.is_err());

        let invalid_hex_length = ColorConfig::try_from("#1c71d".to_string());
        assert!(invalid_hex_length.is_err());
    }
}
