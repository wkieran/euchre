use std::fmt;
use strum_macros::EnumIter;

#[derive(PartialOrd, PartialEq, Copy, Clone, EnumIter, Debug)]
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

#[derive(PartialEq, Copy, Clone, EnumIter, Debug)]
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
            (Suit::Spades, Suit::Clubs)
                | (Suit::Clubs, Suit::Spades)
                | (Suit::Hearts, Suit::Diamonds)
                | (Suit::Diamonds, Suit::Hearts)
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

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}

pub fn effective_suit(card: Card, trump_suit: Suit) -> Suit {
    if card.rank == Rank::Jack && card.suit.same_color(&trump_suit) {
        return trump_suit;
    }
    card.suit
}

fn effective_rank(card: Card, trump_suit: Suit) -> u8 {
    if card.rank == Rank::Jack {
        if card.suit == trump_suit {
            return 17;
        }
        if card.suit.same_color(&trump_suit) {
            return 16;
        }
    }
    card.rank as u8
}

impl Card {
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card {
            suit: suit,
            rank: rank,
        }
    }
    pub fn beats(self, other_card: Card, trump_suit: Suit, lead_suit: Suit) -> bool {
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

        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    //
    // Bower tests
    //
    #[test]
    fn test_right_bower_beats_trump_ace() {
        let trump_suit = Suit::Spades;

        let jack_spades = Card::new(Suit::Spades, Rank::Jack);
        let ace_spades = Card::new(Suit::Spades, Rank::Ace);

        assert!(jack_spades.beats(ace_spades, trump_suit, Suit::Hearts));
    }

    #[test]
    fn test_left_bower_beats_trump_ace() {
        let trump_suit = Suit::Spades;

        let jack_clubs = Card::new(Suit::Clubs, Rank::Jack);
        let ace_spades = Card::new(Suit::Spades, Rank::Ace);

        assert!(jack_clubs.beats(ace_spades, trump_suit, Suit::Hearts));
    }

    #[test]
    fn test_right_bower_beats_left_bower() {
        let trump_suit = Suit::Spades;

        let jack_spades = Card::new(Suit::Clubs, Rank::Jack);
        let jack_clubs = Card::new(Suit::Spades, Rank::Ace);

        assert!(jack_spades.beats(jack_clubs, trump_suit, Suit::Hearts));
    }

    #[test]
    fn test_left_bower_treated_as_trump() {
        let trump_suit = Suit::Spades;
        let lead_suit = Suit::Clubs;

        let jack_clubs = Card::new(Suit::Clubs, Rank::Jack);
        let king_clubs = Card::new(Suit::Clubs, Rank::King);

        assert!(jack_clubs.beats(king_clubs, trump_suit, lead_suit));
    }

    //
    //Trump vs non-trump tests
    //
    #[test]
    fn test_low_trump_beats_high_non_trump() {
        let trump_suit = Suit::Spades;
        let lead_suit = Suit::Hearts;

        let nine_spades = Card::new(Suit::Spades, Rank::Nine);
        let ace_hearts = Card::new(Suit::Hearts, Rank::Ace);

        assert!(nine_spades.beats(ace_hearts, trump_suit, lead_suit));
    }

    #[test]
    fn test_trump_beats_off_suit() {
        let trump_suit = Suit::Spades;
        let lead_suit = Suit::Hearts;

        let nine_spades = Card::new(Suit::Spades, Rank::Nine);
        let ace_diamonds = Card::new(Suit::Diamonds, Rank::Ace);

        assert!(nine_spades.beats(ace_diamonds, trump_suit, lead_suit));
    }

    //
    // Lead suit tests
    //
    #[test]
    fn test_lead_suit_beats_off_suit() {
        let trump_suit = Suit::Spades;
        let lead_suit = Suit::Hearts;

        let nine_hearts = Card::new(Suit::Hearts, Rank::Nine);
        let ace_diamonds = Card::new(Suit::Diamonds, Rank::Ace);

        assert!(nine_hearts.beats(ace_diamonds, trump_suit, lead_suit));
    }

    #[test]
    fn test_higher_card_wins_when_both_follow_lead() {
        let trump_suit = Suit::Spades;
        let lead_suit = Suit::Hearts;

        let ten_hearts = Card::new(Suit::Spades, Rank::Ten);
        let nine_hearts = Card::new(Suit::Spades, Rank::Nine);

        assert!(ten_hearts.beats(nine_hearts, trump_suit, lead_suit));
    }

    //
    // Edge case
    //
    #[test]
    fn test_both_off_suit_neither_wins() {
        let trump_suit = Suit::Spades;
        let lead_suit = Suit::Hearts;

        let ace_diamonds = Card::new(Suit::Diamonds, Rank::Ace);
        let king_clubs = Card::new(Suit::Clubs, Rank::King);

        assert!(!ace_diamonds.beats(king_clubs, trump_suit, lead_suit));
    }
}
