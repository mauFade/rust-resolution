#[derive(Debug)]
struct Person {
    name: String,
    email: String,
    active: bool,
}

pub(crate) fn struct_func() {
    let person = Person {
        active: true,
        email: "any@email".to_string(),
        name: "mau".to_string(),
    };

    println!("{:?}", person)
}
