extern crate hyper;
extern crate regex;

use std::fmt;
use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;

use self::regex::Regex;

#[derive(Debug)]
pub struct Color {
    name: String,
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(name: &str) -> Color {
        let (r, g, b) = match name {
            "red" => (255, 0, 0),
            "green" => (0, 0, 255),
            "blue" => (0, 255, 0),
            "purple" => (128, 0, 128),
            _ => Color::rgb_from_remote(name)
        };

        Color{name: name.to_string(), r: r, g: g, b: b}       
    }

    fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    fn hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    fn rgb_from_remote(name: &str) -> (u8, u8, u8) {
        // Create a client.
        let client = Client::new();

        // Creating an outgoing request.
        let mut res = client.get(&format!("http://rgb.to/{}", name))
            .header(Connection::close())
            .send().unwrap();

        // Read the Response.
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        let re = Regex::new("rgb\\((\\d+), (\\d+), (\\d+)\\)").unwrap();

        let caps = re.captures(&body).unwrap();

        (
            caps.at(1).unwrap().parse::<u8>().unwrap(), 
            caps.at(2).unwrap().parse::<u8>().unwrap(), 
            caps.at(3).unwrap().parse::<u8>().unwrap()
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {:?} - {}", self.name, self.rgb(), self.hex())
    }
}

#[cfg(test)]
pub mod test {
    use core::color::Color;

    #[test]
    fn red_color_should_has_rgb_255_0_0() {
        assert_eq!(Color::new("red").rgb(), (255, 0, 0))
    }

    #[test]
    fn red_color_should_has_hex_ff0000() {
        assert_eq!(Color::new("red").hex(), "#FF0000")
    }

    #[test]
    fn coral_color_should_has_rgb_255_127_80(){
        assert_eq!(Color::new("coral").rgb(), (255, 127, 80))
    }
}
