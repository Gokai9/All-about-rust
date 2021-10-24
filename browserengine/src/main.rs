mod html;

//use html;
fn main() {
    let mut x = html::ParseHtml {
        input: String::from("<html>"),
        pos: 0,
    };
    let y = x.parse_tag();
    println!("{}", y);
}
