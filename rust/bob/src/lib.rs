pub fn reply(message: &str) -> &str {
    if is_question(message) {
        if is_yelling(message) {
            return "Calm down, I know what I'm doing!"
        }
        return "Sure."
    }

    if is_yelling(message) {
        return "Whoa, chill out!"
    }

    if not_saying(message) {
        return "Fine. Be that way!"
    }

    "Whatever."
}

fn is_question(message: &str) -> bool {
    if message.trim().ends_with("?") {
        return true
    }

    false
}

fn is_yelling(message: &str) -> bool {
    let mut yelling = false;
    let mut capsCount = 0;
    let mut foundLower = false;
    // split at space to get words
    // check if all alphabetical words are uppercase or not
    for word in message.trim().split_whitespace() {
        for character in word.chars() {
            if character.is_alphabetic() {
                if character.to_ascii_uppercase() == character {
                    capsCount += 1;
                } else {
                    foundLower = true;
                }
            }
        }
    }

    if capsCount >= 2 && !foundLower {
        yelling = true;
    }

    yelling
}

fn not_saying(message: &str) -> bool {
    if message.trim() == "" {
        return true
    }

    false
}
