pub struct Brackets {
    stack: Vec<char>,
}

impl<'a> From<&'a str> for Brackets {
    fn from(input: &str) -> Self {
        let mut stack: Vec<char> = Vec::new();
        for c in input.chars() {
            stack.push(c);
        };

        Brackets {
            stack
        }
    }
}

impl<'a> Brackets {
    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();
        for &c in &self.stack {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => {
                    let popped = stack.pop();
                    match popped {
                        Some('(') => {
                            if c != ')' {
                                return false
                            };
                        },
                        Some('[') => {
                            if c != ']' {
                                return false
                            };
                        },
                        Some('{') => {
                            if c != '}' {
                                return false
                            };
                        },
                        _ => return false,
                    }
                },
                _ => continue,
            }
        }

        // If there are items left in the stack, they were unbalanced
        if stack.len() > 0 {
            return false
        };

        true
    }
}
