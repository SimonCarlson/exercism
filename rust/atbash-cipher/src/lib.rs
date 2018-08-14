pub fn crypt(input: &str) -> String {
    let encoding: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().rev().collect();
    let mut crypto = String::new();
    for c in input.replace(" ", "").replace(",", "").replace(".", "").to_ascii_lowercase().chars() {
        if !c.is_ascii() {
            continue
        };

        if c.is_digit(10) {
            crypto.push(c)
        } else {
            // offset the ascii value of lower case letters to index the reversed alphabet
            crypto.push(encoding[c as usize - 97])
        }
    };

    crypto
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let crypto = crypt(plain);
    let mut space_crypto = String::new();
    for (i, c) in crypto.chars().enumerate() {
        if i % 5 == 0 && i != 0 {
            space_crypto.push_str(" ")
        };
        space_crypto.push(c);
    };

    space_crypto
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    crypt(cipher)
}
