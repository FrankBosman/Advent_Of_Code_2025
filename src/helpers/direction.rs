use crate::helpers::Point;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn new(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '>' => Self::Right,
            '<' => Self::Left,
            _ => panic!("invalid field direction, {c}")
        }
    }

    pub fn to_point(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.eq(&Self::Left) || self.eq(&Self::Right)
    }

    pub fn is_vertical(&self) -> bool {
        self.eq(&Self::Up) || self.eq(&Self::Down)
    }

    pub fn directions() -> [Self;4] {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }

    pub fn is_opposite(&self, other: &Direction) -> bool {
        other.eq(&self.opposite())
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
