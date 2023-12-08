use std::cmp::Ordering;
use crate::days::day_7::card_hand::CardHand;
use crate::days::day_7::hand_type::HandType;

pub static mut J_VALUE: u8 = 11;

#[derive(Debug, Hash, Copy, Clone)]
pub enum CardType {
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VT,
    VJ,
    VQ,
    VK,
    VA
}

impl PartialEq<Self> for CardType {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.value() == other.value() }
    }
}

impl Eq for CardType{}

impl PartialOrd<Self> for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe { Option::from(u8::cmp(&self.value(), &other.value())) }
    }
}

impl Ord for CardType {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { u8::cmp(&self.value(), &other.value()) }
    }
}

impl CardType {
    unsafe fn value(&self) -> u8 {
        match self {
            CardType::V2 => 2,
            CardType::V3 => 3,
            CardType::V4 => 4,
            CardType::V5 => 5,
            CardType::V6 => 6,
            CardType::V7 => 7,
            CardType::V8 => 8,
            CardType::V9 => 9,
            CardType::VT => 10,
            CardType::VJ => J_VALUE,
            CardType::VQ => 12,
            CardType::VK => 13,
            CardType::VA => 14,
        }
    }

    pub(crate) fn parse_cards(line: String) -> Vec<CardType> {
        let mut vec = vec!();

        for i in 0..line.len() {
            let c = &line[i..i+1];
            let _type = match c {
                "2" => CardType::V2,
                "3" => CardType::V3,
                "4" => CardType::V4,
                "5" => CardType::V5,
                "6" => CardType::V6,
                "7" => CardType::V7,
                "8" => CardType::V8,
                "9" => CardType::V9,
                "T" => CardType::VT,
                "J" => CardType::VJ,
                "Q" => CardType::VQ,
                "K" => CardType::VK,
                "A" => CardType::VA,
                 _  => CardType::V2
            };
            vec.push(_type);
        }

        vec
    }
}

