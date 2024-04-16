#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) fn enum_fn() {
    // let player: Direction = Direction::Down;
    // let player: Direction = Direction::Up;
    // let player: Direction = Direction::Left;
    let player: Direction = Direction::Down;

    // match player {
    //     Direction::Down => println!("Player wend down"),
    //     Direction::Up => println!("Player wend up"),
    //     Direction::Right => println!("Player wend right"),
    //     Direction::Left => println!("Player wend left"),
    // }

    println!("{:?}", player)
}
