

fn main() {
    let boxes:Vec<String> = input.split("\n").map(|x| String::from(x)).collect();
    //let boxes:Vec<Vec<String>> = boxes.iter().map(|x| x.split("x").map(|y| String::from(y)).collect()).collect();
    for i in 0..boxes.len() {
        println!("{}", boxes[i]);
    }
}
