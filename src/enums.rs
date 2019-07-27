#[derive(Copy, Clone)]
pub enum Board {
    Left = 0,
    Right = 1,
}

impl From<u32> for Board {
    fn from(n: u32) -> Self {
        match n {
            0 => Board::Left,
            1 => Board::Right,
            _ => panic!("expecting 0 or 1, got {}", n),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Orientation {
    Both = 0,
    Horizontal = 1,
    Vertical = 2,
}

impl From<u32> for Orientation {
    fn from(n: u32) -> Self {
        match n {
            0 => Orientation::Both,
            1 => Orientation::Horizontal,
            2 => Orientation::Vertical,
            _ => panic!("expecting 0..2, got {}", n),
        }
    }
}

#[derive(Copy, Clone)]
pub enum PushDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub enum Shape {
    Full = 0,
    TopRight = 1,
    TopLeft = 2,
    BottomLeft = 3,
    BottomRight = 4,
}

impl From<u32> for Shape {
    fn from(n: u32) -> Self {
        match n {
            0 => Shape::Full,
            1 => Shape::TopRight,
            2 => Shape::TopLeft,
            3 => Shape::BottomLeft,
            4 => Shape::BottomRight,
            _ => panic!("expecting 0..4, got {}", n),
        }
    }
}
