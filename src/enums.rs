use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Copy, Clone)]
pub enum Board {
    Left = 0,
    Right = 1,
}

impl From<i32> for Board {
    fn from(n: i32) -> Self {
        match n {
            0 => Board::Left,
            1 => Board::Right,
            _ => panic!("expecting 0 or 1, got {}", n),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Orientation {
    None = 0,
    Horizontal = 1,
    Vertical = 2,
    Both = 3,
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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum PushDir {
    Up,
    Down,
    Left,
    Right,
}

impl PushDir {
    pub fn as_pair(&self) -> (i32, i32) {
        match self {
            PushDir::Up => (0, -1),
            PushDir::Down => (0, 1),
            PushDir::Left => (-1, 0),
            PushDir::Right => (1, 0),
        }
    }
}

impl Add<PushDir> for (i32, i32, Board) {
    type Output = (i32, i32, Board);

    fn add(self, rhs: PushDir) -> Self::Output {
        let offset = rhs.as_pair();
        (self.0 + offset.0, self.1 + offset.1, self.2)
    }
}

//  /\
// /21\
// \34/
//  \/
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Shape {
    Full = 0,
    TopRight = 1,
    TopLeft = 2,
    BottomLeft = 3,
    BottomRight = 4,
}

impl From<i32> for Shape {
    fn from(n: i32) -> Self {
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

impl Shape {
    pub fn opposite(&self) -> Shape {
        use Shape::*;
        match self {
            TopRight => BottomLeft,
            BottomLeft => TopRight,
            TopLeft => BottomRight,
            BottomRight => TopLeft,
            Full => Full,
        }
    }

    pub fn is_opposite(&self, other: &Shape) -> bool {
        use Shape::*;
        match (self, other) {
            (TopRight, BottomLeft)
            | (BottomLeft, TopRight)
            | (TopLeft, BottomRight)
            | (BottomRight, TopLeft) => true,
            _ => false,
        }
    }
}
