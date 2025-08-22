/// Methods for operating on cards and collections of cards.
///
/// ## Internal representation of card collections
/// The information below is not necessary to use the library but is useful to understand how the
/// data is being represented.
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
/// Ranks can be directly compared by casting them to an integer.
///
/// #### Color
/// The bit at place value 2^5 = `0b10_0000` represents the card's color. `0` means black; `1` means
/// red.
///
/// #### Suit
/// The bit at place value 2^4 = `0b1_0000` represents the card's suit.
///
/// For black cards, `0` means clubs or a black joker; `1` means spades.
///
/// For red cards, `0` means diamonds or a red joker; `1` means hearts.
///
/// When comparing two cards' suit, both the color and the suit must be compared because the meaning
/// of the suit bit is dependent on the color. For reference:
/// - **Clubs/black joker:** `00xxxx` binary or `0x` hex
/// - **Spades:** `01xxxx` binary or `1x` hex
/// - **Diamonds/red joker:** `10xxxx` binary or `2x` hex
/// - **Hearts:** `11xxxx` binary or `3x` hex
mod card;

fn main() {
    println!("Hello, world!");
}
