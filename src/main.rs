//make recursive function
fn recursive(n:i32) -> i32{
    if n > 1{
        return n*recursive(n-1);
    }
    return 1;
}


fn main() {
    dbg!(recursive(5));
}
