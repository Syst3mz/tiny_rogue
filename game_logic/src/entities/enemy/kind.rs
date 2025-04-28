use core::fmt::{Display, Formatter};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Kind {
    Rat,
    Zombie,
    ZombieArcher,
    Lich(bool),
}

impl Kind {
    pub fn display_char(&self) -> char {
        match self {
            Kind::Rat => 'r',
            Kind::Zombie => 'z',
            Kind::ZombieArcher => 'a',
            Kind::Lich(_) => 'L',
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Kind::Rat => write!(f, "Rat"),
            Kind::Zombie => write!(f, "Zombie"),
            Kind::ZombieArcher => write!(f, "Zombie Archer"),
            Kind::Lich(_) => write!(f, "Lich"),
        }
    }
}