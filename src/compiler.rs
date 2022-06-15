use crate::ast::{Instruction, Program, Values};

#[derive(Clone)]
pub struct Compiler {
    pub code: String,
    imports: Vec<String>,
    functions: Vec<String>,
    program: Program,
    cursor: i32,
    current_function: String,
}

impl Compiler {
    pub fn new(program: Program) -> Compiler {
        Compiler {
            program,
            cursor: 0,
            imports: vec!["#include<stdio.h>".to_string()],
            functions: vec![],
            current_function: String::from(""),
            code: String::from(""),
        }
    }

    pub fn compile(&mut self) {
        while self.cursor < self.program.instructions.len() as i32 {
            let stat = self.program.instructions[self.cursor as usize].clone();

            self.compile_statement(stat);
            self.cursor += 1;
        }
        // Adding the imports to the compiled program.
        let mut imports = String::new();
        for import in self.imports.clone() {
            imports.push_str(import.as_str());
            imports.push_str("\n");
        }

        // Defining memory
        let memory = "// Instantiating of the emulated memory\nint MEMORY[255];\n";

        // Adding the user defined functions to the program.
        let mut funcs = String::new();
        for func in self.functions.clone() {
            funcs.push_str(func.as_str());
            funcs.push_str("\n\n");
        }

        self.code = format!("{}\n{}\n{}\n{}", imports, memory, funcs, self.code);
    }

    fn compile_statement(&mut self, stat: Instruction) {
        match stat {
            Instruction::Print(value) => {
                self.add_code_str("\tprintf(\"%d\\n\", ");
                self.add_code(format!("{});\n", value.generate_code()))
            }
            Instruction::Write(value) => {
                self.add_code_str("\tprintf(\"%c\", ");
                self.add_code(format!("{});\n", value.generate_code()))
            }
            Instruction::Expression(op, left, right) => {
                let op_str = format!("{}", op);
                self.add_code(format!(
                    "\t{} = {} {} {};\n",
                    left.generate_code(), left.generate_code(), op_str, right.generate_code()
                ));
            }
            Instruction::Store(ref loc, ref value) => {
                self.add_code(format!("\t{} = {};\n", loc.generate_code(), value.generate_code()));
            }
            Instruction::Label(ident) => {
                if !ident.compare(Values::Identifier(String::new())) {
                    panic!("A label needs to be an identifier")
                }
                self.add_code(format!("\t{}:\n", ident.generate_code()));
            }
            Instruction::Jump(ident) => {
                if !ident.compare(Values::Identifier(String::new())) {
                    panic!("A goto needs to have an identifer")
                }
                self.add_code(format!("\tgoto {};\n", ident.generate_code()))
            }
            Instruction::Compare(op, left, right) => {
                self.add_code(format!("\tif({} {} {})", left.generate_code(), op, right.generate_code()));
                self.compile_statement(
                    self.program.instructions[(self.cursor + 1) as usize].clone(),
                );
                self.cursor += 1;
            }
            Instruction::Function(ident) => {
                if !ident.compare(Values::Identifier(String::new())) {
                    panic!("Functions needs to be an identifier")
                }
                if self.current_function == "" {
                    self.current_function = format!("{}", ident.generate_code());
                    self.functions.push(format!("void {}() {{\n", ident.generate_code()));
                } else {
                    panic!("You cant define a function inside a function");
                }
            }
            Instruction::Call(ident) => {
                if !ident.compare(Values::Identifier(String::new())) {
                    panic!("Call needs to be a identifer")
                }
                self.add_code(format!("\t{}();\n", ident.generate_code()))
            }
            Instruction::Return => {
                self.add_code_str("\treturn;\n}");
                self.current_function = "".to_string();
            }
        }
    }

    fn add_code(&mut self, code: String) {
        self.add_code_str(code.as_str());
    }

    fn add_code_str(&mut self, code: &str) {
        if self.current_function == "" {
            self.code.push_str(format!("{}", code.to_string()).as_str());
        } else {
            let mut func = self.functions.pop().unwrap();
            func.push_str(format!("{}", code.to_string()).as_str());
            self.functions.push(func);
        }
    }
}
