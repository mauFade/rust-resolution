use std::io;

fn sum_numbers(numbers: Vec<i32>) -> i32 {
    let mut sum = 0;

    for i in 0..numbers.len() {
        if numbers[i] % 2 == 0 {
            println!("\nO número {} é par.", numbers[i]);
            sum = sum + numbers[i];
        }
    }

    return sum;
}

fn main() {
    let mut numbers = vec![];

    for i in 1..6 {
        let mut input = String::new();

        println!(
            "Insira o valor a ser somado (ímpares serão ignorados) [{}]",
            i
        );

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input value");

        numbers.push(input.trim().parse::<i32>().unwrap())
    }

    let output = sum_numbers(numbers);

    println!("Soma final: {}", output)
}
