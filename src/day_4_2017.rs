fn is_valid_passphrase(passphrase: &str) -> bool {
    let words = passphrase.split_whitespace();
    let len = words.len();
    let mut valid = true;

    for i in 1..len {
        valid = words.zip(words.cycle().skip(i)).filter(|(a,b)| {
            a == b
        }).len() == 0;
    }

    valid
}

#[test]
fn probar_day_4_1_2017() {
    let input = "aa bb cc dd ee";
    let result = is_valid_passphrase(input);
    println!("result: {:?}", result);

    let input = "aa bb cc dd aa";
    let result = is_valid_passphrase(input);
    println!("result: {:?}", result);


    let input = "aa bb cc dd aaa";
    let result = is_valid_passphrase(input);
    println!("result: {:?}", result);

}