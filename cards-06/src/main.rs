#[allow(unused)]
#[derive(Debug, PartialEq)]
enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

impl Card {
    fn score(&self, current_score: usize) -> usize {
        use Card::*;
        match &self {
            Ace => {
                if current_score > 10 {
                    1
                } else {
                    11
                }
            }
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Jack | Queen | King => 10,
        }
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new() -> Self {
        Hand { cards: vec![] }
    }

    fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn value(&self) -> usize {
        let mut score = 0;

        for card in &self.cards {
            score += card.score(score);
        }

        // let mut aces_seen = 0;
        // // ...
        // for _ in 0..aces_seen {
        //     let ace_value = if score < 11 { 11 } else { 1 };
        //     score += ace_value;
        // }

        score
    }

    fn is_losing_hand(&self) -> bool {
        self.value() > 21
    }
}

fn main() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Ace);

    println!(
        "King, Seven & Ace is losing hand? {}. Score: {}",
        hand.is_losing_hand(),
        hand.value()
    );
}

#[test]
fn empty_hand() {
    let hand = Hand::new();

    assert_eq!(hand.value(), 0);
}

#[test]
fn strong_hand() {
    let mut hand = Hand::new();
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn risky_hand() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Queen);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 21);
}

#[test]
fn oops() {
    let mut hand = Hand::new();
    hand.add(Card::King);
    hand.add(Card::Seven);
    hand.add(Card::Five);

    assert!(hand.is_losing_hand());
    assert_eq!(hand.value(), 22);
}

#[test]
fn double_ace() {
    let mut hand = Hand::new();
    hand.add(Card::Ace);
    hand.add(Card::Ace);

    assert_eq!(hand.value(), 12);
}
