use std::collections::HashSet;

use crate::util::read_input_as_string;

fn prepare_input() -> String {
    let day_5_input: Vec<String> = read_input_as_string("day_5_2018.txt");
    return day_5_input.get(0).unwrap().clone();
}

pub fn day_5_1_2018() -> Option<usize> {
    let the_input = prepare_input();
    Some(make_all_reactions(the_input))
}

pub fn day_5_2_2018() -> Option<u32> {
    let input = prepare_input();
    let letters_set = input.chars().map(|x| x.to_ascii_uppercase()).collect::<HashSet<char>>();

    letters_set.iter()
        .map(|&x| input.chars().filter(move |&f| f.to_ascii_uppercase() != x))
        .map(|val| {
            make_all_reactions(val.collect()) as u32
        })
        .min()
}

fn react(input: &str) -> String {
    let mut result = String::from(input);

    for cai in input.chars().zip(input.chars().skip(1)) {
        if (cai.0 as i32 - cai.1 as i32).abs() == 32 {
            let pat = format!("{}{}", cai.0, cai.1);
            result = input.replace(pat.as_str(), "");
            break;
        }
    }
    return result;
}

fn make_all_reactions(the_input: String) -> usize {
    let mut result: usize = the_input.len();
    let mut reacted = react(the_input.as_str());

    loop {
        if reacted.len() == result {
            break;
        } else {
            reacted = react(&reacted);
            result = reacted.len();
            reacted = react(&reacted);
        }
    }

    result
}

#[test]
fn probando_day_5_1_2018() {
    assert_eq!(Some(11108), day_5_1_2018())
}
#[test]
fn probando_day_5_2_2018() {
    assert_eq!(Some(5094), day_5_2_2018())
}
