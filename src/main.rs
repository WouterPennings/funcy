use std::fs::File;
use std::io::prelude::*;

mod ast;
pub mod compiler;
pub mod lexer;
pub mod parser;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let contents = std::fs::read_to_string(args[1].as_str());

    if let Err(_) = contents {
        println!("ERROR: Could not read file");
    } else if let Ok(input) = contents {
        let mut l = Lexer::new(input);
        let tokens = l.lex();

        let mut p = Parser::new(tokens);
        let program = p.parse();

        let mut compiler = Compiler::new(program);
        compiler.compile();

        if args.len() < 3 || args[2] == "-com" {
            let output_file = format!("{}.c", args[1].as_str());
            let file = File::create(&output_file);
            if let Ok(mut f) = file {
                let _ = f.write_all(compiler.code.as_bytes());
                println!("Compiled successfully, saved at: {}", output_file);
            } else {
                println!("Could not save compiled file");
            }
        } else if args[2] == "-d" {
            println!("C code:\n{}", compiler.code);
        } else {
            println!("Unknown flag: {}", args[2]);
        }
    }
}
