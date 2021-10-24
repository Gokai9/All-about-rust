//parse html

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
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
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
    //still confusing :(
    // fn consume_whitespace(&mut self) {
    //     self.consume_all(CharExt::is_whitespace);
    // }
    pub fn parse_tag(&mut self) -> String {
        self.consume_all(|ch| match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' => true,
            _ => false,
        })
    }
}

// fn main() {
//     let x = Parse{input: String::from("html"), pos: 2};
//     let y = x.nextchar();
//     println!("{}", y);
// }
