pub struct LexicAnalyzer {
    pub input: String,
    pub current: char,
    pub current_pos: usize,
    pub current_line: usize,
    pub current_col: usize,
}

impl LexicAnalyzer {
    pub fn new(input: String) -> Self {
        let mut cloned_input = input.clone();
        let current = cloned_input.remove(0);
        LexicAnalyzer {
            input: cloned_input,
            current,
            current_pos: 1,
            current_line: 1,
            current_col: 1,
        }
    }

    pub fn next_char(&mut self) -> char {
        let next = if let Some(character) = self.input.pop() {
            character
        } else {
            '\0'
        };
        if next != '\0' {
            self.current_pos += 1;
            self.current_col += 1;
            if next == '\n' {
                self.current_line += 1;
                self.current_col = 0;
            }
        }
        self.current = next;
        next
    }

    pub fn skip_empty(&mut self) {
        while self.current == ' '
            || self.current == '\n'
            || self.current == '\r'
            || self.current == '\t'
        {
            self.next_char();
        }
    }

    pub fn next_token(&mut self) {
        self.skip_empty();
    }
}
