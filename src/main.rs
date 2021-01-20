mod lib;
use lib::{deal, evaluate};

fn main() {
    let hand = deal(5);
    for card in &hand {
        println!("{}", card);
    }
    println!("\n{}", evaluate(&hand))
}
