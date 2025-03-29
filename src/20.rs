use std::fs;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();
    let lines: Vec<&str> = data.split("\n").collect();

    for line in lines {
        println!("{}", line);
    }
}
