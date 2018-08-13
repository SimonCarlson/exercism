/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let cipher: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().rev().collect();
    let mut crypto = String::new();
    for c in plain.replace(" ", "").replace(",", "").replace(".", "").to_ascii_lowercase().chars() {
        if c.is_digit(10) {
            crypto.push(c)
        } else {
            crypto.push(cipher[c as usize - 97])
        }
    };

    crypto
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    unimplemented!("Decoding of {:?} in Atbash cipher.", cipher);
}
