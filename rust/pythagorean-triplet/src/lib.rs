pub fn find() -> Option<u32> {
    for a in 0..500 {
        for b in 0..500 {
            for c in 0..500 {
                if testTriplet(a, b, c) {
                    if a + b + c == 1000 {
                        println!("{} {} {}", a,b,c);
                        return Some(a * b * c)
                    }
                }
            }
        }
    }

    return None
}

fn testTriplet(a: u32, b: u32, c: u32) -> bool {
    if a.pow(2) + b.pow(2) == c.pow(2) {
        return true
    }

    false
}
