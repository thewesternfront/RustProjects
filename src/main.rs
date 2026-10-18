use std::io;

fn main() {
    println!("Hello, world!");
    let mut x = 100;
    println!("x = {}", x);
    x = 200;
    println!("x = {}", x);
    println!("Now I can change the variable because it is no longer immutable");
    println!("Calling a function");
    testing(11, 44);
    testing_floats(11.0, 22.0);
    test_input();
    math_and_overflows()
}

fn testing(num1: i32, num2: i32) {
    let mut result = num1 + num2;
    println!("Result = {}", result);
}

fn testing_floats(num1: f32, num2: f32) {
    let result = num1 + num2;
    println!("Result = {}", result);
}

fn test_input() {
    let mut myinput = String::new(); // allocate space
    io::stdin()
        .read_line(&mut myinput)
        .expect("Failed to read line");
    println!("You input: {}", myinput);
}

fn math_and_overflows() {
    let x: u8 = 255;
    let y: i8 = 10;

    println!("x = {}, y = {}", x, y);
}
