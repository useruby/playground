extern crate playground;
extern crate docopt;

use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc, Mutex};

use playground::core::color::Color;
use playground::core::storage::ColorRemoteStorage;
use docopt::Docopt;

const USAGE: &'static str = "
Get color code

Usage:
  playground <color_name>...
";

fn main() {
    let args = Docopt::new(USAGE)
                        .and_then(|dopt| dopt.parse())
                        .unwrap_or_else(|e| e.exit());

    let color_remote_storage = Arc::new(ColorRemoteStorage::new());

    let colors: Arc<Mutex<Vec<Color>>> = Arc::new(Mutex::new(Vec::new()));

    for thread in args.get_vec("<color_name>").iter().map(|color_name| {
        let shared_colors = colors.clone();
        let color_remote_storage = color_remote_storage.clone();
        let color_name = color_name.to_string();
        
        thread::spawn( move || {
            let color = Color::new(&color_name, &color_remote_storage);

            let mut shared_colors = shared_colors.lock().unwrap();
            shared_colors.push(color)
        })
    }).collect::<Vec<JoinHandle<_>>>(){
        thread.join().unwrap();
    }

    for color in colors.lock().unwrap().iter() {
        println!("{}", color)
    }
}
