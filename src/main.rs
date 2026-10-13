fn main() {
    println!("Hello, world!");
    let mut x = 100;
    println!("x = {}", x);
    x = 200;
    println!("x = {}", x);
    println!("Now I can change the variable because it is no longer immutable");
    println!("Calling a function");
    testing(11,44);
    testing_floats(11.0, 22.0);
    data_types_test();
}


fn testing(num1: i32, num2: i32) {
    let mut result = num1 + num2;
    println!("Result = {}", result);
}

fn testing_floats(num1: f32, num2: f32 ) {
    let result = num1 + num2;
    println!("Result = {}", result);
}

fn data_types_test() {
    // integer
    let mut x = 5;
    println!("x = {}", x);

    // floating point
    let mut fpa: f32 = 123.456;
    let mut fpb: f32 = 321.321;
    println!("fpa = {}, fb = {}", fpa, fpb);
    println!("fpa + fpb = {}", fpa + fpb);

    // booleans
    let mut t_or_f: bool = true;
    println!("t_or_f = {}", t_or_f);
    t_or_f = false;
    println!("t_or_f = {}", t_or_f);
}