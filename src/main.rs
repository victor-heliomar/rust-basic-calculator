use std::env::{args, Args};

mod operate;
use operate::operate;

mod output;
use output::output;

fn main() {
    let mut args: Args = args();

    let first_number: f32 = args.nth(1).unwrap().parse::<f32>().unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second_number: f32 = args.nth(0).unwrap().parse::<f32>().unwrap();

    let result = operate(operator, first_number, second_number);
    
    println!("{}", output(first_number, operator, second_number, result));
}