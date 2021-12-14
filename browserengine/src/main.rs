pub mod css;
pub mod dom;
pub mod html;

use std::fs::File;
use std::io::Read;

//use html;
fn main() {
    let html = read_file("examples/index.html");
    let css = read_file("examples/style.css");
    let root = html::parse(html);
    let style = css::parse(css);
    println!("{:?}", root.nodetype);
    println!("{:?}", style)
}

fn read_file(filename: &str) -> String {
    let mut file = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut file)
        .unwrap();
    file
}
