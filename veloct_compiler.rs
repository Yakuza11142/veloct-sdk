use std::collections::HashMap;

// ============================================================================
// 1. LEXER / TOKENIZER
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Module, Struct, System, Public, Kernel, Fn, Mut, Let, Return, If, Else, While,
    GPUCompute, Ident(String), IntLit(i64), FloatLit(f64),
    Colon, Semicolon, Comma, Dot, Assign,
    Plus, Minus, Star, Slash, Modulo,
    LParen, RParen, LBrace, RBrace, LBracket, RBracket,
    Eof,
}

pub struct Lexer<'a> {
    input: &'a [char],
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [char]) -> Self {
        Lexer { input: source, pos: 0 }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch.is_whitespace() {
                self.pos += 1;
            } else if ch == '/' && self.pos + 1 < self.input.len() && self.input[self.pos + 1] == '/' {
                while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                    self.pos += 1;
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input[self.pos];

        // Identifiers and Keywords
        if ch.is_alphabetic() || ch == '_' {
            let start = self.pos;
            while self.pos < self.input.len() && (self.input[self.pos].is_alphanumeric() || self.input[self.pos] == '_') {
                self.pos += 1;
            }
            let word: String = self.input[start..self.pos].iter().collect();
            return match word.as_str() {
                "module" => Token::Module,
                "struct" => Token::Struct,
                "system" => Token::System,
                "public" => Token::Public,
                "kernel" => Token::Kernel,
                "fn" => Token::Fn,
                "mut" => Token::Mut,
                "let" => Token::Let,
                "return" => Token::Return,
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,
                "GPUCompute" => Token::GPUCompute,
                _ => Token::Ident(word),
            };
        }

        // Numeric Literals
        if ch.is_numeric() {
            let start = self.pos;
            let mut is_float = false;
            while self.pos < self.input.len() && (self.input[self.pos].is_numeric() || self.input[self.pos] == '.') {
                if self.input[self.pos] == '.' { is_float = true; }
                self.pos += 1;
            }
            let num_str: String = self.input[start..self.pos].iter().collect();
            return if is_float {
                Token::FloatLit(num_str.parse().unwrap_or(0.0))
            } else {
                Token::IntLit(num_str.parse().unwrap_or(0))
            };
        }

        // Single Character Symbols
        self.pos += 1;
        match ch {
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '.' => Token::Dot,
            '=' => Token::Assign,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Modulo,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            _ => Token::Eof,
        }
    }
}

// ============================================================================
// 2. ABSTRACT SYNTAX TREE (AST)
// ============================================================================

#[derive(Debug, Clone)]
pub enum Type {
    I16, I32, I64, U8, U16, U32, U64, F32, F64, Bool, Vec2, Vec3, Vec4,
    Array(Box<Type>, usize),
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum ASTExpr {
    LiteralInt(i64),
    LiteralFloat(f64),
    Variable(String),
    BinaryOp { left: Box<ASTExpr>, op: char, right: Box<ASTExpr> },
    FnCall { name: String, args: Vec<ASTExpr> },
}

#[derive(Debug, Clone)]
pub enum ASTStmt {
    LetDecl { name: String, is_mut: bool, init: ASTExpr },
    Assign { name: String, value: ASTExpr },
    Return(Option<ASTExpr>),
    Expr(ASTExpr),
}

#[derive(Debug, Clone)]
pub struct ASTFunction {
    pub name: String,
    pub is_gpu_kernel: bool,
    pub is_public: bool,
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: Vec<ASTStmt>,
}

#[derive(Debug, Clone)]
pub struct ASTModule {
    pub name: String,
    pub functions: Vec<ASTFunction>,
}

// ============================================================================
// 3. PARSER
// ============================================================================

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let first_tok = lexer.next_token();
        Parser { lexer, cur_tok: first_tok }
    }

    fn eat(&mut self, expected: Token) {
        if std::mem::discriminant(&self.cur_tok) == std::mem::discriminant(&expected) {
            self.cur_tok = self.lexer.next_token();
        } else {
            panic!("Parser Error: Expected {:?}, got {:?}", expected, self.cur_tok);
        }
    }

    pub fn parse_module(&mut self) -> ASTModule {
        self.eat(Token::Module);
        let mod_name = match &self.cur_tok {
            Token::Ident(name) => name.clone(),
            _ => panic!("Expected module identifier"),
        };
        self.eat(self.cur_tok.clone());
        self.eat(Token::Semicolon);

        let mut functions = Vec::new();

        while self.cur_tok != Token::Eof {
            let mut is_public = false;
            let mut is_gpu_kernel = false;

            if self.cur_tok == Token::LBracket {
                self.eat(Token::LBracket);
                if self.cur_tok == Token::GPUCompute {
                    is_gpu_kernel = true;
                    self.eat(Token::GPUCompute);
                }
                self.eat(Token::RBracket);
            }

            if self.cur_tok == Token::Public {
                is_public = true;
                self.eat(Token::Public);
            }

            if self.cur_tok == Token::System {
                self.eat(Token::System);
                match &self.cur_tok { Token::Ident(_) => self.eat(self.cur_tok.clone()), _ => {} }
                self.eat(Token::LBrace);
            }

            if self.cur_tok == Token::Kernel || self.cur_tok == Token::Fn {
                if self.cur_tok == Token::Kernel { is_gpu_kernel = true; }
                self.eat(self.cur_tok.clone());

                let fn_name = match &self.cur_tok {
                    Token::Ident(n) => n.clone(),
                    _ => panic!("Expected function name"),
                };
                self.eat(self.cur_tok.clone());
                self.eat(Token::LParen);
                self.eat(Token::RParen);

                let mut return_type = None;
                if self.cur_tok == Token::Minus {
                    self.eat(Token::Minus);
                    // Single-pass return parsing demo
                    return_type = Some(Type::F32);
                }

                self.eat(Token::LBrace);
                let mut body = Vec::new();

                while self.cur_tok != Token::RBrace && self.cur_tok != Token::Eof {
                    if self.cur_tok == Token::Let {
                        self.eat(Token::Let);
                        let mut is_mut = false;
                        if self.cur_tok == Token::Mut {
                            is_mut = true;
                            self.eat(Token::Mut);
                        }
                        let var_name = match &self.cur_tok {
                            Token::Ident(n) => n.clone(),
                            _ => panic!("Expected variable name"),
                        };
                        self.eat(self.cur_tok.clone());
                        self.eat(Token::Assign);
                        
                        let init_expr = match &self.cur_tok {
                            Token::FloatLit(v) => ASTExpr::LiteralFloat(*v),
                            Token::IntLit(v) => ASTExpr::LiteralInt(*v),
                            _ => ASTExpr::LiteralFloat(0.0),
                        };
                        self.eat(self.cur_tok.clone());
                        self.eat(Token::Semicolon);

                        body.push(ASTStmt::LetDecl { name: var_name, is_mut, init: init_expr });
                    } else if self.cur_tok == Token::Return {
                        self.eat(Token::Return);
                        body.push(ASTStmt::Return(None));
                        self.eat(Token::Semicolon);
                    } else {
                        self.eat(self.cur_tok.clone());
                    }
                }

                self.eat(Token::RBrace);
                functions.push(ASTFunction {
                    name: fn_name,
                    is_gpu_kernel,
                    is_public,
                    params: vec![],
                    return_type,
                    body,
                });
            } else if self.cur_tok == Token::RBrace {
                self.eat(Token::RBrace);
            } else {
                self.cur_tok = self.lexer.next_token();
            }
        }

        ASTModule { name: mod_name, functions }
    }
}

