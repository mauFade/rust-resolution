fn avg(numbers: Vec<f64>) -> f64 {
    let total_items = numbers.len() as f64;
    let mut sum = 0.0;

    for i in &numbers {
        sum = sum + i;
    }

    println!("Total: {}, sum: {}", total_items, sum);

    return sum / total_items;
}

fn main() {
    let avg = avg(vec![1.1, 2.7, 3.8, 4.0, 9.8]);

    println!("MEDIA: {}", avg);
}
