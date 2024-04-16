pub(crate) fn tuple() {
    let tuple = (23, "idade", (1, 2, 3), (100, 2, 3));
    let (a, b, c, d) = tuple;

    println!("{}, {}, {:?}, {:?}", a, b, c, d);
}
