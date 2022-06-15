#[derive(Debug, PartialEq, Clone)]
pub enum OpCode {
    // Types
    Number(i32),
    Pointer(i32),
    PointerPointer(i32),
    Label(String),
    Literal(String),
    Function(String),
    Return,
    Call,
    Add,
    Sub,
    Mul,
    Store,
    Print,
    Write,
    Jump,
    Equal,
    Greater,
}

pub struct Lexer {
    code: String,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        // Adding a standard library of sorts
        let mut code = "
.print_str
    start:
    eql @@0 0
    jmp stop
    write @@0
    add @0 1
    jmp start
    stop:
    ret\n\n".to_string();
        code.push_str(input.as_str());
        Lexer { code }
    }

    pub fn lex(&mut self) -> Vec<OpCode> {
        let mut tokens: Vec<OpCode> = vec![];

        // Cleaning up input text
        self.remove_comments();
        self.code = self.code.replace('\n', " ").replace('\r', " ");

        let split = self.code.split(" ").collect::<Vec<&str>>();
        let words = split
            .iter()
            .enumerate()
            .filter_map(|(_, &r)| if r.len() > 0 { Some(r) } else { None })
            .collect::<Vec<&str>>();

        for word in words {
            match word {
                "store" => tokens.push(OpCode::Store),
                "print" => tokens.push(OpCode::Print),
                "write" => tokens.push(OpCode::Write),
                "call" => tokens.push(OpCode::Call),
                "set" => tokens.push(OpCode::Store),
                "add" => tokens.push(OpCode::Add),
                "sub" => tokens.push(OpCode::Sub),
                "mul" => tokens.push(OpCode::Mul),
                "jmp" => tokens.push(OpCode::Jump),
                "eql" => tokens.push(OpCode::Equal),
                "grt" => tokens.push(OpCode::Greater),
                "ret" => tokens.push(OpCode::Return),
                _ => {
                    if word.chars().nth(0).unwrap() == '@' {
                        // TODO: Check whether pointer is an integer
                        if word.chars().nth(1).unwrap() == '@' {
                            tokens.push(OpCode::PointerPointer(
                                word[2..].to_string().parse::<i32>().unwrap(),
                            ))
                        } else {
                            tokens.push(OpCode::Pointer(
                                word[1..].to_string().parse::<i32>().unwrap(),
                            ))
                        }
                    } else if word.chars().last() == Some(':') {
                        let mut s = word.to_string();
                        s.pop();
                        tokens.push(OpCode::Label(s))
                    } else if word.chars().nth(0).unwrap() == '.' {
                        tokens.push(OpCode::Function(word[1..].to_string()));
                    } else if self.valid_literal(word.to_string()) {
                        tokens.push(OpCode::Literal(word.to_string()))
                    } else {
                        // TODO: Check whether number is an integer
                        tokens.push(OpCode::Number(word.to_string().parse::<i32>().unwrap()))
                    }
                }
            }
        }

        tokens
    }

    fn remove_comments(&mut self) {
        let mut start: i32 = -1;
        let mut i: i32 = 0;
        while i < self.code.len() as i32 {
            if self.code.chars().nth(i as usize).unwrap() == ';' {
                start = i;
            }
            if start >= 0 && self.code.chars().nth(i as usize).unwrap() == '\n' {
                self.code.replace_range(start as usize..i as usize, "");
                i = start;
                start = -1;
            }
            i += 1;
        }
    }

    fn valid_literal(&self, literal: String) -> bool {
        for c in literal.chars() {
            if !c.is_alphabetic() && c != '_' {
                return false
            }
        }
        true
    }
}
