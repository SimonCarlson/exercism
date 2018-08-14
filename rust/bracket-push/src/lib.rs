pub struct Brackets<'a> {
    stack: Vec<&'a str>,
}

impl<'a> From<&'a str> for Brackets<'a> {
    fn from(input: &str) -> Self {
        let mut stack: Vec<&str> = Vec::new();
        for c in input.chars() {
            let slice: &str = &c.to_string()[..];
            stack.push(slice);
        };

        Brackets {
            stack
        }
    }
}

impl<'a> Brackets<'a> {
    pub fn are_balanced(&self) -> bool {
        let mut stack = Vec::new();
        for c in &self.stack {
            // Converts the character to a String to a &str... Feels dumb
            match &c.to_string()[..] {
                "(" | "[" | "{" => stack.push(c),
                ")" | "]" | "}" => {
                    let popped = stack.pop();
                    match popped {
                        Some(&")") => {
                            if c != &"(" {
                                return false
                            };
                        },
                        Some(&"]") => {
                            if c != &"[" {
                                return false
                            };
                        },
                        Some(&"}") => {
                            if c != &"{" {
                                return false
                            };
                        },
                        _ => return false,
                    }
                },
                _ => continue,
            }
        }

        true
    }
}
