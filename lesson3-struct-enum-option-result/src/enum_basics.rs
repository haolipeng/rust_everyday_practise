//演示枚举类型的普通使用
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

//dir为不可变借用
fn move_player(dir: &Direction) {
    // match 必须穷举所有可能，漏一个都编译不过
    match dir {
        Direction::Up    => println!("⬆️ 向上走"),
        Direction::Down  => println!("⬇️ 向下走"),
        Direction::Left  => println!("⬅️ 向左走"),
        Direction::Right => println!("➡️ 向右走"),
    }
}

pub fn run() {
    println!("===== 场景2：枚举 =====");

    let dir = Direction::Up;
    move_player(&dir);
}