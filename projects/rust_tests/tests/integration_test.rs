#[path = "../src/utils.rs"]
mod utils;
use utils::math::{adder, subtractor, multiplier, divider, squarer, cuber, power};

#[test]
fn test_adder() {
    let a: i32 = 1;
    let b: i32 = 2;
    let input: &str = "adder";
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
    assert_eq!(result, 3);
}

#[test]
fn test_subtractor() {
    let a: i32 = 1;
    let b: i32 = 2;
    let input: &str = "subtractor";
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
    assert_eq!(result, -1);
}

#[test]
fn test_multiplier() {
    let a: i32 = 1;
    let b: i32 = 2;
    let input: &str = "multiplier";
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
    assert_eq!(result, 2);
}

#[test]
fn test_divider() {
    let a: i32 = 1;
    let b: i32 = 2;
    let input: &str = "divider";
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
    assert_eq!(result, 0);
}

#[test]
fn test_squarer() {
    let a: i32 = 2;
    let input: &str = "squarer";
    let result: i32 = match input {
        "adder" => adder(a, a),
        "subtractor" => subtractor(a, a),
        "multiplier" => multiplier(a, a),
        "divider" => divider(a, a),
        "squarer" => squarer(a),
        "cuber" => cuber(a),
        "power" => power(a, a),
        _ => panic!("Invalid operation!"),
    };
    assert_eq!(result, 4);
}

#[test]
fn test_cuber() {
    let a: i32 = 2;
    let input: &str = "cuber";
    let result: i32 = match input {
        "adder" => adder(a, a),
        "subtractor" => subtractor(a, a),
        "multiplier" => multiplier(a, a),
        "divider" => divider(a, a),
        "squarer" => squarer(a),
        "cuber" => cuber(a),
        "power" => power(a, a),
        _ => panic!("Invalid operation!"),
    };
    assert_eq!(result, 8);
}

#[test]
fn test_power() {
    let a: i32 = 2;
    let b: i32 = 3;
    let input: &str = "power";
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
    assert_eq!(result, 8);
}

#[test]
#[should_panic(expected = "Invalid operation!")]
fn test_invalid_operation() {
    let a: i32 = 2;
    let b: i32 = 3;
    let input: &str = "invalid";
    let _result: i32 = match input {
        "adder" => adder(a, b),
        "subtractor" => subtractor(a, b),
        "multiplier" => multiplier(a, b),
        "divider" => divider(a, b),
        "squarer" => squarer(a),
        "cuber" => cuber(a),
        "power" => power(a, b),
        _ => panic!("Invalid operation!"),
    };
}
