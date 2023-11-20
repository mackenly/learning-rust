mod utils;

use crate::utils::inputs::prompt_for_string;
use crate::utils::math::{adder, subtractor, multiplier, divider, squarer, cuber, power};

fn main() {
    println!("Adder, subtractor, multiplier, divider, squarer, cuber, power");
    let input: Vec<&str> = prompt_for_string("Enter two numbers separated by a space:").split(" ").collect();
    let a: i32 = input[0].parse::<i32>().unwrap();
    let b: i32 = input[1].parse::<i32>().unwrap();
    
    let input: &str = prompt_for_string("Enter an operation:");
    let result: i32 = match input {
        "adder" => adder(a, b),
        "subtractor" => subtractor(a, b),
        "multiplier" => multiplier(a, b),
        "divider" => divider(a, b),
        "squarer" => squarer(a),
        "cuber" => cuber(a),
        "power" => power(a, b),
        _ => panic!("Invalid operation!"),
    };
    println!("Result: {}", result);
}