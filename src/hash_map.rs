use std::collections::HashMap;

pub(crate) fn hash_map() {
    let mut hash_map = HashMap::new();

    hash_map.insert("Calculo 2", 10);
    hash_map.insert("Eletromag", 8);
    hash_map.insert("POO", 10);
    hash_map.insert("EDO", 7);
    hash_map.insert("Fisica", 8);

    match hash_map.get("Calculo 2") {
        Some(k) => println!("Passei em calculo com {}, {}", k, hash_map.len()),
        None => println!("Não fiz calculo"),
    }
}
