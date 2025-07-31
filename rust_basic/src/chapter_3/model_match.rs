pub enum Direction {
    East,
    West,
    North,
    South,
}

pub fn test() {
    let dire = Direction::East;
    let result = match dire {
        Direction::East => 0,
        Direction::North => {
            let mut total = 0;
            for i in 0..5 {
                total += i;
            }
            total
        }
        Direction::South | Direction::West => 1,
    };
    println!("{}", result);
}

// 有时会遇到只有一个模式的值需要被处理，其它值直接忽略的场景，如果用 match 来处理就不够优雅，那么就可以使用if let来实现这个功能
pub fn test_1() {
    let dire = Direction::East;
    if let Direction::East = dire {
        "east";
    } else {
        "other";
    };
}

// match范围匹配
pub fn test_2() {
    let i = 5;
    match i {
        0..5 => "OK",
        _ => "NO",
    };
}
