use poker::Hand;
use std::str::FromStr;

fn main() {
    let hand = Hand::deal(5);
    println!("hand = {}", hand);
    println!("{}", hand.evaluate());

    let hand = Hand::from_str("A♥ K♥ Q♥ J♥ 10♥").unwrap();
    println!("\nhand = {}", hand);
    println!("{}", hand.evaluate());
}
