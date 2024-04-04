use std::io;

fn show_multiplication_table(int: i32) {
    for i in 1..11 {
        println!("{}: {}", i, int * i);
    }
}

fn main() {
    let mut input = String::new();

    println!("Insira o valor a ser mostrado a tabuada");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input value");

    let num = input.trim().parse::<i32>().unwrap();

    show_multiplication_table(num)
}
