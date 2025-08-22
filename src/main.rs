/// Methods for operating on cards and collections of cards.
///
/// ## Internal representation of card collections
///
/// Each card is given a unique **index number**. A collection of cards is represented as a
/// [Collection] -- a type alias for [u64]. When a card is present in a Collection, the bit at the
/// card's index number will be enabled.
///
/// Since there are 52 cards in the deck and up to 2 jokers, we will need at most 54 bits, so all
/// information fits in a 64-bit integer with 10 bits to spare.
///
/// ### Card index numbers
/// The index number of each card is assigned as described below. The index number is contained
/// within the low 6 bits, and all other bits should be zeroed.
///
/// #### Rank
/// The low four bits -- or the lowest place value hex digit -- of any index number are the card's
/// [Rank]. There are 14 ranks of cards from Ace to Joker.
///
/// #### Color
/// The bit at place value 2^5 = `0b10_0000` represents the card's color. `0` means black; `1` means
/// red.
///
///
mod card;

fn main() {
    println!("Hello, world!");
}
