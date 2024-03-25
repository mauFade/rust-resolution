fn count(num: i32) {
    for i in 1..11 {
        println!("number {}: {}", i, num);
    }
}

fn count_down(num: i32) {
    let mut print_num = num;

    while print_num >= 1 {
        println!("number: {}", print_num);
        print_num = print_num - 1;
    }
}

fn main() {
    count(24);

    count_down(25);
}
