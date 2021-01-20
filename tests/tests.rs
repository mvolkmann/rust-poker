use poker;
use std::str::FromStr;

#[test]
fn it_gets_suit_name() {
    assert_eq!(poker::suit_name('♣'), "clubs");
}

#[test]
fn it_evaluates_hand() {
    let hand = poker::Hand::from_str("A♥ K♥ Q♥ J♥ 10♥").unwrap();
    assert_eq!(hand.evaluate(), "royal flush");

    let hand = poker::Hand::from_str("Q♥ J♥ 10♥ 9♥ 8♥").unwrap();
    assert_eq!(hand.evaluate(), "straight flush");

    let hand = poker::Hand::from_str("Q♥ 7♥ Q♣ Q♦ Q♠").unwrap();
    assert_eq!(hand.evaluate(), "4 of a kind of queens");

    let hand = poker::Hand::from_str("Q♥ 7♥ Q♣ Q♦ 7♠").unwrap();
    assert_eq!(hand.evaluate(), "full house");

    let hand = poker::Hand::from_str("Q♥ 7♥ 3♥ A♥ 9♥").unwrap();
    assert_eq!(hand.evaluate(), "flush");

    // ♣ ♦ ♥ ♠
    let hand = poker::Hand::from_str("Q♥ 9♣ J♦ 8♠ 10♥").unwrap();
    assert_eq!(hand.evaluate(), "straight");

    let hand = poker::Hand::from_str("Q♥ 7♥ Q♣ Q♦ J♠").unwrap();
    assert_eq!(hand.evaluate(), "3 of a kind of queens");

    let hand = poker::Hand::from_str("Q♥ 7♥ Q♣ 5♦ 7♠").unwrap();
    assert_eq!(hand.evaluate(), "two pairs");

    let hand = poker::Hand::from_str("Q♥ 7♥ Q♣ 5♦ J♠").unwrap();
    assert_eq!(hand.evaluate(), "pair of queens");

    let hand = poker::Hand::from_str("Q♥ 7♥ J♣ 5♦ A♠").unwrap();
    assert_eq!(hand.evaluate(), "high card ace of ♠");
}

//TODO: Add more tests.
