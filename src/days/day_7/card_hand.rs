use std::cmp::Ordering;
use std::collections::HashMap;
use crate::days::day_7::card_types::CardType;
use crate::days::day_7::hand_type::HandType;

#[derive(Debug, Clone)]
pub struct CardHand {
    pub(crate) cards: Vec<CardType>,
    pub(crate) hand_type: HandType,
    pub(crate) points: i32
}

impl CardHand {
    pub(crate) fn classify(&mut self) {
        let mut map: HashMap<CardType, i32> = HashMap::new();

        for card in &self.cards {
            map.insert(card.clone(), map.get(&card).unwrap_or(&0) + 1);
        }

        let mut card_nums: Vec<i32> = map.values().map(|x| *x).collect();

        self.hand_type = match card_nums.len() {
            5 => HandType::HIGH,
            4 => HandType::ONE_PAIR,
            3 => {
                if card_nums.contains(&2) { HandType::TWO_PAIR } else { HandType::THREE }
            },
            2 => {
                if card_nums.contains(&3) { HandType::FULL_HOUSE } else { HandType::FOUR }
            },
            _ => HandType::FIVE
        };
    }
}


impl PartialEq<Self> for CardHand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }

        return self.cards[0] == other.cards[0];
    }
}

impl PartialOrd<Self> for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Your custom comparison logic here
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type);
        } else {
            for i in 0..self.cards.len() {
                match self.cards[i].partial_cmp(&other.cards[i]) {
                    Some(val) => {
                        match val {
                            Ordering::Greater |
                            Ordering::Less => {
                                return Option::from(val);
                            },
                            Ordering::Equal => {
                                continue;
                            }
                        }
                    },
                    None => { return None; }
                }
            }
        }

        return Option::from(Ordering::Equal);
    }
}

impl Eq for CardHand { }


impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.partial_cmp(&other.hand_type).unwrap();
        }
        self.cards[0].partial_cmp(&other.cards[0]).unwrap()
    }
}