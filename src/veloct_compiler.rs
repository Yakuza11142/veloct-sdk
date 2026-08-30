// ============================================================================
// MODULE: Veloct Compiler Core
// DESCRIPTION: Zero-Allocation, Hot-Path Data-Driven Ingestion Loop
// ============================================================================

/// Minimal, tightly packed token representations for the zero-allocation parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Module,
    Identifier(&'a str),
    Assign,
    Number(&'a str),
    Semicolon,
    Eof,
}

/// Zero-allocation Lexer that slices string references without copying memory.
pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        if self.cursor >= self.source.len() {
            return Token::Eof;
        }

        let remaining = &self.source[self.cursor..];

        // Match Keywords or Identifiers
        if let Some(first_char) = remaining.chars().next() {
            if first_char.is_alphabetic() || first_char == '_' {
                let len = remaining
                    .chars()
                    .take_while(|&c| c.is_alphanumeric() || c == '_' || c == '.')
                    .map(|c| c.len_utf8())
                    .sum::<usize>();

                let ident = &remaining[..len];
                self.cursor += len;

                return match ident {
                    "module" => Token::Module,
                    _ => Token::Identifier(ident),
                };
            }

            // Match Numeric Literals
            if first_char.is_numeric() {
                let len = remaining
                    .chars()
                    .take_while(|&c| c.is_numeric() || c == '.')
                    .map(|c| c.len_utf8())
                    .sum::<usize>();

                let num = &remaining[..len];
                self.cursor += len;
                return Token::Number(num);
            }

            // Match Structural Syntax Operators
            match first_char {
                '=' => {
                    self.cursor += 1;
                    return Token::Assign;
                }
                ';' => {
                    self.cursor += 1;
                    return Token::Semicolon;
                }
                _ => {
                    self.cursor += first_char.len_utf8();
                }
            }
        }

        Token::Eof
    }

    fn skip_whitespace(&mut self) {
        let remaining = &self.source[self.cursor..];
        let ws_len = remaining
            .chars()
            .take_while(|c| c.is_whitespace())
            .map(|c| c.len_utf8())
            .sum::<usize>();
        self.cursor += ws_len;
    }
}

/// Tightly packed abstract syntax layout nodes.
#[derive(Debug, Clone)]
pub struct AstNode<'a> {
    pub module_name: &'a str,
    pub key: &'a str,
    pub value: &'a str,
}

/// The Zero-Allocation Structural Schema Parser.
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn consume(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse_module(&mut self) -> Option<AstNode<'a>> {
        if let Token::Module = self.current_token {
            self.consume();

            if let Token::Identifier(mod_name) = self.current_token {
                self.consume();

                if let Token::Semicolon = self.current_token {
                    self.consume();
                }

                let mut key = "";
                let mut value = "";

                if let Token::Identifier(k) = self.current_token {
                    key = k;
                    self.consume();

                    if let Token::Assign = self.current_token {
                        self.consume();

                        if let Token::Number(v) = self.current_token {
                            value = v;
                            self.consume();

                            if let Token::Semicolon = self.current_token {
                                self.consume();
                            }
                        }
                    }
                }

                return Some(AstNode {
                    module_name: mod_name,
                    key,
                    value,
                });
            }
        }
        None
    }
}

/// The master code generation engine.
pub struct VeloctCompiler;

impl VeloctCompiler {
    pub fn new() -> Self {
        Self
    }

    /// Generates flattened binary schema opcodes branchlessly into a continuous byte array.
    pub fn compile_module(&self, ast: &AstNode) -> Vec<u8> {
        let mut opcodes = Vec::with_capacity(32);
        
        // Command Byte: 0xCC represents a fresh Module Registration command
        opcodes.push(0xCC);
        opcodes.extend_from_slice(ast.module_name.as_bytes());
        opcodes.push(0x3A); // Divider token ":"
        opcodes.extend_from_slice(ast.key.as_bytes());
        opcodes.push(0x3D); // Assignment operator token "="
        opcodes.extend_from_slice(ast.value.as_bytes());
        opcodes.push(0xFF); // Termination token marker
        
        opcodes
    }
}
