use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::util::read_input_as_string;

pub fn day_2_1_2018() -> u32 {
    let day_2_input: Vec<String> = read_input_as_string("day_2_2018.txt");
    let result: (u32, u32) = day_2_input
        .iter()
        .map(get_tuple_of_twice_and_thrice)
        .fold((0, 0), |(x1, y1), (x2, y2)| {
            (x1 + x2 as u32, y1 + y2 as u32)
        });
    result.0 * result.1
}

fn get_tuple_of_twice_and_thrice(line: &String) -> (u8, u8) {
    let mut letter_counter: HashMap<char, u8> = HashMap::with_capacity(line.len());
    let mut result = (0, 0);

    for letter in line.chars() {
        match letter_counter.clone().get(&letter) {
            Some(value) => letter_counter.insert(letter, value + 1),
            None => letter_counter.insert(letter, 1),
        };
    }

    for (_, count) in letter_counter {
        match (count, result) {
            (2, (0, thrice)) => result = (1, thrice),
            (3, (twice, 0)) => result = (twice, 1),
            _ => (),
        };
    }

    result
}

fn distance(first_line: &String, second_line: &String) -> u8 {
    first_line
        .chars()
        .zip(second_line.chars())
        .fold(0, |acc, (first, second)| match first == second {
            true => acc,
            false => acc + 1,
        })
}

pub fn day_2_2_2018() -> String {
    let day_2_input: Vec<String> = read_input_as_string("day_2_2018.txt");
    let mut differ_by_one: Option<(String, String)> = None;

    for i in 1..(day_2_input.len() / 2) {
        match day_2_input
            .clone()
            .into_iter()
            .zip(day_2_input.clone().into_iter().cycle().skip(i))
            .filter(|(first_line, second_line)| distance(first_line, second_line) == 1)
            .take(1)
            .next()
        {
            Some(value) => differ_by_one = Some(value),
            None => (),
        };
    }

    let (line1, line2) = match differ_by_one {
        Some(value) => value,
        None => (String::from(""), String::from("")),
    };

    line1
        .clone()
        .chars()
        .filter(|letter| {
            HashSet::<char>::from_iter(line1.chars())
                .intersection(&HashSet::<char>::from_iter(line2.chars()))
                .collect::<HashSet<&char>>()
                .contains(letter)
        })
        .collect()
}

#[test]
fn probando_day_2_1_2018() {
    assert_eq!(7936, day_2_1_2018())
}

#[test]
fn probando_day_2_2_2018() {
    assert_eq!(String::from("lnfqdscwjyteorambzuchrgpx"), day_2_2_2018())
}
