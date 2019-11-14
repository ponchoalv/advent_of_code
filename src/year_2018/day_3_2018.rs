use std::collections::HashMap;
use std::str::FromStr;

use crate::util::read_input_as_string;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rectangle {
    id: u32,
    left_top: (u32, u32),
    right_top: (u32, u32),
    left_bottom: (u32, u32),
    right_bottom: (u32, u32),
}

pub fn day_3_1_2018() -> Option<u32> {
    let day_3_input: Vec<String> = read_input_as_string("day_3_2018.txt");

    let transformed_input: Vec<Rectangle> =
        day_3_input.clone().iter().map(normalize_point).collect();

    println!(
        "this is equal {equal}, this are not {not_equal}",
        equal = ((3, 4) == (3, 4)),
        not_equal = (3, 4) == (4, 3)
    );

    match (
        get_min_x(transformed_input.clone()),
        get_max_x(transformed_input.clone()),
        get_min_y(transformed_input.clone()),
        get_max_y(transformed_input.clone()),
    ) {
        (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) => {
            println!(
                "min_x={min_x}, max_x={max_x}, min_y={min_y}, max_y={max_y}",
                min_x = min_x,
                max_x = max_x,
                min_y = min_y,
                max_y = max_y
            );

            let map_points = get_initial_map(Rectangle {
                id: 9999,
                left_top: (min_x, min_y),
                right_top: (max_x, min_y),
                left_bottom: (min_x, max_y),
                right_bottom: (max_x, max_y),
            });

            let counted_map = get_counted_points(map_points, transformed_input);

            Some(
                counted_map
                    .iter()
                    .filter(|((_, _), &count)| count > 1)
                    .count() as u32,
            )
        }
        _ => None,
    }
}

fn normalize_point(line: &String) -> Rectangle {
    let first_split: Vec<&str> = line.split(": ").collect();
    let size_string = first_split
        .clone()
        .get(1)
        .unwrap()
        .split("x")
        .collect::<Vec<&str>>();

    let size: (u32, u32) = vec_to_tuple(size_string);

    let id_and_origin = first_split
        .clone()
        .get(0)
        .unwrap()
        .split(" @ ")
        .collect::<Vec<&str>>();

    let id = u32::from_str(
        id_and_origin
            .clone()
            .get(0)
            .unwrap()
            .split("#")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap(),
    )
    .unwrap();

    let origin_string = id_and_origin
        .clone()
        .get(1)
        .unwrap()
        .split(",")
        .collect::<Vec<&str>>();

    let origin: (u32, u32) = vec_to_tuple(origin_string);

    Rectangle {
        id,
        left_top: origin,
        right_top: (origin.0 + size.0, origin.1),
        left_bottom: (origin.0, origin.1 + size.1),
        right_bottom: (origin.0 + size.0, origin.1 + size.1),
    }
}

fn vec_to_tuple(vec: Vec<&str>) -> (u32, u32) {
    (
        u32::from_str(vec.get(0).unwrap()).unwrap(),
        u32::from_str(vec.get(1).unwrap()).unwrap(),
    )
}

/*
Another implementation for checking overlaping between two rectangles.
fn count_overlaps_inchs(left_rectangle: Rectangle, right_rectangle: Rectangle) -> u32 {
    let x_overlap = max(
        0,
        min(left_rectangle.right_top.0, right_rectangle.right_top.0) as i32
            - max(left_rectangle.left_top.0, right_rectangle.left_top.0) as i32,
    ) as u32;

    let y_overlap = max(
        0,
        min(left_rectangle.left_bottom.1, right_rectangle.left_bottom.1) as i32
            - max(left_rectangle.left_top.1, right_rectangle.left_top.1) as i32,
    ) as u32;

    x_overlap * y_overlap
}
*/

fn is_overlaping(left_rectangle: Rectangle, right_rectangle: Rectangle) -> bool {
    if (left_rectangle.right_top.0 < right_rectangle.left_top.0
        || right_rectangle.right_top.0 < left_rectangle.left_top.0)
        || (right_rectangle.left_top.1 > left_rectangle.left_bottom.1
            || left_rectangle.left_top.1 > right_rectangle.left_bottom.1)
    {
        return false;
    }
    true
}

fn get_min_x(rectangles: Vec<Rectangle>) -> Option<u32> {
    rectangles
        .clone()
        .iter()
        .map(|rectangle| rectangle.left_top.0)
        .min()
}

fn get_max_x(rectangles: Vec<Rectangle>) -> Option<u32> {
    rectangles
        .clone()
        .iter()
        .map(|rectangle| rectangle.right_top.0)
        .max()
}

fn get_min_y(rectangles: Vec<Rectangle>) -> Option<u32> {
    rectangles
        .clone()
        .iter()
        .map(|rectangle| rectangle.left_top.1)
        .min()
}

fn get_max_y(rectangles: Vec<Rectangle>) -> Option<u32> {
    rectangles
        .clone()
        .iter()
        .map(|rectangle| rectangle.left_bottom.1)
        .max()
}

fn get_initial_map(rectangle: Rectangle) -> HashMap<(u32, u32), u8> {
    let x_range = rectangle.left_top.0..rectangle.right_top.0;
    let y_range = rectangle.left_top.1..rectangle.left_bottom.1;

    let mut initial_map: HashMap<(u32, u32), u8> =
        HashMap::with_capacity(x_range.len() * (y_range.len() * 2) as usize);

    for x in x_range {
        for y in y_range.clone() {
            initial_map.insert((x, y), 0);
        }
    }

    println!("Initial map len-> {map:?}", map = initial_map.len());
    initial_map
}

fn get_rectangle_points(rectangle: Rectangle) -> Vec<(u32, u32)> {
    let x_range = rectangle.left_top.0..rectangle.right_top.0;
    let y_range = rectangle.left_top.1..rectangle.left_bottom.1;

    let mut points: Vec<(u32, u32)> = Vec::with_capacity(x_range.len() * (y_range.len() * 2));

    for x in x_range {
        for y in y_range.clone() {
            points.push((x, y));
        }
    }

    points
}

fn get_counted_points(
    initial_map: HashMap<(u32, u32), u8>,
    rectangles: Vec<Rectangle>,
) -> HashMap<(u32, u32), u8> {
    let mut point_counter = initial_map.clone();

    let rectangles_points = rectangles
        .into_iter()
        .flat_map(get_rectangle_points)
        .collect::<Vec<(u32, u32)>>();
    println!(
        "rectangle_points len_vec={len_vec}",
        len_vec = rectangles_points.len()
    );

    for point in rectangles_points {
        point_counter.insert(point, point_counter[&point] + 1);
    }

    point_counter
}

pub fn day_3_2_2018() -> Option<u32> {
    let day_3_input: Vec<String> = read_input_as_string("day_3_2018.txt");

    let transformed_input: Vec<Rectangle> =
        day_3_input.clone().iter().map(normalize_point).collect();

    let mut result: Option<u32> = None;

    for left in transformed_input.clone() {
        let mut overlaps_counter = 0;

        for right in transformed_input.clone() {
            if left == right {
                continue;
            }

            overlaps_counter += if is_overlaping(left, right) { 1 } else { 0 }
        }

        if overlaps_counter == 0 {
            result = Some(left.id);
        }
    }

    result
}

#[test]
fn probando_day_3_1_2018() {
    assert_eq!(Some(117489), day_3_1_2018())
}

#[test]
fn probando_day_3_2_2018() {
    assert_eq!(Some(32), day_3_2_2018())
}
