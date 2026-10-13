// include memory functions for things like data types and variable sizes
use std::mem;

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
    data_types_test();
    type_conversions();
}

fn testing(num1: i32, num2: i32) {
    let mut result = num1 + num2;
    println!("Result = {}", result);
}

fn testing_floats(num1: f32, num2: f32) {
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
    let x: u8 = 4;

    let y: u32 = x as u32; // cast with `as`
    let y: u32 = u32::from(x); // From trait
    let y: u32 = x.into(); // Into trait (target type comes from the annotation)
    println!("t_or_f = {}", t_or_f);

    // characters and possibly determining size...
    let my_char: char = 'H';
    println!("my_char:   {} bytes", mem::size_of_val(&my_char)); // 4

    //*** Compound Data Types ***//
    // Arrays, Tuples, etc.

    // Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x = {}, y = {}, z = {}", tup.0, y, z);

    // Arrays
    let myarr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("myarr[0] = {}", myarr[0]);

    // Make a 2-D Array and print it to the console
    let mut grid = [[0usize; 4]; 4];

    for row in 0..4 {
        for col in 0..4 {
            grid[row][col] = row * 4 + col;
        }
    }

    for row in &grid {
        for value in row {
            print!("{:3}", value);
        }
        println!();
    }
}

// Converting types and considering data loss protection
fn type_conversions() {
    let x: u8 = 4;

    let y: u32 = x as u32; // cast with `as`
    println!("x = {}, y = {}", x, y);
    let y: u32 = u32::from(x); // From trait
    println!("x = {}, y = {}", x, y);
    let y: u32 = x.into(); // Into trait (target type comes from the annotation)
    println!("x = {}, y = {}", x, y);
}
