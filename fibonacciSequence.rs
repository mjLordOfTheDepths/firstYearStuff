
fn main() {
    println("Enter what term of the Fibonacci Sequence you would like to display: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input: i128 = input.trim().parse().unwrap();

    let mut fib: i128 = 1;
    let mut fibPrevious: i128 = 0;
    let mut storageBucket: i128;
    let mut i = 2;

    for _ in 1..input {
        storageBucket = fib;
        fib += fibPrevious;
        fibPrevious = storageBucket;
        println!("Fib {} Contains: {}", i, fib);
        i += 1;
    }

    println!("The {} term in the sequence is: {}", input, fib);
}
