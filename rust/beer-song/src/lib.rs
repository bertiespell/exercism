pub fn verse(n: i32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1),
    }
}

pub fn singx(start: i32, end: i32) -> String {
    let mut song = String::new();
    let mut counter = start;
    while counter >= end {
        song.push_str(&verse(counter));
        song.push_str(&String::from("\n"));
        counter = counter - 1;
    }
    song.pop(); // remove additional \n character
    song
}

// refactored

pub fn sing(start: i32, end: i32) -> String {
    (end..=start)
        .rev() // we want them backwards
        .map(verse) // yaaas
        .collect::<Vec<_>>() // need to collect these up now into a Vec
        .join("\n") // join them together with a new line
}