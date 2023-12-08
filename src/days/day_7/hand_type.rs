use std::cmp::Ordering;
use crate::days::day_7::card_types::CardType;

#[derive(Debug, Copy, Clone)]
pub enum HandType {
    FIVE,
    FOUR,
    FULL_HOUSE,
    THREE,
    TWO_PAIR,
    ONE_PAIR,
    HIGH
}

impl PartialEq<Self> for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd<Self> for HandType {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Option::from(u8::cmp(&self.value(), &other.value()))
    }
}

impl HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::FIVE => 7,
            HandType::FOUR => 6,
            HandType::FULL_HOUSE => 5,
            HandType::THREE => 4,
            HandType::TWO_PAIR => 3,
            HandType::ONE_PAIR => 2,
            HandType::HIGH => 1
        }
    }
}


