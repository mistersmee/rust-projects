use std::io;

fn main() {
    let mut measure = String::new();
    println!("Enter the temperature measure you are using: 'Celsius' for Celsius, 'Fahrenheit' for Fahrenheit.");

    io::stdin()
        .read_line(&mut measure)
        .expect("Please enter a string.");

    let mut temp = String::new();
    println!("Enter the temperature you wish to convert.");

    io::stdin()
        .read_line(&mut temp)
        .expect("Please enter a number.");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    if measure.trim() == "Celsius".to_string() {
        let fahrenheit = (temp * 1.8) + 32.0;
        println!("The temperature initially in Celsius when converted to Fahrenheit is: {fahrenheit}.");
    } else if measure.trim() == "Fahrenheit".to_string() {
        let celsius = (temp - 32.0) * (5.0 / 9.0);
        println!("The temperature initially in Fahrenheit when converted to Celsius is: {celsius}.")
    } else {
        println!("Please enter either 'Celsius' or 'Fahrenheit' only.");
    }
}
