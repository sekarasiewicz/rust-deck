#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
    let ranks = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
    ];

    for suit in suits {
        for rank in ranks {
            let card = format!("{} of {}", rank, suit);
            println!("{}", card);
        }
    }

    let deck = Deck { cards: vec![] };

    println!("Here is your deck: {:?}", deck);
}
