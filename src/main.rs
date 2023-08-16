use std::io;
fn main() {
    test_ownership()
}

fn test_ownership() {
    let mut test_var_a = String::from("test");

    let test_var_b = test_var_a;

    //This must show error and not compiled because it is borrowing value that is moved already.
    // println!("{}", test_var_a);

    println!("{}", test_var_b);

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

    if index <= 2 {
        println!("Requested value : 1");
    } else {
        let mut number1 = 1;
        let mut number2 = 1;
        let mut temp : i32;

        for _i in 2..index {
            temp = number1;
            number1 = number2;
            number2 = number1 + temp;
        };
        
        println!("Requested value : {}", number2);
    }
}