use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Section {
    Definitions, 
    Rules, 
    Subroutines,
}

impl Section {
    fn next(&mut self) {
        match self {
            Section::Definitions => *self = Section::Rules,
            Section::Rules => *self = Section::Subroutines,
            _ => *self = Section::Subroutines,
        }
    }
}

#[derive(PartialEq)]
pub enum Delimiters {
    SectionDelimiter, 
    CodeBlockDelimiter, 
    CodeBlockClosingDelimiter,
    WordDelimiter,
    WhiteSpace,
    LineBreak
}

impl Delimiters {
    fn from(c: char) -> Self {
        match c {
            '%' => Delimiters::SectionDelimiter,
            '{' => Delimiters::CodeBlockDelimiter,
            '}' => Delimiters::CodeBlockClosingDelimiter,
            ' ' | '\t' => Delimiters::WhiteSpace,
            '\n'  => Delimiters::LineBreak,
            _ => Delimiters::WordDelimiter
        }
    }
}

pub struct LexSpecParser{
    source: String,
    section: Section,
    lines: u64,
    percent: bool,
    code_block: bool,
    word: String,
    definitions: [String; 2],
    name_dict: HashMap<String, String>,
    rules: Vec<(String,String)>,
}

//static Functions
impl LexSpecParser{
    pub fn new(src: String) -> Self {
        LexSpecParser{
            source: src,
            section: Section::Definitions,
            lines: 1,
            percent: false,
            code_block: false,
            word: String::new(),
            definitions: [String::new(),String::new()],
            name_dict: HashMap::new(),
            rules: vec!(),
        }
    }
}

//Public Methods
impl LexSpecParser{
    pub fn get_section(&self) -> Section{
        self.section
    }

    pub fn read(&mut self, target: &str) {
        for char in target.chars() {
            match Delimiters::from(char) {
                Delimiters::SectionDelimiter => self.percent_handler(),
                Delimiters::CodeBlockDelimiter => self.code_block_handler(),
                Delimiters::CodeBlockClosingDelimiter => self.close_code_block_handler(),
                Delimiters::WordDelimiter => self.word_handler(char),
                Delimiters::WhiteSpace => self.white_space_handler(),
                Delimiters::LineBreak => {},
            }
        }
        self.end_line_handler();
        self.increment_line();
    }

    pub fn print_error(&self, message: &str) {
        eprintln!("{}:{}: {}", self.source, self.lines, message);
    }
}

//Private Methods
impl LexSpecParser {
    fn next_section(&mut self){
        self.section.next();
    }

    fn set_percent(&mut self){
        self.percent = true;
    }

    fn unset_percent(&mut self){
        self.percent = false;
    }

    fn set_code_block(&mut self){
        self.code_block = true;
    }

    fn unset_code_block(&mut self){
        self.code_block = false;
        self.unset_percent();
    }

    fn increment_line(&mut self){
        self.lines += 1;
    }

    fn percent_handler(&mut self){
        if self.code_block == false {
            if self.percent == true {
                self.next_section(); self.unset_percent();
            }
            else {
                self.set_percent();
            }
        }
    }

    fn code_block_handler(&mut self){
        if self.section == Section::Definitions
        {
            if self.percent == true{
            self.set_code_block()
            }
        }
    }

    fn close_code_block_handler(&mut self){
        if self.section == Section::Definitions {
            if self.percent == true{
                self.unset_code_block()
            }
        }
    }

    fn is_defining_word(&self) -> bool{
        !self.word.is_empty()
    }

    fn word_handler(&mut self, c: char){
        for entry in &self.definitions {
            if entry.is_empty() {
                self.word.push(c);
                break;
            }
        }
    }

    fn white_space_handler(&mut self){
        if self.definitions[0].is_empty() && self.is_defining_word(){
            self.definitions[0]= std::mem::take(&mut self.word);
        }
    }

    fn end_line_handler(&mut self){
        let [key, mut value] = std::mem::take(&mut self.definitions);

        if key.is_empty(){
            if self.word.is_empty(){
                return ;
            }
            else {
                self.word.clear();
                self.print_error("incomplete name definition");
                return ;
            }
        }
        if value.is_empty(){
            if self.word.is_empty(){
                self.print_error("incomplete name definition");
                return ;
            }
            else {
                value = std::mem::take(&mut self.word);
            }
        }
        if self.name_dict.contains_key(&key) {
            self.print_error("name defined twice");
        } 
        else {
            self.name_dict.insert(key, value);
        }
    }

}