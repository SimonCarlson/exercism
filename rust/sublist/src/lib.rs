#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}


pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() > b.len() {
        let result = sublist(b, a);
        match result {
            Comparison::Sublist => return Comparison::Superlist,
            _ => return result
        };
    }

    for i in 0..(b.len() - a.len() + 1) {
        if b[i..(i + a.len())] == a[..] {
            if b.len() == a.len() {
                return Comparison::Equal
            } else {
                return Comparison::Sublist
            }
        }
    }

    Comparison::Unequal
}