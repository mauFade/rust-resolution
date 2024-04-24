pub(crate) fn max_profit(prices: Vec<i32>) {
    if prices.is_empty() {
        return;
    }

    let mut min_price = prices[0];
    let mut max_profit = 0;

    for price in prices.iter() {
        if *price < min_price {
            min_price = *price;
        } else if *price - min_price > max_profit {
            max_profit = *price - min_price;
        }
    }

    println!("Lucro máximo: {}", max_profit);
}
