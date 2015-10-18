extern crate playground;
extern crate docopt;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::channel;

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

    let colors = args.get_vec("<color_name>");
    let color_remote_storage = Arc::new(ColorRemoteStorage::new());

    let (tx, rx) = channel();

    for color in colors.iter() {
        let color_remote_storage = color_remote_storage.clone();
        let color = color.to_string();
        let tx = tx.clone();
        
        thread::spawn( move || {
            let color = Color::new(&color, &color_remote_storage);

            tx.send(color).unwrap();
        });
    }

    for _ in colors {
        println!("{}", rx.recv().unwrap())
    }
}
