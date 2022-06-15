use std::fmt::Formatter;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            instructions: vec![],
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Function(Values),
    Label(Values),
    Call(Values),
    Expression(MathOperators, Values, Values),
    Store(Values, Values),
    Print(Values),
    Write(Values),
    Jump(Values),
    Compare(CompOperators, Values, Values),
    Return,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Values {
    Number(i32),
    Pointer(i32),
    PointerPointer(i32),
    Identifier(String), // String name, function name
}

impl std::fmt::Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Values::Number(n) => write!(f, "{}", n),
            Values::Pointer(ptr) => write!(f, "{}", ptr),
            Values::Identifier(ident) => write!(f, "{}", ident),
            Values::PointerPointer(ptrptr) => write!(f, "{}", ptrptr),
        }
    }
}

impl Values {
    pub fn compare(&self, other: Values) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(&other)
    }

    pub fn generate_code(&self) -> String {
        match self {
            Values::Number(n) => format!("{}", n),
            Values::Pointer(ptr) => format!("MEMORY[{}]", ptr),
            Values::Identifier(ident) => format!("{}", ident),
            Values::PointerPointer(ptrptr) => format!("MEMORY[MEMORY[{}]]", ptrptr),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MathOperators {
    Add,
    Subtract,
    Multiply,
}

impl std::fmt::Display for MathOperators {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MathOperators::Add => write!(f, "+"),
            MathOperators::Subtract => write!(f, "-"),
            MathOperators::Multiply => write!(f, "*"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CompOperators {
    Equal,
    Greater,
}

impl std::fmt::Display for CompOperators {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompOperators::Equal => write!(f, "=="),
            CompOperators::Greater => write!(f, ">"),
        }
    }
}
