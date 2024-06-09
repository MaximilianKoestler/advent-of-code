//! Advent of code 2015 day 10 part 2

use day_10_1::repeat_look_and_say;

fn main() {
    let input = "1113222113";
    let size = repeat_look_and_say(input, 50).len();
    print!("Final length: {size}");
}
