#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum GridDirection {
    North,
    East,
    South,
    West,
}

impl GridDirection {
    pub fn to_index(self) -> usize {
        match self {
            GridDirection::North => 0,
            GridDirection::East => 1,
            GridDirection::South => 2,
            GridDirection::West => 3,
        }
    }
}

