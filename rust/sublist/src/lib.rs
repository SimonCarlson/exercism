#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}


pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == 0 && b.len() > 0 {
        return Comparison::Sublist
    };

    if a.len() > 0 && b.len() == 0 {
        return Comparison::Superlist
    };

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