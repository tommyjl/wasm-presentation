use std::ops::Range;

pub fn md_to_html(input: String) -> String {
    Parser::new(input).parse().to_html()
}

pub fn md_to_slides(input: String) -> Slides {
    let mut slides = Slides::new();

    let parser = Parser::new(input).parse();
    let mut next_slide: Option<Slide> = None;

    for node in parser.nodes.iter() {
        match node {
            Node::Heading { level: _, range } => {
                if let Some(slide) = next_slide {
                    slides.slides.push(slide);
                    next_slide = None;
                }
                if next_slide.is_none() {
                    let title = parser.input[range.clone()].to_string();

                    next_slide = Some(Slide {
                        title,
                        ..Default::default()
                    });
                }
            }
            Node::Paragraph { range } => {
                if next_slide.is_none() {
                    next_slide = Some(Default::default());
                }
                if let Some(ref mut slide) = next_slide {
                    let paragraph = parser.input[range.clone()].to_string();
                    slide.paragraphs.push(paragraph);
                }
            }
        }
    }

    if let Some(slide) = next_slide {
        slides.slides.push(slide);
    }

    slides
}

#[derive(Default)]
pub struct Slide {
    title: String,
    paragraphs: Vec<String>,
}

impl Slide {
    pub fn to_html(&self) -> String {
        let mut result = String::new();

        result.push_str(&format!("<h2>{}</h2>\n", self.title));
        for p in self.paragraphs.iter() {
            result.push_str(&format!("<p>{}</p>\n", p));
        }

        result
    }
}

pub struct Slides {
    current: i32,
    slides: Vec<Slide>,
}

impl Slides {
    pub fn new() -> Self {
        Self {
            current: -1,
            slides: Vec::new(),
        }
    }

    pub fn next(&mut self) -> Option<&Slide> {
        if self.current < (self.slides.len() as i32) - 1 {
            self.current += 1;
        }
        self.slides.get(self.current as usize)
    }

    pub fn previous(&mut self) -> Option<&Slide> {
        if self.current > 0 {
            self.current -= 1;
        }
        self.slides.get(self.current as usize)
    }
}

struct Parser {
    input: String,
    scanner: Scanner,
    nodes: Vec<Node>,
}

impl Parser {
    fn new(input: String) -> Self {
        Self {
            scanner: Scanner::new(input.clone()),
            input,
            nodes: Vec::new(),
        }
    }

    fn parse(mut self) -> Self {
        loop {
            if let Some(heading) = self.parse_heading() {
                self.nodes.push(heading);
            } else if let Some(paragraph) = self.parse_paragraph() {
                self.nodes.push(paragraph);
            } else if self.scanner.peek() == Some(b'\n') {
                self.scanner.advance();
                continue;
            } else if self.scanner.peek() == None {
                break;
            } else {
                panic!("Parser did not reach the end of the input");
            }
        }
        self
    }

    fn parse_heading(&mut self) -> Option<Node> {
        self.scanner.save_cursor();

        let level_start = self.scanner.cursor;
        self.scanner.take_while(|c| c == b'#');
        let level_end = self.scanner.cursor;

        let level = level_end - level_start;
        if !(level_end > level_start && level < 7) {
            self.scanner.restore_cursor();
            return None;
        }

        self.scanner.take_while(|c| c == b' ');

        let start = self.scanner.cursor;
        if !self.scanner.take_while(|c| c != b'\n') {
            self.scanner.restore_cursor();
            return None;
        }
        let end = self.scanner.cursor;

        Some(Node::Heading {
            level,
            range: start..end,
        })
    }

    fn parse_paragraph(&mut self) -> Option<Node> {
        self.scanner.save_cursor();

        let start = self.scanner.cursor;
        if !self.scanner.take_while(|ch| ch != b'\n') {
            self.scanner.restore_cursor();
            return None;
        }
        let end = self.scanner.cursor;

        Some(Node::Paragraph { range: start..end })
    }

    fn to_html(&self) -> String {
        let mut result = String::new();

        for node in self.nodes.iter() {
            match node {
                Node::Heading { level, range } => {
                    result.push_str(&format!("<h{level}>"));
                    result.push_str(self.input.get(range.clone()).unwrap());
                    result.push_str(&format!("</h{level}>\n"));
                }
                Node::Paragraph { range } => {
                    result.push_str("<p>");
                    result.push_str(self.input.get(range.clone()).unwrap());
                    result.push_str("</p>\n");
                }
            }
        }

        result
    }
}

struct Scanner {
    chars: Vec<u8>,
    cursor: usize,
    saved_cursor: usize,
}

impl Scanner {
    fn new(input: String) -> Self {
        Self {
            chars: input.into_bytes(),
            cursor: 0,
            saved_cursor: 0,
        }
    }

    fn peek(&mut self) -> Option<u8> {
        if self.cursor < self.chars.len() {
            let output = *self.chars.get(self.cursor).unwrap();
            Some(output)
        } else {
            None
        }
    }

    fn advance(&mut self) {
        debug_assert!(self.cursor < self.chars.len());
        self.cursor += 1;
    }

    fn take_while(&mut self, func: impl Fn(u8) -> bool) -> bool {
        let start = self.cursor;

        loop {
            if let Some(ch) = self.chars.get(self.cursor) {
                if func(*ch) {
                    self.advance();
                } else {
                    break self.cursor > start;
                }
            } else {
                break self.cursor > start;
            }
        }
    }

    fn save_cursor(&mut self) {
        self.saved_cursor = self.cursor;
    }

    fn restore_cursor(&mut self) {
        self.cursor = self.saved_cursor;
    }
}

enum Node {
    Heading { level: usize, range: Range<usize> },
    Paragraph { range: Range<usize> },
}
