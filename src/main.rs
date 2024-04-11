fn avg(numbers: Vec<i32>) -> f64 {
    let total_items = numbers.len() as f64;
    let mut sum = 0;

    for i in &numbers {
        sum = sum + i;
    }

    println!("Total: {}, sum: {}", total_items, sum);

    return (sum as f64) / total_items;
}

fn main() {
    let avg = avg(vec![1, 2, 3, 4]);

    println!("MEDIA: {}", avg);
}
