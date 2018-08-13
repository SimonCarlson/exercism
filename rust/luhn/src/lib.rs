/// Check a Luhn checksum.
pub fn is_valid(input_code: &str) -> bool {
    let code = String::from(input_code);
    let mut stripped_code = String::new();

    if code.trim().len() < 2 {
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
    // If stripped_code.len is even, start from zeroth digit. If odd, start from first
    // Iterate over stripped_code with step 2
    let skip = stripped_code.len() % 2;
    let mut double_sum = 0;
    for c in stripped_code.chars().skip(skip).step_by(2) {
        let mut digit;
        match c.to_digit(10) {
            Some(i) => digit = i * 2,
            _ => return false,
        };

        if digit > 9 {
            digit -= 9
        };

        double_sum += digit;
        digit = 0;

    };

    // Sum up the numbers that werent doubled
    for c in stripped_code.chars().skip(1 - skip).step_by(2) {
        match c.to_digit(10) {
            Some(i) => double_sum += i,
            _ => return false,
        }
    };

    if double_sum % 10 == 0 {
        return true
    };

    false
}
