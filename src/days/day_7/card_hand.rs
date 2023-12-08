use std::cmp::Ordering;
use std::collections::HashMap;
use crate::days::day_7::card_types::CardType;
use crate::days::day_7::hand_type::HandType;

#[derive(Debug, Clone)]
pub struct CardHand {
    pub(crate) cards: Vec<CardType>,
    pub(crate) points: i32,

    pub(crate) hand_type: HandType,
    pub(crate) strongest_hand: Vec<CardType>
}

impl CardHand {
    pub(crate) fn classify(&mut self) {
        let mut map: HashMap<CardType, i32> = HashMap::new();

        for card in &self.strongest_hand {
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

    pub(crate) fn get_strongest_hand(&mut self) {
        let mut all_ps = self.get_strongest_hand_helper(0, vec!());

        let mut all_hands = vec!();
        for ps in all_ps {
            let mut card_hand =
                CardHand { cards: ps.clone(), strongest_hand: ps.clone(), points: self.points, hand_type: self.hand_type };
            card_hand.classify();
            all_hands.push(
                card_hand
            )
        }

        all_hands.sort();
        all_hands.reverse();

        self.strongest_hand = all_hands[0].strongest_hand.clone();
    }

    pub(crate) fn get_strongest_hand_helper(&mut self, i: usize, mut vec: Vec<CardType>) -> Vec<Vec<CardType>>{
        let mut result = vec!();
        if self.cards[i] == CardType::VJ {
            for card in [ CardType::V2,
                                    CardType::V3,
                                    CardType::V4,
                                    CardType::V5,
                                    CardType::V6,
                                    CardType::V7,
                                    CardType::V8,
                                    CardType::V9,
                                    CardType::VT,
                                    CardType::VJ,
                                    CardType::VQ,
                                    CardType::VK,
                                    CardType::VA ] {
                if card == CardType::VJ {
                    continue;
                }

                let mut sub_vec = vec.clone();
                sub_vec.push(card);

                if i < self.cards.len() - 1 {
                    result.append(&mut self.get_strongest_hand_helper(i + 1, sub_vec));
                } else {
                    result.push(sub_vec);
                }
            }

        } else {
            vec.push(self.cards[i]);
            if i < self.cards.len() - 1 {
                result.append(&mut self.get_strongest_hand_helper(i + 1, vec));
            } else {
                result.push(vec);
            }
        }

        result
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