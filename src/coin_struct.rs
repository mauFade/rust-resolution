use std::io;

struct Coin {
    value: u32,
}

impl Coin {
    fn new_coin(v: u32) -> Coin {
        return Coin { value: v };
    }

    fn get_value(&self) -> u32 {
        return self.value;
    }

    fn set_value(&mut self, v: u32) {
        self.value = v;
    }
}

pub(crate) fn coins() {
    let mut coin = Coin::new_coin(30);

    println!("Initial value: {}", coin.get_value());

    let mut new_value = String::new();

    println!("Input the new coin value");

    io::stdin()
        .read_line(&mut new_value)
        .expect("Error reading input value");

    coin.set_value(new_value.trim().parse::<u32>().unwrap());

    println!("New value: {}", coin.get_value());
}
