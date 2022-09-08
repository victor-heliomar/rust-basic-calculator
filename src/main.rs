use std::env::{args, Args};

mod operate;
use operate::operate;

mod output;
use output::output;

fn main() {
    let mut args: Args = args();

    let first_number: f32 = args.nth(1).expect("\nFirst number not found\nPlease remember add the numbers with this format `calculator 2 + 2`\n").parse::<f32>().expect("\nWe had problems to read the first number, remember that should use numbers and should have this format: `calculator 2 + 2`\n");
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second_number: f32 = args.nth(0).expect("\nSecond number not found\nPlease remember add the numbers with this format `calculator 2 + 2`\n").parse::<f32>().expect("\nWe had problems to read the second number, remember that should use numbers and should have this format: `calculator 2 + 2\n`");

    let result = operate(operator, first_number, second_number);
    
    println!("{}", output(first_number, operator, second_number, result));
}