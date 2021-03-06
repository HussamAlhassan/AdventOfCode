/*
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from
here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with
our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been
allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according
to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest
and highest number of times a given letter must appear for the password to be valid. For example,
1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no
instances of b, but needs at least 1. The first and third passwords are valid: they contain one a
or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
*/

use std::io::Read;

fn main() {
   let mut valid = 0;

   let mut file = std::fs::File::open(r#"D:\05- Projects\AdventOfCode\Puzzle_2-1\target\debug\input.txt"#).unwrap();
   let mut contents = String::new();
   file.read_to_string(&mut contents).unwrap();

   let lines: Vec<&str> = contents.lines().collect();

   for l in lines {
      valid += is_valid(l);
   }

   println!("Number of valid entries is: {}", valid)
}

fn is_valid(value: &str) -> i32 {
   let split = value.split_whitespace().collect::<Vec<&str>>();
   let range = split[0];
   let letter = split[1].chars().nth(0).unwrap();
   let password = split[2];

   // println!("{} :: \"{}\" and \"{}\" and \"{}\"", value, range, letter, password);

   let range_values = range.split('-').collect::<Vec<&str>>();
   let lower_range = range_values[0].parse::<usize>().unwrap();
   let upper_range = range_values[1].parse::<usize>().unwrap();

   let occurrences = password.matches(letter).count();
   // println!("{}", occurrences);

   match occurrences >= lower_range && occurrences <= upper_range {
      true => 1,
      false => 0,
   }
}