// ============================================================================
// 4. BYTECODE EMITTER
// ============================================================================

#[derive(Debug, Clone)]
pub enum Opcode {
    Nop,
    PushF32(f32),
    StoreLocal(u32),
    LoadLocal(u32),
    AddF32,
    SubF32,
    MulF32,
    DispatchGPUKernel { threads_x: u32, threads_y: u32, threads_z: u32 },
    Return,
}

pub struct VeloctCompiler {
    pub symbol_table: HashMap<String, u32>,
    pub bytecode: Vec<Opcode>,
}

impl VeloctCompiler {
    pub fn new() -> Self {
        VeloctCompiler {
            symbol_table: HashMap::new(),
            bytecode: Vec::new(),
        }
    }

    pub fn compile_module(&mut self, module: &ASTModule) -> Vec<Opcode> {
        for func in &module.functions {
            if func.is_gpu_kernel {
                self.bytecode.push(Opcode::DispatchGPUKernel { threads_x: 64, threads_y: 1, threads_z: 1 });
            }

            let mut local_idx = 0;
            for stmt in &func.body {
                match stmt {
                    ASTStmt::LetDecl { name, init, .. } => {
                        match init {
                            ASTExpr::LiteralFloat(val) => self.bytecode.push(Opcode::PushF32(*val as f32)),
                            ASTExpr::LiteralInt(val) => self.bytecode.push(Opcode::PushF32(*val as f32)),
                            _ => {}
                        }
                        self.symbol_table.insert(name.clone(), local_idx);
                        self.bytecode.push(Opcode::StoreLocal(local_idx));
                        local_idx += 1;
                    }
                    ASTStmt::Return(_) => {
                        self.bytecode.push(Opcode::Return);
                    }
                    _ => {}
                }
            }
        }
        self.bytecode.clone()
    }
}

// ============================================================================
// MAIN PIPELINE EXECUTION
// ============================================================================

fn main() {
    let raw_vct_code = "
        module Core.SpatialEngine;

        public system SpatialNodeProcessor {
            [GPUCompute]
            public kernel fn update_kinematics() {
                let mut node_mass = 12.5;
                let delta_time = 0.016;
                return;
            }
        }
    ".chars().collect::<Vec<char>>();

    println!("--- [1/3] Tokenizing .vct source ---");
    let lexer = Lexer::new(&raw_vct_code);

    println!("--- [2/3] Parsing into Veloct AST ---");
    let mut parser = Parser::new(lexer);
    let ast_module = parser.parse_module();
    println!("Parsed Module: {}", ast_module.name);
    println!("Functions Count: {}", ast_module.functions.len());

    println!("--- [3/3] Generating Veloct Bytecode ---");
    let mut compiler = VeloctCompiler::new();
    let compiled_opcodes = compiler.compile_module(&ast_module);

    for (index, op) in compiled_opcodes.iter().enumerate() {
        println!("{:04}: {:?}", index, op);
    }
}
