use std::fmt;

// TODO :
// - Team struct needs Copy, Clone for easier use
// - Card struct might need Clone or Copy depending on ownership model

#[derive(PartialOrd, PartialEq, Copy, Clone)]
#[repr(u8)]
enum Rank {
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
enum Suit {
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
struct Card {
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
        let self_is_lead = (effective_suit(self, trump_suit) == lead_suit);
        let other_is_lead = (effective_suit(other_card, trump_suit) == lead_suit);

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

#[derive(Debug)]
enum Team {
    East,
    West,
}

struct Player {
    id: usize,
    hand: Vec<Card>,
    team: Team,
    is_dealer: bool,
    tricks_won: usize,
    is_going_alone: bool,
}

impl Player {
    fn new(&self, id: usize, team: Team) -> Option<Player>{
        todo!("implement player constructor");
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

struct Deck {
    cards: Vec<Card>,
}

// TODO : Deck methods
impl Deck {
    fn new() {
        todo!("implement new deck");
    }

    fn shuffle(&mut self) {
        todo!("implement shuffle on existing deck");
    }

    fn deal(&mut self) -> Option<Card> {
        todo!("implement dealing. pop card from vec");
    }

    fn reveal_top(&self) -> Card {
        todo!("flip the top card for revealing kitty card. doesn't pop from vec");
    }
}

struct Trick {
    played_cards: Vec<(usize, Card)>, // player ID, Card played
    lead_suit: Suit,
    trump_suit: Suit,
}

impl Trick {
    fn play_card(&self) {
        todo!("implement player play card in trick if it's their turn");
    }

    fn determine_winner() {
        todo!("determine player winner of trick if trick is over");
    }

    fn is_complete() {
        todo!("determine if trick is over");
    }

    fn clear() {
        todo!("prepare for next trick");
    }
}

enum Phase {
    Dealing,
    Bidding,
    Playing,
    Scoring,
}

struct GameState {
    // players, deck, current trick, tricks won, trump, kitty, 
    players: [Player; 4],
    deck: Deck,
    current_trick: Trick,
    tricks_won: [usize; 2],
    trump: Option<Suit>,
    kitty: Option<Card>,
    // current phase, current player, dealer
    current_phase: Phase,
    current_player: usize,
    dealer: usize,
    // team scores, maker team
    team_scores: [usize; 2],
    maker_team: Option<Team>,
}

fn main() {
    // card struct
    let test_card = Card{
        suit: Suit::Spades,
        rank: Rank::Ace
    };
    println!("test card : {}\n", test_card);

    // player struct
    let test_player = Player{
        id: 0,
        hand: vec!(),
        team: Team::East,
        is_dealer: false,
        tricks_won: 0,
        is_going_alone: false
    };

    println!("{}", test_player);

    let test_trick = Trick {
        played_cards: vec!(),
        lead_suit: Suit::Spades,
        trump_suit: Suit::Spades
    };
    test_trick.play_card();
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
