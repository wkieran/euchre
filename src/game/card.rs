use std::fmt;

// TODO :
// - Team struct needs Copy, Clone for easier use
// - Card struct might need Clone or Copy depending on ownership model

#[derive(PartialOrd, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Rank {
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    fn same_color(&self, other: &Suit) -> bool {
        matches!(
            (self, other),
            (Suit::Spades, Suit::Clubs) | (Suit::Clubs, Suit::Spades) |
            (Suit::Hearts, Suit::Diamonds) | (Suit::Diamonds, Suit::Hearts)
        ) 
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            Suit::Clubs => "c",
            Suit::Diamonds => "d",
            Suit::Hearts => "h",
            Suit::Spades => "s",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

// TODO : test if this works if the right bower is checked
fn effective_suit(card: Card, trump_suit: Suit) -> Suit {
    if card.rank == Rank::Jack && card.suit.same_color(&trump_suit) {
        return trump_suit;
    }
    card.suit
}

fn effective_rank(card: Card, trump_suit: Suit) -> u8 {
    if card.rank == Rank::Jack{
        if card.suit == trump_suit{
            return 17;
        }
        if card.suit.same_color(&trump_suit){
            return 16;
        }
    }
    return card.rank as u8;
}

impl Card {
    fn beats(self, other_card: Card, trump_suit: Suit, lead_suit: Suit) -> bool {
        let self_is_trump = trump_suit == effective_suit(self, trump_suit);
        let other_is_trump = trump_suit == effective_suit(other_card, trump_suit);

        // case 1: one is trump, one isn't
        if self_is_trump && !other_is_trump {
            return true;
        }
        if !self_is_trump && other_is_trump {
            return false;
        }

        // case 2: both are trump
        if self_is_trump && other_is_trump {
            return effective_rank(self, trump_suit) > effective_rank(other_card, trump_suit);
        }

        // case 3: neither are trump
        let self_is_lead = effective_suit(self, trump_suit) == lead_suit;
        let other_is_lead = effective_suit(other_card, trump_suit) == lead_suit;

        if self_is_lead && !other_is_lead {
            return true;
        }
        if !self_is_lead && other_is_lead {
            return false;
        }

        if self_is_lead && other_is_lead {
            return self.rank > other_card.rank;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_non_trump_card_comparison() {
        let ace_hearts = Card {rank: Rank::Ace, suit: Suit::Hearts};
        let king_hearts = Card {rank: Rank::King, suit: Suit::Hearts};

        assert!(ace_hearts.beats(king_hearts, Suit::Spades, Suit::Hearts));
        assert!(!king_hearts.beats(ace_hearts, Suit::Spades, Suit::Hearts));
    }
}
