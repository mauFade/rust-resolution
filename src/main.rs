use std::io;

fn convert_str_to_int(param: &String) -> i32 {
    let output = param.trim().parse::<i32>().unwrap();

    output
}

fn factorial() {
    let mut input_value = String::new();
    let mut factorial = 1;

    io::stdin()
        .read_line(&mut input_value)
        .expect("Error reading input value");

    let mut input_i32 = convert_str_to_int(&input_value);

    while input_i32 > 1 {
        factorial = factorial * input_i32;

        input_i32 = input_i32 - 1;
    }

    println!("Factorial: {}", factorial);
}

fn main() {
    // factorial();
}
