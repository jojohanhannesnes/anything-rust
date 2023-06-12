fn main() {
    let mut lyrics:[&str; 12] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];
    lyrics.reverse();
    for i in 0..lyrics.len() {
        println!("On the {} day of Christmas, my true love sent to me", i + 1);
        for index in (0..i+1).rev() {
            println!("{}", lyrics[index]);
        }
    }
}
