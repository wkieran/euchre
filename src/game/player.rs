use std::fmt;
use super::card::Card;

#[derive(Debug)]
pub enum Team {
    East,
    West,
}

pub struct Player {
    id: usize,
    hand: Vec<Card>,
    team: Team,
    is_dealer: bool,
    tricks_won: usize,
    is_going_alone: bool,
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

