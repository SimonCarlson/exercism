pub fn verse(n: i32) -> String {
    let mut verse = String::new();
    
    if n > 2 {
        verse = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1);
    }

    if n == 2 {
        verse = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n - 1);
    }

    if n == 1 {
        verse.push_str("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
    }

    if n == 0 {
        verse.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
    }

    verse
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = String::new();
    let mut i = start;
    while i >= end {
        song.push_str(&verse(i));
        if i - end > 0 {
            song.push_str("\n");
        }
        i -= 1;
    }

    song
}
