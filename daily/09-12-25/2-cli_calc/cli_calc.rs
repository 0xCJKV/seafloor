use std::io;

fn main() {
    println!("-------- CLI Calculator --------");

    loop {
        print_menu();

        let choice = get_input("Make a selection: ");

        match choice.trim() {
            "1" => add_numbers(),
            "2" => subtract_numbers(),
            "3" => multiply_numbers(),
            "4" => divide_numbers(),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again!"),
        }
    }
}

fn print_menu() {
    println!("\nWhat would you like to do?");
    println!("1. Add two numbers");
    println!("2. Subtract two numbers");
    println!("3. Multiply two numbers");
    println!("4. Divide two numbers");
    println!("5. Exit");
    println!("--------------------------------");
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn get_number(prompt: &str) -> f64 {
    loop {
        let input = get_input(prompt);
        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn add_numbers() {
    let x = get_number("Enter the first number: ");
    let y = get_number("Enter the second number: ");
    println!("Addition: {} + {} = {}", x, y, x + y);
}

fn subtract_numbers() {
    let x = get_number("Enter the first number: ");
    let y = get_number("Enter the second number: ");
    println!("Subtraction: {} - {} = {}", x, y, x - y);
}

fn multiply_numbers() {
    let x = get_number("Enter the first number: ");
    let y = get_number("Enter the second number: ");
    println!("Multiplication: {} * {} = {}", x, y, x * y);
}

fn divide_numbers() {
    let x = get_number("Enter the first number: ");
    let y = get_number("Enter the second number: ");

    if y != 0.0 {
        println!("Division: {} / {} = {}", x, y, x / y);
    } else {
        println!("Error: Division by zero!");
    }
}
