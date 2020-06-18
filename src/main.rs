use std::fs;
fn main() {
    let contents = fs::read_to_string("1");
    println!("{:?}", contents);
}
