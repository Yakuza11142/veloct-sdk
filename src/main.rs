// ============================================================================
// MAIN RUNTIME EXECUTOR
// ============================================================================

pub mod veloct_compiler;

use veloct_compiler::{Lexer, Parser, VeloctCompiler};

fn main() {
    println!("--- Veloct Spatial Engine Ingestion Core Online ---");

    // Simulating a dynamic network schema payload ingestion tick
    let source_code = "module Pipeline.Config; STAGE_LED_WIDTH = 3840;";
    println!("Source Schema Ingested:\n  \"{}\"\n", source_code);

    // 1. Initialize the zero-allocation parser pipeline
    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);

    // 2. Parse the string payload directly into abstract syntax tracking models
    if let Some(ast) = parser.parse_module() {
        println!("=== Abstract Syntax Tree Resolved ===");
        println!("  Module Branch : {}", ast.module_name);
        println!("  Target Key    : {}", ast.key);
        println!("  Assigned Val  : {}\n", ast.value);

        // 3. Compile the structural AST nodes directly into byte array opcodes
        let compiler = VeloctCompiler::new();
        let opcodes = compiler.compile_module(&ast);

        println!("=== Compiled Binary Machine Opcodes ===");
        println!("  Raw Hex Byte Stream: {:X?}\n", opcodes);
        println!("Compilation Sequence Finalized: Complete.");
    } else {
        println!("Fatal Compilation Error: Invalid structural schema formatting.");
    }
}
