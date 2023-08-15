use std::io;

fn main() {
    run_temperature_conversion()
}

fn run_temperature_conversion() {
    println!("Please enter your celsius degree to convert to farenheit.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed in read_line");
    
    let celsius : f32 = input.trim().parse().expect("Fail to parse. Your input is not in format of integer");
    println!("Converted value of celsius to farenheit is : {}", ((celsius * 1.8) + 32.0));
}