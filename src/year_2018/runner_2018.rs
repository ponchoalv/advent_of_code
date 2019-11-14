use crate::year_2018;

pub fn run_advent_of_code_2018() {
    println!("--- Advent of Code 2018 ---");
    println!(
        "* Day 1 - part 1 => {result}",
        result = year_2018::day_1_2018::day_1_1_2018()
    );
    println!(
        "* Day 1 - part 2 => {result}",
        result = year_2018::day_1_2018::day_1_2_2018()
    );

    println!(
        "* Day 2 - part 1 => {result}",
        result = year_2018::day_2_2018::day_2_1_2018()
    );
    println!(
        "* Day 2 - part 2 => {result}",
        result = year_2018::day_2_2018::day_2_2_2018()
    );

    println!(
        "* Day 3 - part 1 => {result:?}",
        result = year_2018::day_3_2018::day_3_1_2018()
    );
    println!(
        "* Day 3 - part 2 => {result:?}",
        result = year_2018::day_3_2018::day_3_2_2018()
    );

    println!(
        "* Day 4 - part 1 => {result:?}",
        result = year_2018::day_4_2018::day_4_1_2018()
    );
    println!(
        "* Day 4 - part 2 => {result:?}",
        result = year_2018::day_4_2018::day_4_2_2018()
    );
}
