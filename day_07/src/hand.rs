use std::{collections::HashMap, cmp::Ordering};

use crate::{card::{Card, convert_card, JACK_CARD}, rules::Edition};

const HAND_SIZE: usize = 5;

pub type Hand = Vec<Card>;

pub fn parse_hand(hand: &str) -> Hand {
    assert_eq!(HAND_SIZE, hand.len());

    hand
        .chars()
        .map(|c| convert_card(c))
        .collect::<Vec<_>>()
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
pub trait HandOfCards {
    fn compute_type(&self, edition: Edition) -> HandType;
}

impl HandOfCards for Hand {
    fn compute_type(&self, edition: Edition) -> HandType {

        match edition {
            Edition::Standard => {
                let card_groups = create_card_groups(&self);

                let size_groups = create_size_groups(&card_groups);

                extract_hand_type(&size_groups)
            },
            Edition::JacksAreJokers => {
                let mut card_groups = create_card_groups(&self);

                let jokers = card_groups.remove(&JACK_CARD);

                let size_groups = create_size_groups(&card_groups);

                let hand_type = extract_hand_type(&size_groups);

                if let Some(jokers) = jokers {
                    if jokers == 5 {
                        HandType::FiveOfAKind
                    } else {
                        match hand_type {
                            HandType::FiveOfAKind => panic!("Five of a kind should be impossible with {} jokers", jokers),
                            HandType::FourOfAKind => HandType::FiveOfAKind,
                            HandType::FullHouse => panic!("Full house should be impossible with {} jokers", jokers),
                            HandType::ThreeOfAKind => {
                                match jokers {
                                    2 => HandType::FiveOfAKind,
                                    1 => HandType::FourOfAKind,
                                    _ => panic!("Three of a kind should be impossible with {} jokers", jokers),
                                }
                            },
                            HandType::TwoPair => HandType::FullHouse,
                            HandType::OnePair => {
                                match jokers {
                                    3 => HandType::FiveOfAKind,
                                    2 => HandType::FourOfAKind,
                                    1 => HandType::ThreeOfAKind,
                                    _ => panic!("One pair should be impossible with {} jokers", jokers),
                                }
                            },
                            HandType::HighCard => {
                                match jokers {
                                    4 => HandType::FiveOfAKind,
                                    3 => HandType::FourOfAKind,
                                    2 => HandType::ThreeOfAKind,
                                    1 => HandType::OnePair,
                                    _ => panic!("High card should be impossible with {} jokers", jokers)
                                }
                            }
                        }
                    }
                } else {
                    hand_type
                }
            }
        }
    }
}

type CardGroup = HashMap<u8, i32>;

fn create_card_groups(hand: &Hand) -> CardGroup {
    let mut card_groups = HashMap::new();

    for card in hand.iter() {
        let group = card_groups.entry(*card).or_insert(0);
        *group += 1;
    }

    card_groups
}

type SizeGroup = HashMap<i32, i32>;

fn create_size_groups(card_groups: &CardGroup) -> SizeGroup {
    let mut size_groups = HashMap::new();

    for (_, count) in card_groups {
        let group = size_groups.entry(*count).or_insert(0);
        *group += 1;
    }

    size_groups
}

fn extract_hand_type(size_groups: &HashMap<i32, i32>) -> HandType {
    if size_groups.contains_key(&5) {
        HandType::FiveOfAKind
    } else if size_groups.contains_key(&4) {
        HandType::FourOfAKind
    } else if size_groups.contains_key(&3) && size_groups.contains_key(&2) {
        HandType::FullHouse
    } else if size_groups.contains_key(&3) {
        HandType::ThreeOfAKind
    } else if let Some(count) = size_groups.get(&2) {
        match count {
            2 => HandType::TwoPair,
            1 => HandType::OnePair,
            _ => panic!("A hand of Camel Cards cannot have {} pairs", count)
        }
    } else {
        HandType::HighCard
    }
}

pub fn compare_hands(hand_a: &Hand, hand_b: &Hand, edition: Edition) -> Ordering {
    let type_a = hand_a.compute_type(edition);
    let type_b = hand_b.compute_type(edition);

    let convert_card = |card| {
        match edition {
            Edition::Standard => card,
            Edition::JacksAreJokers => {
                match card {
                    JACK_CARD => 1,
                    _ => card,
                }
            }
        }
    };
    match type_a.cmp(&type_b) {
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => {
            for i in 0..HAND_SIZE {
                let card_a = convert_card(hand_a[i]);
                let card_b = convert_card(hand_b[i]);

                match card_a.cmp(&card_b) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Less,
                    Ordering::Equal => {}
                }
            }

            Ordering::Equal
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hand_five_of_a_kind() {
        let hand = parse_hand("AAAAA");
        assert_eq!(HandType::FiveOfAKind, hand.compute_type(Edition::Standard));
    }

    #[test]
    fn hand_four_of_a_kind() {
        let hand = parse_hand("KK2KK");
        assert_eq!(HandType::FourOfAKind, hand.compute_type(Edition::Standard));
    }

    #[test]
    fn hand_full_house() {
        let hand = parse_hand("QJQJQ");
        assert_eq!(HandType::FullHouse, hand.compute_type(Edition::Standard));
    }

    #[test]
    fn hand_three_of_a_kind() {
        let hand = parse_hand("TT23T");
        assert_eq!(HandType::ThreeOfAKind, hand.compute_type(Edition::Standard));
    }

    #[test]
    fn hand_two_pair() {
        let hand = parse_hand("98298");
        assert_eq!(HandType::TwoPair, hand.compute_type(Edition::Standard));
    }

    #[test]
    fn hand_one_pair() {
        let hand = parse_hand("23747");
        assert_eq!(HandType::OnePair, hand.compute_type(Edition::Standard));
    }

    #[test]
    fn high_card() {
        let hand = parse_hand("65432");
        assert_eq!(HandType::HighCard, hand.compute_type(Edition::Standard));
    }

    #[test]
    fn compare_hands_example_1() {
        let hand_a = parse_hand("33332");
        let hand_b = parse_hand("2AAAA");

        assert_eq!(Ordering::Greater, compare_hands(&hand_a, &hand_b, Edition::Standard));
    }

    #[test]
    fn compare_hands_example_2() {
        let hand_a = parse_hand("77888");
        let hand_b = parse_hand("77788");

        assert_eq!(Ordering::Greater, compare_hands(&hand_a, &hand_b, Edition::Standard));
    }
}