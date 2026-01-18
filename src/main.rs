use std::fmt;

// TODO :
// - Team struct needs Copy, Clone for easier use
// - Card struct might need Clone or Copy depending on ownership model

#[derive(PartialOrd, PartialEq, Copy, Clone)]
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

impl Card {
    fn beats(self, other_card: Card, trump_suit: Suit, lead_suit: Suit) -> bool {
        let is_self_trump = self.suit == trump_suit;
        let is_other_trump = other_card.suit == trump_suit;

        // if one is trump and other is not, return whether this card is trump
        if is_self_trump != is_other_trump {
            return is_self_trump;
        }

        // if both are trump, return highest value
        if is_self_trump {
            return self.rank > other_card.rank;
            // TODO : handle bowers here
        }

        // neither are trump
        let is_self_led = self.suit == lead_suit;
        let is_other_lead = other_card.suit == lead_suit;
        
        if is_self_led != is_other_lead {
            return is_self_led;
        }

        is_self_led && self.rank > other_card.rank
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
    fn play_card() {
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
    let test_card = Card{suit: Suit::Spades, rank: Rank::Ace};
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
