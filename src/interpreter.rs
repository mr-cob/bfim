use std::fs::read_to_string;

pub struct Interpreter {
    memory: Vec<u8>,
    current_pointer: usize,
    tokens: Vec<u8>,
    current_token: usize,
    line: usize,
    debug: bool,
}

impl Interpreter {
    pub fn new(memory_size: usize, debug: bool) -> Self {
        let mut memory = Vec::new();
        let tokens = Vec::new();
        for _ in 0..memory_size {
            memory.push(0);
        }
        Self {
            memory,
            current_pointer: 0,
            tokens,
            current_token: 0,
            line: 1,
            debug,
        }
    }

    pub fn interpret(&mut self, filepath: &str) {
        self.tokens = self.scan_source(filepath);
        loop {
            let token = self.next_token();
            if token == '\0' {
                break;
            }
            self.interpret_token(token);
        }
    }

    fn interpret_token(&mut self, current_charecter: char) {
        if self.debug {
            println!(
                "line: {}, token_index: {}, pointer address: {}, memory: {:?}",
                self.line,
                self.current_token + 1,
                self.current_pointer,
                self.memory
            );
        }

        match current_charecter {
            '>' => {
                if self.current_pointer < self.memory.len() {
                    self.current_pointer += 1;
                } else {
                    self.current_pointer = 0;
                }
            }
            '<' => {
                if self.current_pointer > 0 {
                    self.current_pointer -= 1;
                } else {
                    self.current_pointer = self.memory.len() - 1;
                }
            }
            '+' => {
                self.memory[self.current_pointer] += 1;
            }
            '-' => {
                self.memory[self.current_pointer] -= 1;
            }
            '[' => {
                if self.memory[self.current_pointer] == 0 {
                    let end_block = self.match_ending_block(']');
                    if end_block == -1 {
                        self.error(&format!("Unterminated Block `[` in line {}", self.line));
                    } else {
                        self.current_token = end_block as usize;
                    }
                } else {
                    return;
                }
            }
            ']' => {
                if self.memory[self.current_pointer] == 0 {
                    return;
                } else {
                    let opening_block = self.match_opening_block('[');
                    if opening_block == -1 {
                        self.error(&format!("Unterminated Block `]` in line {}", self.line));
                    } else {
                        self.current_token = opening_block as usize;
                    }
                }
            }
            '.' => {
                let data = self.memory[self.current_pointer] as char;
                println!("{data}");
            }
            ' ' | '\t' | '\r' => {
                return;
            }
            '\n' => {
                self.line += 1;
                return;
            }
            token => {
                self.error(&format!("Invalid Token `{}` in line {}", token, self.line));
            }
        }
    }

    fn scan_source(&self, filepath: &str) -> Vec<u8> {
        let tokens = read_to_string(filepath)
            .unwrap()
            .as_bytes()
            .to_ascii_lowercase();
        tokens
    }

    fn match_ending_block(&mut self, expected_block: char) -> isize {
        for i in self.current_token..self.tokens.len() {
            let block = self.tokens[i] as char;
            if block == expected_block {
                return i as isize;
            }
        }
        -1
    }

    fn match_opening_block(&mut self, expected_block: char) -> isize {
        for i in 0..self.current_token {
            let block = self.tokens[i] as char;
            if block == expected_block {
                return i as isize;
            }
        }
        -1
    }

    fn next_token(&mut self) -> char {
        if self.current_token < self.tokens.len() {
            let token = self.tokens.get(self.current_token).unwrap().to_owned();
            self.current_token += 1;
            return token as char;
        }
        '\0'
    }

    fn error(&self, message: &str) {
        println!("ERROR: {}", message);
        std::process::exit(1);
    }
}
