pub type Card = u8;

pub fn convert_card(card: char) -> Card {
    let card = card.to_ascii_uppercase();
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '2'..='9' => card.to_digit(10).unwrap() as u8,
        _ => panic!("unknown card '{}'", card),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_conversion() {
        assert_eq!(14, convert_card('A'));
        assert_eq!(13, convert_card('K'));
        assert_eq!(12, convert_card('Q'));
        assert_eq!(11, convert_card('J'));
        assert_eq!(10, convert_card('T'));
        assert_eq!(9, convert_card('9'));
        assert_eq!(8, convert_card('8'));
        assert_eq!(7, convert_card('7'));
        assert_eq!(6, convert_card('6'));
        assert_eq!(5, convert_card('5'));
        assert_eq!(4, convert_card('4'));
        assert_eq!(3, convert_card('3'));
        assert_eq!(2, convert_card('2'));
    }
}