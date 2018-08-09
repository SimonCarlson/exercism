pub fn raindrops(n: usize) -> String {
    let mut dropSound = String::new();
    if n % 3 == 0 {
        dropSound.push_str("Pling")
    }
    if n % 5 == 0 {
        dropSound.push_str("Plang")
    }
    if n % 7 == 0 {
        dropSound.push_str("Plong")
    }

    if dropSound == "" {
        return n.to_string()
    }

    dropSound
}
