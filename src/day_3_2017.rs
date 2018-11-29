enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn get_new_direction(direction: &Direction, change_dir: bool) -> Direction {
    match (direction, change_dir) {
        (Direction::Up, true) => Direction::Left,
        (Direction::Down, true) => Direction::Right,
        (Direction::Left, true) => Direction::Down,
        (Direction::Right, true) => Direction::Up,
        (Direction::Up, false) => Direction::Up,
        (Direction::Down, false) => Direction::Down,
        (Direction::Left, false) => Direction::Left,
        (Direction::Right, false) => Direction::Right,
    }
}

fn get_level_for_value(number: i32) -> i32 {
    (((number as f32).sqrt() + 1.0) / 2.0).ceil() as i32 - 1
}

fn get_spiral_iter(max: i32) -> impl Iterator<Item=(i32,i32)> {
    let mut direction = Direction::Up;
    let mut actual_pair = (0, 0);
    let mut is_first = true;

    (2..=max).map(move |x| {
        let level = get_level_for_value(x) * 2;

        match direction {
            Direction::Up => {
                if is_first {
                    actual_pair = (actual_pair.0 + 1, actual_pair.1);
                    is_first = false;
                } else {
                    actual_pair = (actual_pair.0, actual_pair.1 + 1);
                }
                direction = get_new_direction(&direction, x % level == 1);
            },
            Direction::Down => {
                actual_pair = (actual_pair.0, actual_pair.1 - 1);
                direction = get_new_direction(&direction, x % level == 1);
            },
            Direction::Left => {
                actual_pair = (actual_pair.0 - 1, actual_pair.1);
                direction = get_new_direction(&direction, x % level == 1);
            },
            Direction::Right => {
                actual_pair = (actual_pair.0 + 1, actual_pair.1);
                let has_change = x % level == 1;
                direction = get_new_direction(&direction, has_change);
                is_first = has_change;
            },
        }
        actual_pair
    })
}

pub fn get_distance_for_number(number: i32) -> i32{
    if number == 1 {
        0
    } else {
        get_spiral_iter(number).last().map(|(a,b)| {
            a.abs() + b.abs()
        }).unwrap()
    }
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
fn test_distance(){
    let input = 1;
    let distance = get_distance_for_number(input);
    assert_eq!(0, distance);

    let input = 2;
    let distance = get_distance_for_number(input);
    assert_eq!(1, distance);

    let input = 3;
    let distance = get_distance_for_number(input);
    assert_eq!(2, distance);

    let input = 12;
    let distance = get_distance_for_number(input);
    assert_eq!(3, distance);

    let input = 1024;
    let distance = get_distance_for_number(input);
    assert_eq!(31, distance);

    let input = 361527;
    let distance = get_distance_for_number(input);
    assert_eq!(326, distance);
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