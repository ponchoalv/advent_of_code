// solución utilizando una struct e implementando el Trait Iterator.
#[derive(Copy, Clone)]
enum Direction {
    Up(bool),
    Down,
    Right,
    Left,
}

#[derive(Copy, Clone)]
struct SpiralStruct {
    data: (i32, i32),
    direction: Direction,
    count: i32,
}

trait SpiralGenerator: Iterator {
    fn update_direction(&mut self);
    fn update_data(&mut self);
}

impl SpiralStruct {
    fn new(data: (i32, i32), initial_direction: Direction) -> SpiralStruct {
        SpiralStruct {
            data,
            direction: initial_direction,
            count: 1,
        }
    }
}

impl SpiralGenerator for SpiralStruct {
    fn update_direction(&mut self) {
        self.direction = match &self.direction {
            Direction::Up(true) => Direction::Up(false),
            Direction::Up(false) => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up(true),
        }
    }

    fn update_data(&mut self) {
        self.data = match &self.direction {
            Direction::Up(true) => (self.data.0 + 1, self.data.1),
            Direction::Up(false) => (self.data.0, self.data.1 + 1),
            Direction::Down => (self.data.0, self.data.1 - 1),
            Direction::Left => (self.data.0 - 1, self.data.1),
            Direction::Right => (self.data.0 + 1, self.data.1),
        }
    }
}

impl Iterator for SpiralStruct {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let count = self.count;
        self.count += 1;
        let level = get_level_for_value(count) * 2;
        let change_direction = count % level == 1;

        match (&self.direction, change_direction, count) {
            (_, _, 1) => (),
            (Direction::Up(true), _, _) | (_, true, _) => {
                self.update_data();
                self.update_direction();
            }
            _ => self.update_data(),
        }
        Some(self.data)
    }
}

// se usa en todas las soluciones
fn get_level_for_value(number: i32) -> i32 {
    if number == 1 {
        1
    } else {
        (((number as f32).sqrt() + 1.0) / 2.0).ceil() as i32 - 1
    }
}


pub fn get_distance_with_spiral_struct(number: i32) -> i32 {
    let spiral_struct = SpiralStruct::new((0, 0), Direction::Up(true));

    spiral_struct.take(number as usize).last().map(|(a, b)| {
        (a as i32).abs() + (b as i32).abs()
    }).unwrap()
}


// solución con enfoque funcional. una función que devuelve un Iterator (impl Iterator<Item=T>)
fn update_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up(true) => Direction::Up(false),
        Direction::Up(false) => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
        Direction::Right => Direction::Up(true),
    }
}

fn update_pair(direction: Direction, pair: (i32, i32)) -> (i32, i32) {
    match direction {
        Direction::Up(true) => (pair.0 + 1, pair.1),
        Direction::Up(false) => (pair.0, pair.1 + 1),
        Direction::Down => (pair.0, pair.1 - 1),
        Direction::Left => (pair.0 - 1, pair.1),
        Direction::Right => (pair.0 + 1, pair.1),
    }
}


fn get_spiral_iter() -> impl Iterator<Item=(i32, i32)> {
    // setup initial values
    let mut direction = Direction::Up(true);
    let mut pair = (0, 0);

    (1..).map(move |x| {
        let level = get_level_for_value(x) * 2;
        let direction_change = x % level == 1;

        // change direction when needed and give first pair
        match (&direction, direction_change, x) {
            (_, _, 1) => pair,
            (Direction::Up(true), _, _) | (_, true, _) => {
                pair = update_pair(direction, pair);
                direction = update_direction(direction);
                pair
            }
            (_, false, _) => {
                pair = update_pair(direction, pair);
                pair
            }
        }
    })
}

pub fn get_distance_for_number(number: i32) -> i32 {
    get_spiral_iter().take(number as usize).last().map(|(a, b)| {
        a.abs() + b.abs()
    }).unwrap()
}

