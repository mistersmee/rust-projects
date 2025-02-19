use std::io;

fn main() {
    let mut number = String::new();

    println!("Enter a 'n' number to calculate the 'n'th Fibonacci number.");

    io::stdin()
        .read_line(&mut number)
        .expect("Enter a number.");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let result = nthfibonacci(number);
    println!("{result}");
}

fn nthfibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    return nthfibonacci(n - 1) + nthfibonacci(n - 2);
}
