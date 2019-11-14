use std::collections::HashSet;

use crate::util::read_input_as_i32;
use std::iter::Cycle;
use std::slice::Iter;

pub fn day_1_1_2018() -> i32 {
    let day_1_input: Vec<i32> = read_input_as_i32("day_1_2018.txt");
    day_1_input.iter().sum()
}

pub fn day_1_2_2018() -> i32 {
    let day_1_input: Vec<i32> = read_input_as_i32("day_1_2018.txt");
    let mut cycled_input: Cycle<Iter<i32>> = day_1_input.iter().cycle();
    let mut set_of_frequencies: HashSet<i32> = HashSet::with_capacity(day_1_input.len() * 143);
    let mut acc = 0;
    return loop {
        let item: &i32 = cycled_input.next().unwrap();
        let sum: i32 = acc + *item;
        if set_of_frequencies.contains(&sum) {
            break sum;
        } else {
            set_of_frequencies.insert(sum);
        }
        acc = sum;
    };
}

#[test]
fn probando_day_1_1_2018() {
    assert_eq!(585, day_1_1_2018())
}

#[test]
fn probando_day_1_2_2018() {
    assert_eq!(day_1_2_2018(), 83173)
}