// Solución utilizando unicamente funciones matematicas:
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


// Part 2

struct SpiralWithMemory {
    data: i32,
    count: i32,
    calculated_data: Vec<((i32, i32), i32)>,
    spiral_genrator: SpiralStruct,
}

impl SpiralWithMemory {
    fn new() -> SpiralWithMemory {
        SpiralWithMemory {
            data: 0,
            count: 1,
            calculated_data: {
                let mut calc_vec = Vec::with_capacity(100);
                calc_vec.push(((0, 0), 1));
                calc_vec
            },
            spiral_genrator: SpiralStruct::new((0, 0), Direction::Up(true)),
        }
    }

    fn get_sum_of_adyacents(&self, pair: (i32, i32)) -> i32 {

        let x = pair.0;
        let y = pair.1;


        let pair_1 = &(x + 1, y);
        let pair_2 = &(x + 1, y + 1);
        let pair_3 = &(x, y + 1);
        let pair_4 = &(x - 1, y + 1);
        let pair_5 = &(x - 1, y);
        let pair_6 = &(x - 1, y - 1);
        let pair_7 = &(x, y - 1);
        let pair_8 = &(x + 1, y - 1);

        self.calculated_data.iter().filter(|(calc_pair, _)| {
            (calc_pair == pair_1 ||
                calc_pair == pair_2 ||
                calc_pair == pair_3 ||
                calc_pair == pair_4 ||
                calc_pair == pair_5 ||
                calc_pair == pair_6 ||
                calc_pair == pair_7 ||
                calc_pair == pair_8)
        }).map(|(_, value)| { value }).sum::<i32>()
    }
}

impl Iterator for SpiralWithMemory {
    type Item = i32;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let count = self.count;
        self.count += 1;
        let pair = self.spiral_genrator.take(count as usize).last().unwrap();
        self.data = self.get_sum_of_adyacents(pair);
        self.calculated_data.push((pair, self.data));

        Some(self.data)
    }
}

pub fn get_greater_than(input: i32) -> i32 {
    SpiralWithMemory::new().filter(move |&x| {x > input}).take(1).last().unwrap()
}

#[test]
fn test_memory_spiral() {
    let input = 1;
    let greater = get_greater_than(input);
    assert_eq!(2, greater);

    let input = 2;
    let greater = get_greater_than(input);
    assert_eq!(4, greater);

    let input = 3;
    let greater = get_greater_than(input);
    assert_eq!(4, greater);

    let input = 4;
    let greater = get_greater_than(input);
    assert_eq!(5, greater);

    let input = 22;
    let greater = get_greater_than(input);
    assert_eq!(23, greater);

    let input = 361_527;
    let greater = get_greater_than(input);
    assert_eq!(363010, greater)
}


#[test]
fn test_spiral_struct() {
    let input = 1;
    let distance = get_distance_with_spiral_struct(input);
    assert_eq!(0, distance);

    let input = 2;
    let distance = get_distance_with_spiral_struct(input);
    assert_eq!(1, distance);

    let input = 3;
    let distance = get_distance_with_spiral_struct(input);
    assert_eq!(2, distance);

    let input = 12;
    let distance = get_distance_with_spiral_struct(input);
    assert_eq!(3, distance);

    let input = 1024;
    let distance = get_distance_with_spiral_struct(input);
    assert_eq!(31, distance);

    let input = 361_527;
    let distance = get_distance_with_spiral_struct(input);
    assert_eq!(326, distance);

    let input = 361_527_234;
    let distance = get_distance_with_spiral_struct(input);
    assert_eq!(14051, distance);
}


#[test]
fn test_distance() {
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

    let input = 361_527_234;
    let distance = get_distance_for_number(input);
    assert_eq!(14051, distance);
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

    let input = 361_527_234;
    let distance = day_3_1_2017(input);
    assert_eq!(14051, distance);
}