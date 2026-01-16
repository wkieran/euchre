use std::fmt;

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
            Suit::Clubs => "clurrbs",
            Suit::Diamonds => "diemonds",
            Suit::Hearts => "hurts",
            Suit::Spades => "spuds",
        };
        write!(f, "{}", symbol)
    }
}

struct Card {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.rank)
    }
}

#[derive(Debug)]
enum Team {
    East,
    West,
}

// impl fmt::Display for Team {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let s = match self {
//             Team::East => "east",
//             Team::West => "west",
//         };
//         write!(f, "{}", s)
//     }
// }

struct Player {
    id: usize,
    hand: Vec<Card>,
    team: Team,
    is_dealer: bool,
    tricks_won: usize,
    is_going_alone: bool,
}

// TODO contructor for Player
impl Player {
    fn new(&self){
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
