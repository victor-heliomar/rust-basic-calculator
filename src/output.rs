pub fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
      "{} {} {} = {:.2}",
      first_number, operator, second_number, result
    )
}