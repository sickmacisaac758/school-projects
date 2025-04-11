fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    println!("Original array: {:?}", data);
    
    // Randomly swap two elements in the array
    let random_index_1 = std::random::thread_rng().gen_range(0..data.len());
    let random_index_2 = std::random::thread_rng().gen_range(0..data.len());
    let temp = data[random_index_1];
    data[random_index_1] = data[random_index_2];
    data[random_index_2] = temp;
    
    println!("Array after swapping: {:?}", data);
}
