//TODO: Add doc tests for all of these functions!
// rng stands for "random number generator".
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

const SUITS: &str = "♣♦♥♠";
const RANKS: &str = "23456789TJQKA"; // T is for 10

// Integration tests go in the "tests" directory
// in files with any name and a ".rs" file extension.
// Unit tests go in the same file as the code they test.
// For example:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_suit_name() {
        assert_eq!(suit_name('♣'), "clubs");
    }
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: char,
    pub rank: char,
}

impl Display for Card {
    /// ```
    /// use std::str::FromStr;
    /// let card = poker::Card::from_str("K♥").unwrap();
    /// assert_eq!(format!("{}", card), "king of ♥");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write!(f, "{}{}", rank_name(self.rank), self.suit)
        write!(f, "{} of {}", rank_name(self.rank), self.suit)
    }
}

impl FromStr for Card {
    type Err = String;

    /// ```
    /// use std::str::FromStr;
    /// let card = poker::Card::from_str("K♥").unwrap();
    /// assert_eq!(card.rank, 'K');
    /// assert_eq!(card.suit, '♥');
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        //TODO: Consider fixing this so the rank can be "10" instead of "T".
        if let Some(rank) = iter.next() {
            if let Some(suit) = iter.next() {
                return Ok(Card { suit, rank });
            }
        }
        Err("bad card string".to_string())
    }
}

#[derive(Default)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Display for Hand {
    /// ```
    /// use std::str::FromStr;
    /// let hand = poker::Hand::from_str("K♥ 4♦ 9♥ J♠ A♦").unwrap();
    /// assert_eq!(format!("{}", hand), "king of ♥, 4 of ♦, 9 of ♥, jack of ♠, ace of ♦");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for (index, card) in self.cards.iter().enumerate() {
            if index != 0 {
                s.push_str(", ");
            }
            s.push_str(&format!("{}", card));
        }
        write!(f, "{}", s)
    }
}

impl FromStr for Hand {
    type Err = String;

    /// ```
    /// use std::str::FromStr;
    /// let hand = poker::Hand::from_str("K♥ 4♦ 9♥ J♠ A♦").unwrap();
    /// assert_eq!(hand.cards.len(), 5);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hand = Hand::default();
        for piece in s.split_whitespace() {
            if let Ok(card) = Card::from_str(piece) {
                hand.cards.push(card);
            }
        }
        Ok(hand)
    }
}
impl Hand {
    pub fn evaluate(&self) -> String {
        let mut suit_map = HashMap::new();
        for card in &self.cards {
            *suit_map.entry(card.suit).or_insert(0) += 1;
        }

        let mut rank_map = HashMap::new();
        for card in &self.cards {
            *rank_map.entry(card.rank).or_insert(0) += 1;
        }

        let flush = suit_map.values().any(|&count| count == 5);

        //TODO: Why do I need count here and *count on next line?
        let three_of_a_kind = rank_map.values().any(|&count| count == 3);
        let pairs = rank_map.values().filter(|&count| *count == 2);
        let pair_count = pairs.count();
        let two_of_a_kind = pair_count > 0;
        let two_pairs = pair_count == 2;
        let full_house = three_of_a_kind && two_of_a_kind;

        let straight = is_straight(self);

        let mut kind_count = 0;
        let mut kind_rank = ' ';
        for (rank, count) in rank_map {
            if count == kind_count {
                if rank_cmp(kind_rank, rank) > 0 {
                    kind_rank = rank;
                }
            } else if count > kind_count {
                kind_count = count;
                kind_rank = rank;
            }
        }

        let r = rank_name(kind_rank);

        if flush && kind_rank == 'A' {
            "royal flush".to_string()
        } else if straight && flush {
            "straight flush".to_string()
        } else if kind_count == 4 {
            format!("{} of a kind of {}s", kind_count, r)
        } else if full_house {
            "full house".to_string()
        } else if flush {
            "flush".to_string()
        } else if straight {
            "straight".to_string()
        } else if kind_count == 3 {
            format!("{} of a kind of {}s", kind_count, r)
        } else if two_pairs {
            "two pairs".to_string()
        } else if kind_count == 2 {
            format!("pair of {}s", r)
        } else {
            format!("high card {} of {}", r, get_suit(self, kind_rank))
        }
    }
}

/// ```
/// let hand = poker::deal(5);
/// assert_eq!(hand.cards.len(), 5);
/// ```
pub fn deal(n: usize) -> Hand {
    //let mut hand = Vec::new();
    let mut hand = Hand::default();
    while hand.cards.len() < n {
        let card = Card {
            suit: random_suit(),
            rank: random_rank(),
        };
        if !hand.cards.contains(&card) {
            hand.cards.push(card);
        }
    }
    hand
}

fn get_suit(hand: &Hand, rank: char) -> String {
    if let Some(card) = hand.cards.iter().find(|&c| c.rank == rank) {
        suit_name(card.suit)
    } else {
        String::from("not found")
    }
}

//TODO: This function seems to not be working.
fn is_straight(hand: &Hand) -> bool {
    let mut low_index = RANKS.len();
    let mut high_index = 0;
    for card in &hand.cards {
        let index = RANKS.find(card.rank).unwrap();
        if index > high_index {
            high_index = index;
        } else if index < low_index {
            low_index = index;
        }
    }
    high_index - low_index + 1 == hand.cards.len()
}

/// ```
/// assert_eq!(poker::rank_cmp('T', 'K'), 3);
/// assert_eq!(poker::rank_cmp('K', 'T'), -3);
/// ```
pub fn rank_cmp(rank1: char, rank2: char) -> i8 {
    let index1 = RANKS.find(rank1).unwrap() as i8;
    let index2 = RANKS.find(rank2).unwrap() as i8;
    index2 - index1
}

fn random_rank() -> char {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..RANKS.len());
    //ranks.as_bytes()[index]
    RANKS.chars().nth(index).unwrap()
}

fn random_suit() -> char {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..4);
    SUITS.chars().nth(index).unwrap()
}

/// ```
/// assert_eq!(poker::rank_name('T'), "10");
/// assert_eq!(poker::rank_name('J'), "jack");
/// assert_eq!(poker::rank_name('Q'), "queen");
/// assert_eq!(poker::rank_name('K'), "king");
/// assert_eq!(poker::rank_name('A'), "ace");
/// assert_eq!(poker::rank_name('2'), "2");
/// ```
pub fn rank_name(rank: char) -> String {
    let rank = match rank {
        // Why do I need to convert each of these to String?
        'T' => "10".to_string(),
        'J' => "jack".to_string(),
        'Q' => "queen".to_string(),
        'K' => "king".to_string(),
        'A' => "ace".to_string(),
        r => r.to_string(),
    };
    rank
}

/// ```
/// assert_eq!(poker::suit_name('♣'), "clubs");
/// assert_eq!(poker::suit_name('♦'), "diamonds");
/// assert_eq!(poker::suit_name('♥'), "hearts");
/// assert_eq!(poker::suit_name('♠'), "spades");
/// ```
pub fn suit_name(suit: char) -> String {
    let suit = match suit {
        '♣' => "clubs",
        '♦' => "diamonds",
        '♥' => "hearts",
        '♠' => "spades",
        _ => "invalid",
    };
    suit.to_string()
}
