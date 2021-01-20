mod lib;
use lib::deal;
use std::str::FromStr;

fn main() {
    let hand = deal(5);
    println!("hand = {}", hand);
    for card in &hand.cards {
        println!("{}", card);
    }
    println!("\n{}", hand.evaluate());

    let hand = poker::Hand::from_str("A♥ K♥ Q♥ J♥ T♥").unwrap();
    println!("hand = {}", hand);
    println!("\n{}", hand.evaluate());
}
