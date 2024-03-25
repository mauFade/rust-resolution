fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    } else if num == 2 {
        return true;
    } else if num % 2 == 0 {
        return false;
    } else {
        let max_divisor = (num as f64).sqrt() as u64;
        for divisor in (3..=max_divisor).step_by(2) {
            if num % divisor == 0 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let exe_num = 18;

    if is_prime(exe_num) {
        println!("{} is prime", exe_num);
    } else {
        println!("{} is not prime", exe_num);
    }
}
