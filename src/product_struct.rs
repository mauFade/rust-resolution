struct Product {
    name: String,
    quantity: i32,
    price: f32,
}

impl Product {
    fn new_product(n: String, q: i32, p: f32) -> Product {
        return Product {
            name: n,
            quantity: q,
            price: p,
        };
    }

    fn set_name(&mut self, n: String) {
        self.name = n;
    }

    fn set_price(&mut self, p: f32) {
        self.price = p;
    }

    fn set_quantity(&mut self, q: i32) {
        self.quantity = q;
    }

    fn print_data(&self) {
        println!(
            "Name: {}\nPrice: {}\nQuantity: {}",
            self.name, self.quantity, self.price
        )
    }
}

pub(crate) fn product_struct() {
    let mut p = Product::new_product(String::from("INIT"), 10, 15.0);

    p.set_name(String::from("NOVO NOME"));
    p.set_price(200000.0);
    p.set_quantity(50000);

    p.print_data();
}
