// solución utilizando una struct e implementando el Trait Iterator.
enum DirectionIter {
    Up(bool),
    Down,
    Right,
    Left,
}

struct SpiralStruct<T> {
    data: T,
    direction: DirectionIter,
    count: i32,
}

impl SpiralStruct<(i32, i32)> {
    fn new(pair: (i32, i32), initial_direction: DirectionIter) -> SpiralStruct<(i32, i32)> {
        SpiralStruct {
            data: pair,
            direction: initial_direction,
            count: 2,
        }
    }

    fn update_direction(&mut self) {
        self.direction = match &self.direction {
            DirectionIter::Up(true) => DirectionIter::Up(false),
            DirectionIter::Up(false) => DirectionIter::Left,
            DirectionIter::Down => DirectionIter::Right,
            DirectionIter::Left => DirectionIter::Down,
            DirectionIter::Right => DirectionIter::Up(true),
        }
    }

    fn update_position(&mut self) {
        self.data = match &self.direction {
            DirectionIter::Up(true) => (self.data.0 + 1, self.data.1),
            DirectionIter::Up(false) => (self.data.0, self.data.1 + 1),
            DirectionIter::Down => (self.data.0, self.data.1 - 1),
            DirectionIter::Left => (self.data.0 - 1, self.data.1),
            DirectionIter::Right => (self.data.0 + 1, self.data.1),
        }
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

impl Iterator for SpiralStruct<(i32, i32)> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let count = self.count;
        self.count += 1;
        let level = get_level_for_value(count) * 2;
        let change_direction = count % level == 1;

        match (&self.direction, change_direction) {
            (DirectionIter::Up(true), _) | (_, true) => {
                self.update_position();
                self.update_direction();
            }
            (_, false) => self.update_position(),
        }

        Some(self.data)
    }
}

pub fn get_distance_with_spiral_struct(number: i32) -> i32 {
    SpiralStruct::new((0, 0), DirectionIter::Up(true))
        .take((number - 1) as usize).last().map(|(a, b)| {
        a.abs() + b.abs()
    }).unwrap()
}


// solución con enfoque funcional. una función que devuelve un Iterator (impl Iterator<Item=T>)
enum Direction {
    Up { is_first: bool, pair: (i32, i32) },
    Down { pair: (i32, i32) },
    Right { pair: (i32, i32) },
    Left { pair: (i32, i32) },
}

fn get_new_direction(direction: Direction, change_dir: bool) -> Direction {
    match (direction, change_dir) {
        (Direction::Up { pair, .. }, true) => Direction::Left { pair },
        (Direction::Down { pair }, true) => Direction::Right { pair },
        (Direction::Left { pair }, true) => Direction::Down { pair },
        (Direction::Right { pair }, true) => Direction::Up { is_first: true, pair },
        (Direction::Up { pair, .. }, false) => Direction::Up { is_first: false, pair },
        (Direction::Down { pair }, false) => Direction::Down { pair },
        (Direction::Left { pair }, false) => Direction::Left { pair },
        (Direction::Right { pair }, false) => Direction::Right { pair },
    }
}


fn get_spiral_iter(max: i32) -> impl Iterator<Item=(i32, i32)> {
    let mut direction = Direction::Up { is_first: true, pair: (0, 0) };

    (2..=max).map(move |x| {
        let level = get_level_for_value(x) * 2;

        match direction {
            Direction::Up { is_first: true, pair } => {
                let new_pair = (pair.0 + 1, pair.1);
                let dir = Direction::Up { is_first: true, pair: new_pair };
                direction = get_new_direction(dir, x % level == 1);
                new_pair
            }
            Direction::Up { is_first: false, pair } => {
                let new_pair = (pair.0, pair.1 + 1);
                let dir = Direction::Up { is_first: false, pair: new_pair };
                direction = get_new_direction(dir, x % level == 1);
                new_pair
            }
            Direction::Down { pair } => {
                let new_pair = (pair.0, pair.1 - 1);
                direction = get_new_direction(Direction::Down { pair: new_pair },
                                              x % level == 1);
                new_pair
            }
            Direction::Left { pair } => {
                let new_pair = (pair.0 - 1, pair.1);
                direction = get_new_direction(Direction::Left { pair: new_pair },
                                              x % level == 1);
                new_pair
            }
            Direction::Right { pair } => {
                let new_pair = (pair.0 + 1, pair.1);
                direction = get_new_direction(Direction::Right { pair: new_pair },
                                              x % level == 1);
                new_pair
            }
        }
    })
}

pub fn get_distance_for_number(number: i32) -> i32 {
    if number == 1 {
        0
    } else {
        get_spiral_iter(number).last().map(|(a, b)| {
            a.abs() + b.abs()
        }).unwrap()
    }
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


#[test]
fn test_spiral_struct() {
    let input = 1;
    let distance = get_distance_for_number(input);
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

    let input = 361527;
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