use std::cmp::Ordering;

fn load_input(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

fn get_tuple(data: &str) -> (&str, usize) {
    let joined_data = data.split_whitespace().take(2).collect::<Vec<&str>>();
    let name = joined_data[0];

    let num = joined_data[1].trim_start_matches('(').trim_end_matches(')').parse::<usize>().unwrap();

    (name,num)
}

fn get_vec_of_tuples(data: Vec<&str>) -> Vec<(&str,usize)> {
    data.iter().map(|&elem| {
        get_tuple(elem)
    }).collect()
}

fn get_min(data: &[(&str, usize)]) -> (String, usize) {
    let (name, num) = data.iter().min_by(|(_x, y), (_z, k)| {
        if y < k {
            Ordering::Less
        } else if y == k {
            Ordering::Equal
        }
        else {
            Ordering::Greater
        }
    }).unwrap();
    ((*name).to_owned(), *num)
}

pub fn day_7_1_2017(input: &str) -> String {
    let vector = load_input(input);
    let norm_vector = get_vec_of_tuples(vector);
    get_min(&norm_vector).0
}

#[test]
fn probando_day_7_1_2017() {
    let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    let vector = load_input(input);
    println!("Vector {:?}", vector);

    let norm_vector = get_vec_of_tuples(vector);
    println!("Norm Vector {:?}", norm_vector);

    let min = get_min(&norm_vector);
    println!("The root is {:?}", min)



}