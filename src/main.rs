fn digital_root(n: i64) -> i64{
    let mut accumulator: i64 = 0;

    if n.to_string().chars().count() > 1{
        for letter in n.to_string().chars(){
            accumulator += letter.to_digit(10).unwrap() as i64;
        }
    }
    return accumulator;
}

fn main() {
    let x = 113;
    dbg!(digital_root(x));
}
