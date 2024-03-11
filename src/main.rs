use std::io;

fn convert_str_to_int(param: &String) -> i32 {
    let output = param.trim().parse::<i32>().unwrap();

    return output;
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

fn students_avg() {
    let mut averages_qtd = String::new();

    println!("Input the students averages");

    io::stdin()
        .read_line(&mut averages_qtd)
        .expect("Error reading input value");

    let mut rec_sum = 0;
    let mut apr_sum = 0;
    let mut rep_sum = 0;
    let mut i = 0;

    while convert_str_to_int(&averages_qtd) > i {
        let mut student_avg = String::new();

        println!("Input student average");
        io::stdin()
            .read_line(&mut student_avg)
            .expect("Error reading input value");

        i += 1;

        if convert_str_to_int(&student_avg) < 3 {
            rep_sum += 1;
        }

        if convert_str_to_int(&student_avg) > 3 && convert_str_to_int(&student_avg) < 6 {
            rec_sum += 1;
        }

        if convert_str_to_int(&student_avg) > 6 {
            apr_sum += 1;
        }
    }

    println!(
        "Approved: {}, Recuperation: {}, Reproved: {}",
        apr_sum, rec_sum, rep_sum
    );
}

fn max_common_divisor() {
    let mut num1 = 15;
    let mut num2 = 40;

    while num2 != 0 {
        let temp = num2;

        num2 = num1 % num2;

        num1 = temp
    }

    println!("The maximum commom divisor is: {}", num1);
}

fn for_fn() {
    let animals: Vec<&str> = vec!["Cavalo", "Tigre", "Onça", "Macaco", "Cachorro", "Aguia"];

    for i in animals {
        println!("Animal: {}", i);
    }
}

fn main() {
    // factorial();

    // students_avg();

    // max_common_divisor();

    for_fn();
}
