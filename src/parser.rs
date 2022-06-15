use crate::ast::{CompOperators, Instruction, MathOperators, Program, Values};
use crate::lexer::OpCode;

pub struct Parser {
    tokens: Vec<OpCode>,
    cursor: i32,
}

impl Parser {
    pub fn new(tokens: Vec<OpCode>) -> Parser {
        Parser { tokens, cursor: -1 }
    }

    pub fn parse(&mut self) -> Program {
        let mut prog = Program::new();

        while self.next_token() {
            match self.current_token() {
                OpCode::Add | OpCode::Sub | OpCode::Mul => {
                    let op = self.current_token();
                    let loc = if self.expected(OpCode::Pointer(0))
                        || self.expected(OpCode::PointerPointer(0))
                    {
                        if let OpCode::Pointer(p) = self.current_token() {
                            Values::Pointer(p)
                        } else if let OpCode::PointerPointer(p) = self.current_token() {
                          Values::PointerPointer(p)
                        } else {
                            unreachable!("Should not be reachable")
                        }
                    } else {
                        panic!("Needed a pointer");
                    };
                    let value = if self.expected(OpCode::Pointer(0))
                        || self.expected(OpCode::PointerPointer(0))
                        || self.expected(OpCode::Number(1))
                    {
                        let t = self.current_token();
                        if let OpCode::Number(i) = t {
                            Values::Number(i)
                        } else if let OpCode::Pointer(p) = t {
                            Values::Pointer(p)
                        } else if let OpCode::PointerPointer(p) = t {
                            Values::PointerPointer(p)
                        } else {
                            unreachable!("Should not be reachable")
                        }
                    } else {
                        panic!("Needed a value or pointer")
                    };

                    let oc = match op {
                        OpCode::Add => MathOperators::Add,
                        OpCode::Mul => MathOperators::Multiply,
                        OpCode::Sub => MathOperators::Subtract,
                        _ => unreachable!("Only three operators"),
                    };
                    prog.add_instruction(Instruction::Expression(oc, loc, value))
                }
                OpCode::Store => {
                    let loc = if self.expected(OpCode::Pointer(0)) || self.expected(OpCode::PointerPointer(0)) {
                        if let OpCode::Pointer(p) = self.current_token() {
                            Values::Pointer(p)
                        } else if let OpCode::PointerPointer(p) = self.current_token() {
                          Values::PointerPointer(p)
                        } else {
                            unreachable!("Should not be reachable")
                        }
                    } else {
                        panic!("Needed a pointer");
                    };

                    let value =
                        if self.expected(OpCode::Pointer(0)) || self.expected(OpCode::PointerPointer(0)) || self.expected(OpCode::Number(1)) {
                            let t = self.current_token();
                            if let OpCode::Number(i) = t {
                                Values::Number(i)
                            } else if let OpCode::Pointer(p) = t {
                                Values::Pointer(p)
                            } else if let OpCode::PointerPointer(p) = t {
                                Values::PointerPointer(p)
                            } else {
                                unreachable!("Should not be reachable")
                            }
                        } else {
                            panic!("Needed a value or pointer")
                        };

                    prog.add_instruction(Instruction::Store(loc, value));
                }
                OpCode::Print | OpCode::Write => {
                    let op = self.current_token();
                    let loc = if self.expected(OpCode::Pointer(0)) || self.expected(OpCode::PointerPointer(0)) {
                        if let OpCode::Pointer(p) = self.current_token() {
                            Values::Pointer(p)
                        } else if let OpCode::PointerPointer(p) = self.current_token() {
                            Values::PointerPointer(p)
                        } else {
                            unreachable!("Should not be reachable")
                        }
                    } else {
                        panic!("Needed a pointer");
                    };

                    if let OpCode::Print = op {
                        prog.add_instruction(Instruction::Print(loc));
                    } else {
                        prog.add_instruction(Instruction::Write(loc));
                    }
                }
                OpCode::Equal | OpCode::Greater => {
                    let op = self.current_token();
                    let left = if self.expected(OpCode::Pointer(0)) || self.expected(OpCode::PointerPointer(0)) {
                        if let OpCode::Pointer(p) = self.current_token() {
                            Values::Pointer(p)
                        } else if let OpCode::PointerPointer(p) = self.current_token() {
                            Values::PointerPointer(p)
                        } else {
                            unreachable!("Should not be reachable")
                        }
                    } else {
                        panic!("Needed a pointer");
                    };

                    let right = if self.expected(OpCode::Pointer(0))
                        || self.expected(OpCode::PointerPointer(0))
                        || self.expected(OpCode::Number(1))
                    {
                        let t = self.current_token();
                        if let OpCode::Number(i) = t {
                            Values::Number(i)
                        } else if let OpCode::Pointer(p) = t {
                            Values::Pointer(p)
                        } else if let OpCode::PointerPointer(p) = t {
                            Values::PointerPointer(p)
                        } else {
                            unreachable!("Should not be reachable")
                        }
                    } else {
                        panic!("Needed a value or pointer")
                    };

                    if op == OpCode::Equal {
                        prog.add_instruction(Instruction::Compare(
                            CompOperators::Equal,
                            left,
                            right,
                        ));
                    } else {
                        prog.add_instruction(Instruction::Compare(
                            CompOperators::Greater,
                            left,
                            right,
                        ));
                    }
                }
                OpCode::Label(name) => {
                    prog.add_instruction(Instruction::Label(Values::Identifier(name)))
                }
                OpCode::Function(func) => {
                    prog.add_instruction(Instruction::Function(Values::Identifier(func)))
                }
                OpCode::Return => prog.add_instruction(Instruction::Return),
                OpCode::Jump | OpCode::Call => {
                    let oc = self.current_token();
                    let label = if self.expected(OpCode::Literal("".to_string())) {
                        if let OpCode::Literal(lit) = self.current_token() {
                            Values::Identifier(lit)
                        } else {
                            unreachable!("Should not be reachable")
                        }
                    } else {
                        panic!("Needed a label literal");
                    };

                    if let OpCode::Jump = oc {
                        prog.add_instruction(Instruction::Jump(label))
                    } else {
                        prog.add_instruction(Instruction::Call(label))
                    }
                }
                OpCode::Literal(_) => panic!("I never expect a bare literal"),
                OpCode::Number(_) => panic!("I never expect a bare number"),
                OpCode::Pointer(_) => panic!("I never expect a bare pointer"),
                OpCode::PointerPointer(_) => panic!("I never expect a bare pointerpointer"),
            }
        }

        prog
    }

    fn current_token(&mut self) -> OpCode {
        self.tokens[self.cursor as usize].clone()
    }

    fn peek_token(&mut self) -> Option<OpCode> {
        if (self.tokens.len() - 1) as i32 > self.cursor {
            let i = (self.cursor + 1) as usize;
            Some(self.tokens[i].clone())
        } else {
            None
        }
    }

    fn next_token(&mut self) -> bool {
        if (self.tokens.len() - 1) as i32 > self.cursor {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn expected(&mut self, op_code: OpCode) -> bool {
        if let Some(token) = &self.peek_token() {
            if std::mem::discriminant(&op_code) == std::mem::discriminant(token) {
                self.next_token();
                true
            } else {
                false
            }
        } else {
            panic!("Peeked for token, but had none")
        }
    }
}
