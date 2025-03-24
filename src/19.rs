fn main() {
    let mut count = 0;
    while true {
        println!("{}", count);
        count += 1;
        if count >= 1000 {
            break;
        }
    }
}
