fn get_level_for_value(number: i32) -> i32 {
    (((number as f32).sqrt() + 1.0) / 2.0).ceil() as i32
}

fn get_number_position_on_level(level: i32, number: i32) -> i32 {
    (number - (2 * level - 1).pow(2) - 1) %  level
}

fn get_position_for_value(value: i32) {
    for i in 1..value {
        let level = get_level_for_value(i);
        let position_on_level = get_number_position_on_level(level, i);
        println!("Level for number {}: {}", i, level);
        //println!("Position for number {}: {}", i, position_on_level);
    }
}

pub fn day_3_1_2017() {
    get_position_for_value(81);
    println!("valores 12 {:?}",(1..81).collect::<Vec<i32>>().chunks((get_level_for_value(6) as usize - 1) * 2 ).skip(((get_level_for_value(6) as usize - 2 ) * 2)).collect::<Vec<&[i32]>>());
}

