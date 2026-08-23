use std::fs;
use std::path::Path;

mod veloct_compiler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Veloct Compiler CLI (vctc)");
        println!("Usage: vctc <file.vct> [-o output.vctb]");
        return;
    }

    let input_file = &args[1];
    println!("Reading Veloct source file: {}", input_file);

    let source_code = fs::read_to_string(input_file)
        .expect("Failed to read input .vct file")
        .chars()
        .collect::<Vec<char>>();

    let lexer = veloct_compiler::Lexer::new(&source_code);
    let mut parser = veloct_compiler::Parser::new(lexer);
    let ast = parser.parse_module();

    let mut compiler = veloct_compiler::VeloctCompiler::new();
    let opcodes = compiler.compile_module(&ast);

    println!("Successfully compiled module '{}' into {} opcodes.", ast.name, opcodes.len());
}
