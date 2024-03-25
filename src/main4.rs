fn int_vec(data: Vec<i32>) {
    let mut biggest_num = 0;

    for i in data {
        if i > biggest_num {
            biggest_num = i;
        }
    }

    println!("Biggest number is: {}", biggest_num)
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2024, 2, 4, 7, 10, 100, 1024];

    int_vec(numbers);
}
