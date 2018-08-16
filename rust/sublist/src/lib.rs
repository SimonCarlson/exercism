#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}


pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == b.len() {
        let mut equal = true;
        for (i, j) in a.iter().zip(b.iter()) {
            if i != j {
                equal = false;
                break;
            }
        }

        if equal {
            return Comparison::Equal
        };
    };

    if a.len() == 0 && b.len() > 0 {
        return Comparison::Sublist
    };

    if a.len() > 0 && b.len() == 0 {
        return Comparison::Superlist
    };

    Comparison::Unequal
}