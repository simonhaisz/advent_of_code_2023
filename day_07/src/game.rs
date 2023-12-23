use std::str::FromStr;

use crate::hand::{Hand, parse_hand, compare_hands};

type Bid = u32;

#[derive(Debug, PartialEq)]
struct Player (Hand, Bid);

pub struct Game {
    players: Vec<Player>,
    played: bool,
}

impl Game {
    fn new(players: Vec<Player>) -> Self {
        Self { players, played: false }
    }

    pub fn play(&mut self) {
        assert!(!self.played);
        self.players.sort_by(|a, b| compare_hands(&b.0, &a.0));
        self.played = true;
    }

    pub fn total_winnings(&self) -> u32 {
        assert!(self.played);

        let player_count = self.players.len();

        self.players.iter()
            .enumerate()
            .map(|(order, Player(_, bid))| (player_count - order) as u32 * bid)
            .sum()
    }
}

#[derive(Debug)]
pub struct GameParseError;

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut players = vec![];
        for line in text.lines() {
            if line.is_empty() {
                continue;
            }
            let mut components = line.split(" ");

            let hand = parse_hand(components.next().unwrap());
            let bid = components.next().unwrap().parse::<u32>().unwrap();
            players.push(Player(hand, bid));
        }

        Ok(Game::new(players))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let game = Game::from_str(
            r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
            ".trim()
        ).unwrap();

        assert_eq!(5, game.players.len());
        assert_eq!(Player(parse_hand("32T3K"), 765), game.players[0]);
        assert_eq!(Player(parse_hand("T55J5"), 684), game.players[1]);
        assert_eq!(Player(parse_hand("KK677"), 28), game.players[2]);
        assert_eq!(Player(parse_hand("KTJJT"), 220), game.players[3]);
        assert_eq!(Player(parse_hand("QQQJA"), 483), game.players[4]);
    }

    #[test]
    fn total_winnings_example() {
        let mut game = Game::from_str(
            r"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
            ".trim()
        ).unwrap();
        
        game.play();

        assert_eq!(6440, game.total_winnings());
    }
}