pub fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
      '+' => first_number + second_number,
      '-' => first_number - second_number,
      '/' => first_number / second_number,
      '*' | 'X' | 'x' => first_number * second_number,
      _ => panic!("Invalid operator used."),
    }
}
  