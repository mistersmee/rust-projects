fn main() {
    another_function(5, 'h');

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is {x}{unit_label}");
}
