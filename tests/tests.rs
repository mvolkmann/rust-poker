use poker;
use std::str::FromStr;

#[test]
fn it_gets_suit_name() {
    assert_eq!(poker::suit_name('♣'), "clubs");
}

#[test]
fn it_evaluates_hand() {
    let hand = poker::Hand::from_str("A♥ K♥ Q♥ J♥ T♥").unwrap();
    assert_eq!(hand.evaluate(), "royal flush");

    let hand = poker::Hand::from_str("Q♥ J♥ T♥ 9♥ 8♥").unwrap();
    assert_eq!(hand.evaluate(), "straight flush");

    //TODO: Finish these!
    //assert_eq!(hand.evaluate(), "four of a kind of queens");

    //assert_eq!(hand.evaluate(), "full house");

    //assert_eq!(hand.evaluate(), "flush");

    //assert_eq!(hand.evaluate(), "straight");

    //assert_eq!(hand.evaluate(), "three of a kind of queens");

    //assert_eq!(hand.evaluate(), "two pairs");

    //assert_eq!(hand.evaluate(), "pair of queens");

    //assert_eq!(hand.evaluate(), "high card ace of ♠");
}

//TODO: Add more tests.
