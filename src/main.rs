fn main() {
    println!("Hello, world!");
    let mut x = 100;
    println!("x = {}", x);
    x = 200;
    println!("x = {}", x);
    println!("Now I can change the variable because it is no longer immutable");
    println!("Calling a function");
    testing(11,44);
    testing_floats(11.0, 22.0)
}


fn testing(num1: i32, num2: i32) {
    let mut result = num1 + num2;
    println!("Result = {}", result);
}

fn testing_floats(num1: f32, num2: f32 ) {
    let result = num1 + num2;
    println!("Result = {}", result);
}