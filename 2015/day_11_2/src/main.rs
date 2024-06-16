//! Advent of code 2015 day 11 part 2

use day_11_1::password_generator::{next_password_str, next_valid_password_str};

fn main() {
    let next_password =
        next_valid_password_str(&next_password_str(&next_valid_password_str("hxbxwxba")));
    println!("Next valid password: {next_password}");
}
