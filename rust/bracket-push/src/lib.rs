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
        for c in &self.stack {
            // Converts the character to a String to a &str... Feels dumb
            let slice = &c.to_string()[..];
            match slice {
                "(" | "[" | "{" => stack.push(slice),
                ")" | "]" | "}" => {
                    let popped = stack.pop();
                    match popped {
                        Some(")") => {
                            if slice != "(" {
                                return false
                            };
                        },
                        Some("]") => {
                            if slice != "[" {
                                return false
                            };
                        },
                        Some("}") => {
                            if slice != "{" {
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
