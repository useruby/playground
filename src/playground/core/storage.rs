extern crate hyper;
extern crate regex;

use std::io::Read;

use self::hyper::Client;
use self::hyper::header::Connection;

use self::regex::Regex;

pub struct ColorRemoteStorage {
    url: String,
    regex: Regex,
    client: Client
}

impl ColorRemoteStorage {
    pub fn new() -> ColorRemoteStorage {
        let re = Regex::new("rgb\\((\\d+), (\\d+), (\\d+)\\)").unwrap();
        let client = Client::new();

        ColorRemoteStorage{url: "http://rgb.to/".to_string(), regex: re, client: client}
    }
 
    pub fn get(&self, color_name: &str) -> (u8, u8, u8) {
        let mut res = self.client.get(&(self.build_url(color_name)))
            .header(Connection::close())
            .send().unwrap();

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        let caps = self.regex.captures(&body).unwrap();

        (
            caps.at(1).unwrap().parse::<u8>().unwrap(), 
            caps.at(2).unwrap().parse::<u8>().unwrap(), 
            caps.at(3).unwrap().parse::<u8>().unwrap()
        )
    }

    fn build_url(&self, color_name: &str) -> String {
        self.url.clone() + color_name
    }
}

#[cfg(test)]
mod tests {
    use core::storage::ColorRemoteStorage;

    #[test]
    fn it_should_get_coral_color(){
        assert_eq!(ColorRemoteStorage::new().get("coral"), (255, 127, 80))
    }
}
