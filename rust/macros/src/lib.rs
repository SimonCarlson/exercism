#[macro_export]
macro_rules! hashmap {
    // No trailing comma
    ( $($x:expr => $z:expr), * ) => {
        {
            let mut hm = HashMap::new();
            $(
                hm.insert($x, $z);
            )*
            hm
        }
    };

    // Trailing comma
    ( $($x:expr => $z:expr,) * ) => {
        {
            let mut hm = HashMap::new();
            $(
                hm.insert($x, $z);
            )*
            hm
        }
    };
}
