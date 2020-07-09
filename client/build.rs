use inline_assets;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("build rs");
    let inlined_html =
        inline_assets::inline_file("www/client.html", inline_assets::Config::default()).unwrap();
    let mut file = File::create("www/index.html").unwrap();
    file.write_all(inlined_html.as_bytes()).unwrap();
}
