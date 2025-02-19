fn main() {
    let number = 7;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
    else_if();
    iflet();
}

fn else_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("Number divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
}

fn iflet() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
