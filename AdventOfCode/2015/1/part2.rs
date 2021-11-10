
fn main() {
    let mut x: i32 = 0;
    let input = std::env::args().nth(1).expect("No argument passed");
    for i in 0..input.len() {
        x = match input.get(i..i+1).unwrap() {
            "(" => x+1,
            ")" => x-1,
            _ => panic!("Invalid argument"),
        };
        if x == -1 {
            println!("{}", i + 1);
            break;
        }
    }
}
