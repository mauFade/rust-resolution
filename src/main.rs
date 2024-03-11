use std::io;

fn convert_str_to_int(param: & String) -> i32 {
    let output = param.trim().parse::<i32>().unwrap();

    output
}

fn main() {
    let mut sum = 0;
    let mut input_value = String::new();

    io::stdin().read_line(&mut input_value).expect("Error reading input value");

    let mut i32_value = convert_str_to_int(&input_value);

    while i32_value != 0 {
        let r = i32_value%10;
        println!("R: {}", r);

        sum = sum + r;
        println!("sum: {}", sum);

        i32_value = i32_value / 10;
        println!("i32: {}", i32_value);

    }

    println!("Valor no final: {}", sum);

}
