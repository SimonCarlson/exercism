/// Check a Luhn checksum.
pub fn is_valid(input_code: &str) -> bool {
    let code = String::from(input_code);
    let mut stripped_code = String::new();

    if code.len() < 2 {
        return false
    }

    for c in code.chars() {
        if c.is_digit(10) {
            stripped_code.push(c)
        } else {
            if c.is_whitespace() {
                continue
            } else {
                return false
            }
        }
    };

    // At this point, stripped_code contains a string of only digits

    // Double every second number from the right
    // If the doubling exceeds 9, subtract 9

    // Put the doubled numbers back in order with the rest of the numbers

    // Sum the digits

    // If divisible by 10, pass

    false
}
