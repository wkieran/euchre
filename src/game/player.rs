use std::fmt;
use super::card::Card;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(usize)]
pub enum Team {
    East, // = 0
    West, // = 1
}

pub struct Player {
    id: usize,
    pub hand: Vec<Card>,
    pub team: Team,
    is_dealer: bool,
    tricks_won: usize,
    pub is_going_alone: bool,
}

impl Player {
    pub fn new(id: usize, team: Team) -> Self {
        let mut player = Player {
            id: id,
            hand: vec!(),
            team: team,
            is_dealer: false,
            tricks_won: 0,
            is_going_alone: false,
        };
        player
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

