pub(crate) enum Payment {
    Money(f32),
    DebitCard(bool, f32),
    CreditCard(bool, f32),
}

pub(crate) fn payment_type(payment: Payment) {
    match payment {
        Payment::Money(f) => println!("Payment in money, {}", f),
        Payment::CreditCard(true, f) => println!("Payment in credit card, {}", f),
        Payment::DebitCard(true, f) => println!("Payment in debit card, {}", f),
        Payment::DebitCard(false, _) => println!("Payment wasn't made in debit card"),
        Payment::CreditCard(false, _) => println!("Payment wasn't made in credit card"),
    }
}
