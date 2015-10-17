extern crate playground;
extern crate docopt;

use playground::core::color::Color;
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
   
    let colors: Vec<_> = colors.into_iter().map(|color| Color::new(color)).collect();

    for color in colors {
        println!("{}", color)
    }
}
