struct beer {}

impl beer {
    fn verse(n: u32) -> String {
        verse(n)
    }

    fn sing(start: u32, end: u32) -> String {
        sing(start, end)
    }
}

fn main() {}

pub fn verse(n: u32) -> String {
    match n {
        value if value == 1 => {
            "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
        }
        value if value > 0 => {
            let s = if n > 1 {
                format!("{} bottles", n)
            } else {
                format!("{} bottle", n)
            };
            let q = if n != 2 {
                format!("{} bottles", n - 1)
            } else {
                format!("{} bottle", n - 1)
            };
            format!("{} of beer on the wall, {} of beer.\nTake one down and pass it around, {} of beer on the wall.\n", s, s, q)
        }
        _ => {
            "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = String::new();
    for i in (end..start + 1).rev() {
        println!("{}", i);
        let res = verse(i);
        song.push_str(format!("{}\n", &res).as_str());
    }
    song.truncate(song.len() - 1);
    song
}

#[test]
fn test_verse_0() {
    assert_eq!(beer::verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}
#[test]

fn test_verse_1() {
    assert_eq!(beer::verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
}
#[test]

fn test_verse_2() {
    assert_eq!(beer::verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
}
#[test]

fn test_verse_8() {
    assert_eq!(beer::verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
}
#[test]

fn test_song_8_6() {
    assert_eq!(beer::sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}
#[test]

fn test_song_3_0() {
    assert_eq!(beer::sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}
