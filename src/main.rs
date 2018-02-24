extern crate clap;
use clap::App;

fn main() {
    App::new("rxxma")
       .version("0.0.1")
       .about("Amxmodx plugin reverse utility")
       .author("Fedcomp")
       .get_matches();
}
