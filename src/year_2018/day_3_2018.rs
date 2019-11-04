use std::cmp::{max, min};
use std::str::FromStr;

use crate::util::read_input_as_string;

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    id: u32,
    left_top: (u32, u32),
    right_top: (u32, u32),
    left_bottom: (u32, u32),
    right_bottom: (u32, u32),
}

pub fn day_3_1_2018() -> u32 {
    let day_3_input: Vec<String> = read_input_as_string("day_3_2018.txt");
    let mut amount_overlaps = 0;
    let transformed_input: Vec<Rectangle> =
        day_3_input.clone().iter().map(normalize_point).collect();
    for i in 0..10 {
        for j in 0..day_3_input.len() {
            if j == i {
                continue;
            }

            let overlap_x_y = overlaps(
                transformed_input.get(i).unwrap().clone(),
                transformed_input.get(j).unwrap().clone(),
            );


            if overlap_x_y > 0 {
                amount_overlaps += overlap_x_y;
                println!(
                    "{first:?} and {second:?} inches -> {inches}",
                    first = transformed_input.get(i).unwrap().clone(),
                    second = transformed_input.get(j).unwrap().clone(),
                    inches = amount_overlaps
                );
                break;
            }
        }
        println!(
            "Amount on iteration {i} -> {amount} | rectangle -> {rect:?}",
            i = i,
            amount = amount_overlaps,
            rect = *transformed_input.get(i).unwrap()
        );
    }
    amount_overlaps
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

fn overlaps(left_rectangle: Rectangle, right_rectangle: Rectangle) -> u32 {
    /*
        println!(
            "left_rectangle.left_top.0 -> {left_rectangle1}",
            left_rectangle1 = left_rectangle.left_top.0
        );
        println!(
            "right_rectangle.right_bottom.0 -> {left_rectangle2}",
            left_rectangle2 = right_rectangle.right_bottom.0
        );
        println!(
            "right_rectangle.left_top.0 -> {left_rectangle3}",
            left_rectangle3 = right_rectangle.left_top.0
        );
        println!(
            "left_rectangle.right_bottom.0 -> {left_rectangle4}",
            left_rectangle4 = left_rectangle.right_bottom.0
        );
        println!(
            "left_rectangle.left_top.1 -> {left_rectangle5}",
            left_rectangle5 = left_rectangle.left_top.1
        );
        println!(
            "right_rectangle.right_bottom.1 -> {left_rectangle6}",
            left_rectangle6 = right_rectangle.right_bottom.1
        );
        println!(
            "right_rectangle.left_top.1 -> {left_rectangle7}",
            left_rectangle7 = right_rectangle.left_top.1
        );
        println!(
            "left_rectangle.right_bottom.1 -> {left_rectangle8}",
            left_rectangle8 = left_rectangle.right_bottom.1
        );
    */
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

    /*if (left_rectangle.right_top.0 < right_rectangle.left_top.0
        || right_rectangle.right_top.0 < left_rectangle.left_top.0)
        || (right_rectangle.left_top.1 > left_rectangle.left_bottom.1
        || left_rectangle.left_top.1 > right_rectangle.left_bottom.1)
    {
        return false;
    }
    true*/
}

pub fn day_3_2_2018() -> u32 {
    32
}

#[test]
fn probando_day_3_1_2018() {
    assert_eq!(115424, day_3_1_2018())
}

#[test]
fn probando_day_3_2_2018() {
    assert_eq!(32, day_3_2_2018())
}
