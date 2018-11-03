use std::fmt::{self, Display};
use rand::prelude::*;


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Card(pub Suit, pub Rank);

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Suit {
    Club = 0,
    Diamond,
    Heart,
    Spade,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Rank {
    Deuce = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Board (pub [Card; 5]);
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Flop (pub [Card; 3]);
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Hand (pub [Card; 2]);

impl Display for Flop {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        match random::<u8>() % 6 {
            0 => write!(fmt, "{}{}{}", self.0[0], self.0[1], self.0[2]),
            1 => write!(fmt, "{}{}{}", self.0[0], self.0[2], self.0[1]),
            2 => write!(fmt, "{}{}{}", self.0[1], self.0[0], self.0[2]),
            3 => write!(fmt, "{}{}{}", self.0[1], self.0[2], self.0[0]),
            4 => write!(fmt, "{}{}{}", self.0[2], self.0[1], self.0[0]),
            5 => write!(fmt, "{}{}{}", self.0[2], self.0[0], self.0[1]),
            _ => unimplemented!(),
        }
    }
}

impl Display for Hand {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        write!(fmt, "{}{}", self.0[0], self.0[1])
    }
}

pub trait Evaluate {
    fn evaluate() -> (usize, String);
}

pub trait CardRepr {
    /// Convert card to unique integer
    fn to_int(&self) -> usize;
    /// Converts poker hand to a 52 bit u64 number
    /// |    Clubs   |  Diamonds  |  Hearts    |   Spades   |
    /// 23456789TJQKA23456789TJQKA23456789TJQKA23456789TJQKA
    /// 0001010010000000100000000000000000010000010000000001
    fn to_52(&self) -> u64;
}

impl Display for Card {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        write!(fmt, "{}{}", self.1, self.0)
    }
}

impl Display for Suit {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        use Suit::*;
        match self {
            Heart => write!(fmt, "h"),
            Club => write!(fmt, "c"),
            Spade => write!(fmt, "s"),
            Diamond => write!(fmt, "d")
        }
    }
}

impl Display for Rank {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), fmt::Error> {
        use Rank::*;
        match self {
            Ace => write!(fmt, "A"),
            Deuce => write!(fmt, "2"),
            Three => write!(fmt, "3"),
            Four => write!(fmt, "4"),
            Five => write!(fmt, "5"),
            Six => write!(fmt, "6"),
            Seven => write!(fmt, "7"),
            Eight => write!(fmt, "8"),
            Nine => write!(fmt, "9"),
            Ten => write!(fmt, "T"),
            Jack => write!(fmt, "J"),
            Queen => write!(fmt, "Q"),
            King => write!(fmt, "K"),
        }
    }
}

impl Suit {
    pub fn get(i: u64) -> Suit {
        use Suit::*;
        match i % 4 {
            0 => Club,
            1 => Diamond,
            2 => Heart,
            3 => Spade,
            _ => unimplemented!(),
        }
    }
    pub fn value(&self) -> u64 {
        use Suit::*;
        match *self {
            Club    => 0,
            Diamond => 1,
            Heart   => 2,
            Spade   => 3,
        }
    }
}

impl Rank {
    pub fn get(i: u64) -> Rank {
        use Rank::*;
        match i % 13 {
            0  => Deuce,
            1  => Three,
            2  => Four,
            3  => Five,
            4  => Six,
            5  => Seven,
            6  => Eight,
            7  => Nine,
            8  => Ten,
            9  => Jack,
            10 => Queen,
            11 => King,
            12 => Ace,
            _ => unimplemented!(),
        }
    }

    pub fn value(&self) -> u64 {
        use Rank::*;
        match *self {
            Deuce =>    0,
            Three =>    1,
            Four =>     2,
            Five =>     3,
            Six =>      4,
            Seven =>    5,
            Eight =>    6,
            Nine =>     7,
            Ten =>      8,
            Jack =>     9,
            Queen =>    10,
            King =>     11,
            Ace =>      12,
        }
    }
}

impl CardRepr for Card {
    fn to_int(&self) -> usize {
        (self.1.value() * 4 + self.0.value()) as usize
    }

    fn to_52(&self) -> u64 {
        let Card (suit, rank) = *self;
        let val = 1 << ((3-suit.value()) * 13 + (12 - rank.value()));
        val
    }
}

