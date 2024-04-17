// pub mod one_edit_away;
// pub mod palindrome_number;
// pub mod retroactive_string;
// pub mod string_compression;
// pub mod unique_characters;
// pub mod enum_func;
// pub mod tuple;

enum Payment {
    Money(f32),
    DebitCard(bool, f32),
    CreditCard(bool, f32),
}

fn payment_type(payment: Payment) {
    match payment {
        Payment::Money(f) => println!("Payment in money, {}", f),
        Payment::CreditCard(true, f) => println!("Payment in credit card, {}", f),
        Payment::DebitCard(true, f) => println!("Payment in debit card, {}", f),
        Payment::DebitCard(false, _) => println!("Payment wasn't made in debit card"),
        Payment::CreditCard(false, _) => println!("Payment wasn't made in credit card"),
    }
}

fn main() {
    // unique_characters::unique_characters("cateto");
    // retroactive_string::retroactive_string("tab", "mauricio");
    // one_edit_away::one_edit_away("mau", "mar");
    // string_compression::string_compression("Maaaaaaaauuuuu");
    // palindrome_number::palindrome_number(321);
    // tuple::tuple();
    // enum_func::enum_fn();

    payment_type(Payment::CreditCard(true, 25.0));
}
