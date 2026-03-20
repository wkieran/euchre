use euchre::game::Deck;

fn main() {
    println!("Euchre!");
    let new_deck = Deck::new();
    for c in new_deck.cards {
        print!("{}, ", c);
    }
}