mod tests {
    #[test]
    fn test_card_value() {
        use super::Suit::*;
        use super::Rank::*;
        use super::{Card, CardRepr};
        assert_eq!(0b0000000000001, Card(Spade, Ace).to_52());
        assert_eq!(0b0000000000010, Card(Spade, King).to_52());
        assert_eq!(0b0000000000100, Card(Spade, Queen).to_52());
        assert_eq!(0b0000000001000, Card(Spade, Jack).to_52());
        assert_eq!(0b0000000010000, Card(Spade, Ten).to_52());
        assert_eq!(0b0000000100000, Card(Spade, Nine).to_52());
        assert_eq!(0b0000001000000, Card(Spade, Eight).to_52());
        assert_eq!(0b0000010000000, Card(Spade, Seven).to_52());
        assert_eq!(0b0000100000000, Card(Spade, Six).to_52());
        assert_eq!(0b0001000000000, Card(Spade, Five).to_52());
        assert_eq!(0b0010000000000, Card(Spade, Four).to_52());
        assert_eq!(0b0100000000000, Card(Spade, Three).to_52());
        assert_eq!(0b1000000000000, Card(Spade, Deuce).to_52());

        assert_eq!(0b00000000000010000000000000, Card(Heart, Ace).to_52());
        assert_eq!(0b00000000000100000000000000, Card(Heart, King).to_52());
        assert_eq!(0b00000000001000000000000000, Card(Heart, Queen).to_52());
        assert_eq!(0b00000000010000000000000000, Card(Heart, Jack).to_52());
        assert_eq!(0b00000000100000000000000000, Card(Heart, Ten).to_52());
        assert_eq!(0b00000001000000000000000000, Card(Heart, Nine).to_52());
        assert_eq!(0b00000010000000000000000000, Card(Heart, Eight).to_52());
        assert_eq!(0b00000100000000000000000000, Card(Heart, Seven).to_52());
        assert_eq!(0b00001000000000000000000000, Card(Heart, Six).to_52());
        assert_eq!(0b00010000000000000000000000, Card(Heart, Five).to_52());
        assert_eq!(0b00100000000000000000000000, Card(Heart, Four).to_52());
        assert_eq!(0b01000000000000000000000000, Card(Heart, Three).to_52());
        assert_eq!(0b10000000000000000000000000, Card(Heart, Deuce).to_52());

        assert_eq!(0b000000000000100000000000000000000000000, Card(Diamond, Ace).to_52());
        assert_eq!(0b000000000001000000000000000000000000000, Card(Diamond, King).to_52());
        assert_eq!(0b000000000010000000000000000000000000000, Card(Diamond, Queen).to_52());
        assert_eq!(0b000000000100000000000000000000000000000, Card(Diamond, Jack).to_52());
        assert_eq!(0b000000001000000000000000000000000000000, Card(Diamond, Ten).to_52());
        assert_eq!(0b000000010000000000000000000000000000000, Card(Diamond, Nine).to_52());
        assert_eq!(0b000000100000000000000000000000000000000, Card(Diamond, Eight).to_52());
        assert_eq!(0b000001000000000000000000000000000000000, Card(Diamond, Seven).to_52());
        assert_eq!(0b000010000000000000000000000000000000000, Card(Diamond, Six).to_52());
        assert_eq!(0b000100000000000000000000000000000000000, Card(Diamond, Five).to_52());
        assert_eq!(0b001000000000000000000000000000000000000, Card(Diamond, Four).to_52());
        assert_eq!(0b010000000000000000000000000000000000000, Card(Diamond, Three).to_52());
        assert_eq!(0b100000000000000000000000000000000000000, Card(Diamond, Deuce).to_52());

        assert_eq!(0b0000000000001000000000000000000000000000000000000000, Card(Club, Ace).to_52());
        assert_eq!(0b0000000000010000000000000000000000000000000000000000, Card(Club, King).to_52());
        assert_eq!(0b0000000000100000000000000000000000000000000000000000, Card(Club, Queen).to_52());
        assert_eq!(0b0000000001000000000000000000000000000000000000000000, Card(Club, Jack).to_52());
        assert_eq!(0b0000000010000000000000000000000000000000000000000000, Card(Club, Ten).to_52());
        assert_eq!(0b0000000100000000000000000000000000000000000000000000, Card(Club, Nine).to_52());
        assert_eq!(0b0000001000000000000000000000000000000000000000000000, Card(Club, Eight).to_52());
        assert_eq!(0b0000010000000000000000000000000000000000000000000000, Card(Club, Seven).to_52());
        assert_eq!(0b0000100000000000000000000000000000000000000000000000, Card(Club, Six).to_52());
        assert_eq!(0b0001000000000000000000000000000000000000000000000000, Card(Club, Five).to_52());
        assert_eq!(0b0010000000000000000000000000000000000000000000000000, Card(Club, Four).to_52());
        assert_eq!(0b0100000000000000000000000000000000000000000000000000, Card(Club, Three).to_52());
        assert_eq!(0b1000000000000000000000000000000000000000000000000000, Card(Club, Deuce).to_52());
    }

    #[test]
    fn test_evals() {
        use Suit::*;
        use Rank::*;
        use card::CardRepr;
        use Card;
        use evaluate::evaluate5;

        let a = Card(Club, Ace);
        let b = Card(Club, King);
        let c = Card(Club, Queen);
        let d = Card(Club, Jack);
        let e = Card(Club, Ten);
        let result = evaluate5(
            a.to_int(),
            b.to_int(),
            c.to_int(),
            d.to_int(),
            e.to_int(),
        );

        assert_eq!(1, result);

        let a = Card(Spade, Ace);
        let b = Card(Club, King);
        let c = Card(Club, Queen);
        let d = Card(Club, Jack);
        let e = Card(Club, Ten);
        let result = evaluate5(
            a.to_int(),
            b.to_int(),
            c.to_int(),
            d.to_int(),
            e.to_int(),
        );

        assert_eq!(1600, result);
    }
}