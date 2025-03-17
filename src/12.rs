// A function that takes two numbers as arguments and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// A function that takes a number as an argument and returns its square
fn square(n: i32) -> i32 {
    n * n
}

// A function that takes three numbers as arguments and returns their product
fn multiply(a: i32, b: i32, c: i32) -> i32 {
    a * b * c
}

// A function that takes two numbers as arguments and returns their difference
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// A function that takes a number as an argument and returns its absolute value
fn abs(n: i32) -> i32 {
    if n > 0 {
        return n;
    } else {
        return -n;
    }
}
