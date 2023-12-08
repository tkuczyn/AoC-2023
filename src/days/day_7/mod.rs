pub mod card_types;
pub mod card_hand;
pub mod hand_type;

use crate::common_libs::files::get_lines_from_file;
use crate::days::day_7::card_hand::CardHand;
use crate::days::day_7::card_types::CardType;
use crate::days::day_7::hand_type::HandType;

pub(crate) fn solve(filename: &str) {
    let mut cards: Vec<CardHand> = vec!();

    for line in get_lines_from_file(filename) {
        let mut parts = line.split(" ");

        let vec = CardType::parse_cards(
            parts.nth(0)
                 .unwrap()
                 .to_string()
        );
        let points = str::parse::<i32>(parts.nth(0).unwrap()).unwrap();
        cards.push(CardHand {
            cards: vec,
            hand_type: HandType::HIGH,
            points: points,
        })
    }

    for mut card in cards.iter_mut() {
        card.classify();
    }
    cards.sort();

    let mut points:i32 = 0;
    for i in 0..cards.len() {
        points += (i as i32 + 1) * &cards[i].points;
    }

    println!("{}", points);
}

