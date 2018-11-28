fn get_level_for_value(number: i32) -> i32 {
    (((number as f32).sqrt() + 1.0) / 2.0).ceil() as i32 - 1
}

fn get_number_position_on_level(number: i32, level: i32) -> i32 {
    ((number - (2 * level - 1).pow(2)) % (2 * level))
}

pub fn day_3_1_2017(num: i32) -> i32 {
    if num == 1 {
        0
    } else {
        let level = get_level_for_value(num);
        let posicion = get_number_position_on_level(num, level);
        ((posicion - level).abs() + level)
    }
}

#[test]
fn probando_day_3_1_2017() {
    let input = 1;
    let distance = day_3_1_2017(input);
    assert_eq!(0, distance);

    let input = 2;
    let distance = day_3_1_2017(input);
    assert_eq!(1, distance);

    let input = 3;
    let distance = day_3_1_2017(input);
    assert_eq!(2, distance);

    let input = 12;
    let distance = day_3_1_2017(input);
    assert_eq!(3, distance);

    let input = 1024;
    let distance = day_3_1_2017(input);
    assert_eq!(31, distance);

    let input = 361527;
    let distance = day_3_1_2017(input);
    assert_eq!(326, distance);

}