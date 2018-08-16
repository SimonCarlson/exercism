#[derive(PartialEq, Debug)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal
}


pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == b.len() {
        for (i, j) in a.iter().zip(b.iter()) {
            if i != j {
                break
            }
        }

        return Comparison::Equal
    };

    Comparison::Unequal
}