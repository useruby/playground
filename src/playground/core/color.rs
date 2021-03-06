use std::fmt;
use core::storage::ColorRemoteStorage;

#[derive(Debug)]
pub struct Color {
    name: String,
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(name: &str, remote_storage: &ColorRemoteStorage) -> Color {
        let (r, g, b) = match name {
            "red" => (255, 0, 0),
            "green" => (0, 0, 255),
            "blue" => (0, 255, 0),
            "purple" => (128, 0, 128),
            _ => (*remote_storage).get(name)
        };

        Color{name: name.to_string(), r: r, g: g, b: b}       
    }

    fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {:?} - {}", self.name, self.rgb(), self.hex())
    }
}

#[cfg(test)]
mod test {
    use core::color::Color;
    use core::storage::ColorRemoteStorage;

    fn create_color(color_name: &str) -> Color{
        Color::new(color_name, &ColorRemoteStorage::new())
    }

    #[test]
    fn red_color_should_has_rgb_255_0_0() {
        assert_eq!(create_color("red").rgb(), (255, 0, 0))
    }

    #[test]
    fn red_color_should_has_hex_ff0000() {
        assert_eq!(create_color("red").hex(), "#FF0000")
    }

    #[test]
    fn coral_color_should_has_rgb_255_127_80(){
        assert_eq!(create_color("coral").rgb(), (255, 127, 80))
    }
}
