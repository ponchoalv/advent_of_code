fn get_level_for_value(number: i32) -> i32 {
    (((number as f32).sqrt() + 1.0) / 2.0).ceil() as i32 - 1
}

fn get_number_position_on_level(number: i32, level: i32) -> i32 {
    (number - (2 * level - 1).pow(2) - 1) %  level
}

fn get_distance_for_number(num: i32, level:i32) -> i32 {
    let posicion = get_number_position_on_level(level, num);
    (posicion - level + 1).abs() + level
}

fn get_position_for_value(value: i32) {
    let level = get_level_for_value(value);
    println!("La distancia del nro {} es: {}",value, get_distance_for_number(value, level));
}

pub fn day_3_1_2017() {
    get_position_for_value(12);
}

