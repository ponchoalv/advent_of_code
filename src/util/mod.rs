use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

fn read_input(name: &str) -> Lines<BufReader<File>> {
    let file = File::open(format!("inputs/{name}", name = name)).unwrap();
    let reader = BufReader::new(file);
    reader.lines()
}

pub fn read_input_as_i32(name: &str) -> Vec<i32> {
    read_input(name)
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

pub fn read_input_as_string(name: &str) -> Vec<String> {
    read_input(name)
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
}

#[test]
fn testing_read_input() {
    let vec_of_strings = read_input("day_1_2018.txt")
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    assert_eq!(vec_of_strings.len(), 1012)
}

#[test]
fn testing_read_input_as_i32() {
    let vec_of_i32 = read_input_as_i32("test_i32.txt");
    let expected_result = vec![16, -2, -5, 1];

    assert_eq!(vec_of_i32, expected_result)
}

#[test]
fn testing_read_input_as_string() {
    let vec_of_string = read_input_as_string("test_string.txt");
    let expected_result = vec!["pepe".to_owned(), "pepito".to_owned()];

    assert_eq!(vec_of_string, expected_result)
}
