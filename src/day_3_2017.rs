use std::f32::consts::PI;

// Inicio Day 3 part 1 - 2017
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn distance_between_points(p1: Point, p2: Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn get_center_point_for_value(value: i32) -> Point {
    let value = ((value as f32).sqrt() / 2.0).round() as i32;

    Point {
        x:value,
        y:value
    }
}

fn get_position_for_value(value: i32) {
    let _center =  ((value as f32).sqrt() / 2.0).round() as i32;

    let center_point = get_center_point_for_value(value);

    println!("Center Value: {:?}",center_point);

    let mut valores: Vec<Point> = Vec::with_capacity(value as usize);

    for i in 0..value {
        let point = Point {
            x: (center_point.x as f32+ PI * (i as f32).sin().round()) as i32,
            y: (center_point.y as f32+ PI * (i as f32).cos().round()) as i32
        };

        valores.push(point);
    }

    println!("Valores vector: {:?}",valores)
}

pub fn day_3_1_2017(){
    let p1 = Point {x:2, y:2};
    let p2 = Point {x:1, y:4};

    let result = distance_between_points(p1, p2);

    println!("El resultado es: {}", result);

    println!("El origen para 12 es: {:?}", get_center_point_for_value(12));

    get_position_for_value(11);
}
