// On the 12th day of Christmas my true love sent to me
// 12 drummers drumming
// 11 pipers piping
// 10 lords a-leaping
// Nine ladies dancing
// Eight maids a-milking
// Seven swans a-swimming
// Six geese a-laying
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

fn main() {
    let number_to_cardinal = [
        "a",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve"
    ];

    let number_to_ordinal = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let gifts = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    println!("Hello, world!");

    let days_of_xmas = 12;

    let mut day = 0;
    while day < days_of_xmas {
        let ordinal_day = number_to_ordinal[day];

        println!("On the {} day of Christmas my true love sent to me", ordinal_day);
        for gift_idx in (0..=day).rev() {
            let cardinal_count = number_to_cardinal[gift_idx];
            println!("{} {}", cardinal_count, gifts[gift_idx]);
        }

        day += 1;
    }
}
