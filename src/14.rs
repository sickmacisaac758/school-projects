fn main() {
    let numbers = vec![1, 2, 3, 4];
    let mut total = 0;
    for number in numbers {
        total += number;
    }
    println!("Total: {}", total);
}
