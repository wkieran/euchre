use super::card::Card;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(usize)]
pub enum Team {
    East, // = 0
    West, // = 1
}

#[warn(unused_mut)]
pub struct Player {
    id: usize,
    pub hand: Vec<Card>,
    pub team: Team,
    pub is_going_alone: bool,
}

#[warn(unused_mut)]
impl Player {
    pub fn new(id: usize, team: Team) -> Self {
        Player {
            id,
            hand: vec![],
            team,
            is_going_alone: false,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Player {} (Team {:?}): ", self.id, self.team)?;

        for (i, card) in self.hand.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", card)?;
        }

        Ok(())
    }
}
