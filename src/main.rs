use std::io;
fn main() {
    test_match_with_option();
}

fn test_match_with_option() {
    let test_value: Option<i32> = Some(10);

    println!("{:?}", plus_one(test_value));
}

// fn plus_one_not_compilable(value : Option<i32>) -> Option<i32>{
//     match value {
//         Some(i) => Some(i+1)
//     }
// }

fn plus_one(value : Option<i32>) -> Option<i32>{
    match value {
        Some(i) => Some(i+1),
        None => None
    }
}

fn  test_match() {
    let coin = Coin::Dime;

    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            10
        },
        Coin::Quarter => 25,
    };

    println!("the value is {}", value);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn test_enum() {
    let four = IpAddrKind::V4(192, 168, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    println!("{:#?}", four);
}

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8 ,u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
struct Rectangle {
    width : i32,
    height : i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

impl Rectangle {
    fn square(length : i32) -> Rectangle {
        return Rectangle { width: length, height: length }
    }
}

fn struct_impl_test() {
    let mut first_rectangle = Rectangle {
        width : 50,
        height : 60,
    };
    
    println!("{}", first_rectangle.area());

    println!("my square with length of 30 : {:#?}", Rectangle::square(30));

}

fn struct_print_pretty() {
    let mut my_user = User {
        active : true,
        username: String::from("Jay"),
        email : String::from("jay@jay.com"),
        sign_in_count : 1,
    };
    
    // println!("my user = {:#?}", my_user);

    dbg!(&my_user);

    println!("{}", my_user.active);
}
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn test_struct_update_syntax() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("jay@test.com"),
        ..user1
    };
}

fn struct_example() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //If you want to change the field of structure, you declare the variable mutable
    user1.email = String::from("test@test.com");
}

fn build_user1(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

//You can use shordhand way to initiate a struct like this function
fn build_user2(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn test_borrowing() {
    let mut my_string = String::from("test");
    
    let var2 = &mut my_string;

    //If this line added, you will have an error that tells you cannot borrow more than once.
    // let var3 = &mut my_string;
    // println!("{}{}", var2, var3);
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