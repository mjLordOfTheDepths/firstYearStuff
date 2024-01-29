use std::vec;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: i32 = input.trim().parse().unwrap();

    let mut divisor = String::new();
    std::io::stdin().read_line(&mut divisor).unwrap();
    let mut divisor: i32 = divisor.trim().parse().unwrap();

    let mut neg: i8 = 0;

    if input < 0 {neg += 1;}

    if divisor < 0 {neg += 1; divisor = divisor.abs()}

    let mut numerator = input.to_string();

    let mut funnyVector: Vec<i32> = Vec::new();

    let mut newVector = Vec::new();

    let mut holdingNumber = 0;

    let mut counter = 0;

    let mut j = 0;

    if divisor == 0 {println!("Undefined")}

    else if numerator == "0" {print!("0")}

    else {
        for i in numerator.chars() {
            if let Some(digit) = i.to_digit(10) {
                funnyVector.push((digit as i32));
            }
        }
        
        for n in 0..funnyVector.len() {
            while (divisor + j) <= funnyVector[n] {j += divisor; holdingNumber = funnyVector[n] - j; counter += 1;}
            if divisor > funnyVector[n] {holdingNumber = funnyVector[n]}
            if n + 1 < funnyVector.len() {funnyVector[n + 1] += (holdingNumber * 10)}
            newVector.push(counter);
        }

        counter = 0;

        if neg == 1 {print!("-")} 
        for n in newVector {if n != 0 {print!("{}", &n)}}
        if (holdingNumber > 0) {
            holdingNumber *= 10;
            while holdingNumber >= divisor {divisor += divisor; counter += 1}
            print!(".{}", counter);
        }
    }   
}

// Needn't comment; self documenting.