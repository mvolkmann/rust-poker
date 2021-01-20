//TODO: Add doc tests for all of these functions!
// rng stands for "random number generator".
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

const SUITS: &str = "♣♦♥♠";
const RANKS: &str = "23456789TJQKA"; // T is for 10

#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: char,
    pub rank: char,
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
        if let Some(rank) = iter.next() {
            if let Some(suit) = iter.next() {
                return Ok(Card { suit, rank });
            }
        }
        Err("bad card string".to_string())
    }
}

pub type Hand = Vec<Card>;

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", rank_name(self.rank), suit_name(self.suit))
    }
}

/// ```
/// let hand = poker::deal(5);
/// assert_eq!(hand.len(), 5);
/// ```
pub fn deal(n: usize) -> Hand {
    let mut hand = Vec::new();
    while hand.len() < n {
        let card = Card {
            suit: random_suit(),
            rank: random_rank(),
        };
        if !hand.contains(&card) {
            hand.push(card);
        }
    }
    hand
}

pub fn evaluate(hand: &Hand) -> String {
    let mut suit_map = HashMap::new();
    for card in hand {
        *suit_map.entry(card.suit).or_insert(0) += 1;
    }

    let mut rank_map = HashMap::new();
    for card in hand {
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

    let straight = is_straight(hand);

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
        format!("high card {} of {}", r, get_suit(hand, kind_rank))
    }
}

fn get_suit(hand: &Hand, rank: char) -> String {
    if let Some(card) = hand.iter().find(|&c| c.rank == rank) {
        suit_name(card.suit)
    } else {
        String::from("not found")
    }
}

//TODO: This function seems to not be working.
fn is_straight(hand: &Hand) -> bool {
    let mut low_index = RANKS.len();
    let mut high_index = 0;
    for card in hand {
        let index = RANKS.find(card.rank).unwrap();
        if index > high_index {
            high_index = index;
        } else if index < low_index {
            low_index = index;
        }
    }
    high_index - low_index + 1 == hand.len()
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
