# euchre

## game controller

### core data structures
- card representation
  - rank
  - suit
- hand
- deck
- trick
- player state
- game state

### card comparison logic:
The raw comparison logic will live in a method on Card. It will have the following signature:
```
card.beats(other_card, trump_suit, lead_suit) -> bool
```
This method will be called in a method on Trick. The Trick already knows the trump and lead suit so it only makes sense. The method will be:
```
trick.determine_winner().
```
Lastly, a method to determine a card's "effective suit" will be on Card, to dynamically check for the left bower:
```
effective_suit(card, trump) -> Suit
```

### going alone logic
need to think on this one.

### randomness in shuffling
Because it's fun, the random seed for deck shuffling will be sourced from millisecond time between the players readying-up. So for a 4 player game the 4 times will be XORed and logged.

### testing 

#### category 1: card comparison & trump logic
- [ ] ranking in non-trump suit
- [ ] trump beats non-trump
- [ ] right bower is highest trump
- [ ] left bower recognition
- [ ] following suit validation

#### category 2: trick winning logic
- [ ] lead suit wins when no trump player
- [ ] first trump played if only one
- [ ] highest trump wins

#### category 3: bidding phase
- [ ] all players pass on kitty
- [ ] dealer forces to call in second round
- [ ] player picks up kitty

#### category 4: scoring
- [ ] making team gets 3+ tricks (1 pt)
- [ ] making team gets all 5 tricks (2 pts)
- [ ] defending team wins (2 pts)
- [ ] going alone and winning 5 (4 pts)
