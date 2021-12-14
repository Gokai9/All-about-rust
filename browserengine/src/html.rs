//parse html
use crate::dom;
use std::collections::HashMap;

pub fn parse(source: String) -> dom::Node {
    let mut parser = ParseHtml {
        input: source,
        pos: 0,
    }
    .parse_nodes();
    if parser.len() == 1 {
        parser.swap_remove(0)
    } else {
        dom::element("html".to_string(), HashMap::new(), parser)
    }
}
#[derive(Debug)]
pub struct ParseHtml {
    pub input: String,
    pub pos: usize,
}

impl ParseHtml {
    //membaca setiap karakter didalam input
    pub fn nextchar(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
    //return true jika inputnya dimulai dengan argumen yang diberikan
    pub fn starts(&self, sr: &str) -> bool {
        self.input[self.pos..].starts_with(sr)
    }
    //return true jika semua input sudah dikonsumsi
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
    pub fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices(); //return index 0
        let (_, current_char) = iter.next().unwrap(); //index always start zero
        let (next_pos, _) = iter.next().unwrap_or((1, ' ')); //index start one
        self.pos += next_pos;
        current_char
    }
    fn consume_all<F>(&mut self, tes: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && tes(self.nextchar()) {
            result.push(self.consume_char())
        }
        result
    }
    fn skip_whitespace(&mut self) {
        self.consume_all(char::is_whitespace);
    }
    pub fn parse_tag(&mut self) -> String {
        self.consume_all(|ch| match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' => true,
            _ => false,
        })
    }
    fn parse_node(&mut self) -> dom::Node {
        match self.nextchar() {
            '<' => self.parse_element(),
            _ => self.parse_text(),
        }
    }
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.skip_whitespace();
            if self.eof() || self.starts("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }
    fn parse_element(&mut self) -> dom::Node {
        //opening tag
        assert!(self.consume_char() == '<');
        let tag_name = self.parse_tag();
        let attr = self.parse_attr();
        assert!(self.consume_char() == '>');

        let child = self.parse_nodes();

        assert!(self.consume_char() == '<');
        assert!(self.consume_char() == '/');
        assert!(self.parse_tag() == tag_name);
        assert!(self.consume_char() == '>');

        return dom::element(tag_name, attr, child);
    }
    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_all(|c| c != '<'))
    }
    fn parse_attr(&mut self) -> dom::Map {
        let mut attribute = HashMap::new();
        loop {
            self.skip_whitespace();
            if self.nextchar() == '>' {
                break;
            }
            let (name, value) = self.attr_name();
            attribute.insert(name, value);
        }
        return attribute;
    }
    fn attr_name(&mut self) -> (String, String) {
        let name = self.parse_tag();
        assert!(self.consume_char() == '=');
        let value = self.attr_value();
        return (name, value);
    }
    fn attr_value(&mut self) -> String {
        let quote = self.consume_char();
        assert!(quote == '"' || quote == '\'');
        let value = self.consume_all(|nextchar| nextchar != quote);
        assert!(quote == self.consume_char());
        return value;
    }
}

// fn main() {
//     let x = Parse{input: String::from("html"), pos: 2};
//     let y = x.nextchar();
//     println!("{}", y);
// }

//<div><p>text</p></div>
