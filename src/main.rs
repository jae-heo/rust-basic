use std::io;
fn main() {
    calculate_fibonachi()
}

fn run_temperature_conversion() {
    println!("Please enter your celsius degree to convert to farenheit.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed in read_line");

    let celsius : f32 = input.trim().parse().expect("Fail to parse. Your input is not in format of number");
    println!("Converted value of celsius to farenheit is : {}", ((celsius * 1.8) + 32.0));
}

fn calculate_fibonachi() {
    println!("Please Enter the index of fibonachi.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed in read_line");
    let index : i32 = input.trim().parse().expect("Fail to parse. Your input is not in format of number");

    let mut number1 = 1;
    let mut number2 = 1;
    let mut temp : i32;

    if index <= 2 {
        println!("Requested value : 1");
    } else {
        for _i in 2..index {
            temp = number1;
            number1 = number2;
            number2 = number1 + temp;
        };
        println!("Requested value : {}", number2);
    }
}