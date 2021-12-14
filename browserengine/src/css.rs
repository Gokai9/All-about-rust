// h1,h2,h3 { margin: auto; color: red; }
// div.note { margin-bottom: 20px; padding: 10px; }
// #answer { display: none; }

#[derive(Debug)]
pub struct StyleSheet {
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct Rule {
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>,
}
#[derive(Debug)]
enum Selector {
    Simple(SimpleSelector),
}
#[derive(Debug)]
struct SimpleSelector {
    id: Option<String>,
    class: Vec<String>,
    tag_name: Option<String>,
}
#[derive(Debug)]
struct Declaration {
    name: String,
    value: Value,
}
#[derive(Debug)]
enum Value {
    Keyword(String),
    Length(f32, Unit),
    Color(ColorValue),
}
#[derive(Debug)]
enum Unit {
    Px,
    Rem,
    Persen,
}
#[derive(Debug)]
struct ColorValue {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

type Specificity = (usize, usize, usize);
impl Selector {
    fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;

        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();

        (a, b, c)
    }
}

pub fn parse(source: String) -> StyleSheet {
    let mut par = ParseCss {
        pos: 0,
        input: source,
    };
    StyleSheet {
        rules: par.parse_rules(),
    }
}

struct ParseCss {
    pos: usize,
    input: String,
}

impl ParseCss {
    fn parse_rules(&mut self) -> Vec<Rule> {
        let mut rules = Vec::new();
        loop {
            if self.eof() {
                break;
            }
            self.skip_whitespace();
            rules.push(self.parse_rule())
        }
        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations(),
        }
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.skip_whitespace();
            match self.next_char() {
                ',' => {
                    self.consume_char();
                }
                '{' => {
                    break;
                }
                c => panic!("Unexpected character {} in selector list", c),
            }
        }
        selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));

        selectors
    }
    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {
            id: None,
            class: Vec::new(),
            tag_name: None,
        };
        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }
        selector
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut dec = Vec::new();
        assert_eq!(self.consume_char(), '{');
        loop {
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            dec.push(self.parse_declaration());
        }
        dec
    }
    fn parse_declaration(&mut self) -> Declaration {
        self.skip_whitespace();
        let name = self.parse_identifier();
        //assert_eq!(self.consume_char(), ':');
        self.skip_whitespace();
        let val = self.parse_value();
        self.skip_whitespace();
        //assert_eq!(self.consume_char(), ';');
        //println!("{}, {:?}", name, val);
        Declaration {
            name: name,
            value: val,
        }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..='9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier()),
        }
    }

    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }
    fn parse_float(&mut self) -> f32 {
        let l = self.consume_while(|c| match c {
            '0'..='9' | '.' => true,
            _ => false,
        });
        l.parse().unwrap()
    }
    fn parse_unit(&mut self) -> Unit {
        match &*self.parse_identifier().to_ascii_lowercase() {
            "px" => Unit::Px,
            "rem" => Unit::Rem,
            "%" => Unit::Persen,
            _ => panic!("unrecognized unit"),
        }
    }

    fn parse_color(&mut self) -> Value {
        self.consume_char();
        Value::Color(ColorValue {
            r: self.parse_hex(),
            g: self.parse_hex(),
            b: self.parse_hex(),
            a: 255,
        })
    }
    fn parse_hex(&mut self) -> u8 {
        let c = &self.input[self.pos..self.pos + 2];
        self.pos += 2;
        u8::from_str_radix(c, 16).unwrap()
    }
    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier)
    }

    fn skip_whitespace(&mut self) {
        while self.next_char() == ' ' || self.next_char() == '\n' {
            self.consume_char();
        }
    }

    fn consume_while<F>(&mut self, bok: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        while !self.eof() && bok(self.next_char()) {
            res.push(self.consume_char());
        }
        res
    }
    fn eof(&mut self) -> bool {
        self.pos >= self.input.len()
    }
    fn next_char(&mut self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, current) = iter.next().unwrap();
        let (pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += pos;
        current
    }
}

fn valid_identifier(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
        _ => false,
    }
}
