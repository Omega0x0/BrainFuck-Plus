use std::collections::HashMap;

use crate::token::Token;

pub struct Interpreter {
    pub array_bytes: [u8; 300],
    pub pointer: usize,
    pub function_stack: HashMap<[u8; 4], Vec<Token>>
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            array_bytes: [0; 300],
            pointer: 150,
            function_stack: HashMap::new()
        }
    }

    pub fn run(&mut self, tokens: &Vec<Token>) {
        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                Token::Increment => self.array_bytes[self.pointer] += 1,
                Token::Decrement => self.array_bytes[self.pointer] -= 1,
                Token::MoveLeft => self.pointer -= 1,
                Token::MoveRight => self.pointer += 1,
                Token::Print => print!("{}", self.array_bytes[self.pointer] as char),
                Token::Input => {},
                Token::BlockCode(tokens) => self.run(tokens),

                Token::CreateLoop => {
                    while self.array_bytes[self.pointer] != 0 {
                        self.run(
                            if let Token::BlockCode(tokens) = &tokens[i + 1] {
                                tokens
                            } else { panic!("The code block was not passed to '@'") }
                        );
                    }
                    i += 1;
                },

                Token::CreateFunction => {
                    self.function_stack.insert(
                        [
                            self.array_bytes[self.pointer], self.array_bytes[self.pointer + 1],
                            self.array_bytes[self.pointer + 2], self.array_bytes[self.pointer + 3]
                        ],
                        if let Token::BlockCode(tokens) = &tokens[i + 1] {
                            tokens.clone()
                        } else { panic!("The code block was not passed to '$'") }
                    );
                    i += 1;
                },

                Token::CallFunction => {
                    let fun = self.function_stack.get(&[
                        self.array_bytes[self.pointer], 
                        self.array_bytes[self.pointer + 1],
                        self.array_bytes[self.pointer + 2], 
                        self.array_bytes[self.pointer + 3]
                    ]);

                    if let Some(f) = fun {
                        self.run(&f.clone());
                    } else {
                        self.debug(self.pointer);
                        self.debug(self.pointer + 1);
                        self.debug(self.pointer + 2);
                        self.debug(self.pointer + 3);
                        panic!("The function does not exist");
                    }
                }

                Token::Debug => self.debug(self.pointer),
            }

            i += 1;
        }
    }

    fn debug(&self, pointer: usize) {
        print!("Value(pointer: {}): {};\n", self.pointer, self.array_bytes[pointer])
    }
}