enum Direction {
    Up,
    Down,
    Right,
    Left,
}

fn change_direction(direction: &Direction, change: i32) -> Direction {
    match (direction, change) {
        (Direction::Up, 1) => Direction::Left,
        (Direction::Down, 1) => Direction::Right,
        (Direction::Left, 1) => Direction::Down,
        (Direction::Right, 1) => Direction::Up,
        (Direction::Up, _) => Direction::Up,
        (Direction::Down, _) => Direction::Down,
        (Direction::Left, _) => Direction::Left,
        (Direction::Right, _) => Direction::Right,
    }
}

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


fn get_spiral_iter(max: i32) -> impl Iterator<Item=(i32,i32)> {
    let mut direction = Direction::Up;
    let mut last_pair = (0, 0);
    let mut is_first = true;

    (2..(max + 1)).map(move |x| {
        let level = get_level_for_value(x) * 2;

        match direction {
            Direction::Up => {
                if is_first {
                    last_pair = (last_pair.0 + 1, last_pair.1);
                    is_first = false;
                } else {
                    last_pair = (last_pair.0, last_pair.1 + 1);
                }
                direction = change_direction(&direction, x % level);
            }
            Direction::Down => {
                last_pair = (last_pair.0, last_pair.1 - 1);
                direction = change_direction(&direction, x % level);
            }
            Direction::Left => {
                last_pair = (last_pair.0 - 1, last_pair.1);
                direction = change_direction(&direction, x % level);
            }
            Direction::Right => {
                last_pair = (last_pair.0 + 1, last_pair.1);
                direction = change_direction(&direction, x % level);
                is_first = x % level == 1;
            }
        }
        last_pair
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

#[test]
fn make_spiral() {
    let mut direction = Direction::Up;
    let mut last_pair = (0, 0);
    let mut is_first = true;

    let valores = (2..1025).map(move |x| {
        let level = get_level_for_value(x) * 2;

        match direction {
            Direction::Up => {
                if is_first {
                    last_pair = (last_pair.0 + 1, last_pair.1);
                    is_first = false;
                } else {
                    last_pair = (last_pair.0, last_pair.1 + 1);
                }
                direction = change_direction(&direction, x % level);
            }
            Direction::Down => {
                last_pair = (last_pair.0, last_pair.1 - 1);
                direction = change_direction(&direction, x % level);
            }
            Direction::Left => {
                last_pair = (last_pair.0 - 1, last_pair.1);
                direction = change_direction(&direction, x % level);
            }
            Direction::Right => {
                last_pair = (last_pair.0 + 1, last_pair.1);
                direction = change_direction(&direction, x % level);
                is_first = x % level == 1;
            }
        }
        last_pair
    }).collect::<Vec<(i32, i32)>>();

    println!("Valores de pares creados: {:?}", valores);
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