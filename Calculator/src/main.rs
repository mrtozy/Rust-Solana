use std::io;

#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b == 0.0 {
                panic!("Division by zero is not allowed.");
            } else {
                a / b
            }
        }
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let first_number: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear(); // Clear input buffer

    println!("Enter the operation (Add, Subtract, Multiply, Divide):");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let operation_input = input.trim().to_string(); // Convert to String to avoid borrowing issues

    input.clear(); // Clear input buffer

    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let second_number: f64 = input.trim().parse().expect("Please enter a valid number");

    let operation = match operation_input.as_str() { // Use as_str() to match the &str
        "Add" => Operation::Add(first_number, second_number),
        "Subtract" => Operation::Subtract(first_number, second_number),
        "Multiply" => Operation::Multiply(first_number, second_number),
        "Divide" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = calculate(operation);
    println!("The result is: {}", result);
}
