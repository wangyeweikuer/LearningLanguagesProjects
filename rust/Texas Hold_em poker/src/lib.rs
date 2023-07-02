// Read me about poker first: https://cardguide.fandom.com/wiki/Pip_cards

// mod domain;

use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};
// use random::Source;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;
use std::collections::HashMap;
use crate::Rank::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};
use crate::Suit::{Clubs, Diamonds, Hearts, Spades};

#[derive(Debug, Clone, Copy, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn simplify(&self) -> char {
        match self {
            Spades => '♠',
            Hearts => '♥',
            Diamonds => '♦',
            Clubs => '♣'
        }
    }

    pub fn all() -> [Suit; 4] {
        [Spades, Hearts, Diamonds, Clubs]
    }
}

pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn simplify(&self) -> char {
        match self {
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Ten => 'T',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
            Ace => 'A',
        }
    }
    pub fn int(&self, big_ace: bool) -> i8 {
        match self {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten => 10,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => {
                match big_ace {
                    true => 14,
                    false => 1
                }
            }
        }
    }

    pub fn all() -> [Rank; 13] {
        [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace]
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.suit.simplify(), self.rank.simplify())
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {
            cards: Vec::new(),
        };
        let suits = Suit::all();
        let ranks = Rank::all();
        for suit in suits {
            for rank in ranks {
                deck.cards.push(Card { suit, rank });
            }
        }
        deck.shuffle();
        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();
        irs.shuffle(&mut self.cards, &mut rng);
    }

    pub fn deal(&mut self) -> Card {
        if self.cards.is_empty() {
            panic!("No cards to deal.");
        }
        self.cards.pop().unwrap()
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}

pub struct Player {
    name: String,
}

pub struct Playing<'a> {
    hand: Vec<Card>,
    player: &'a Player,
    round: Option<&'a Round<'a>>,
}

pub enum Category {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn partition_by<F, K>(cards: Vec<&Card>, pred: F) -> Vec<Vec<&Card>>
    where F: Fn(&Card) -> K {
    let mut map = HashMap::new();
    for card in cards {
        let k = pred(card);
        map.entry(k).or_insert(Vec::new()).push(card);
    }
    let mut vv = Vec::new();
    for x in map.values() {
        vv.push(x.to_vec());
    }
    vv
}

fn sort_as_acs_is_big(mut cards: &Vec<&Card>) {
    cards.sort_by(|&a, &b| a.rank.int(true).cmp(&b.rank.int(true)));
}

fn sort_as_acs_is_small(mut cards: &Vec<&Card>) {
    cards.sort_by(|&a, &b| a.rank.int(false).cmp(&b.rank.int(false)));
}


impl Category {
    fn find_straight(mut cards: Vec<&Card>) -> Option<[&Card; 5]> {
        if cards.len() < 5 {
            return None;
        }
        sort_as_acs_is_big(&cards);

    }

    fn find_straight_flush(cards: Vec<&Card>) -> Option<[&Card; 5]> {
        if cards.len() < 5 {
            return None;
        }
        let vv = partition_by(cards, |card| card.suit);
        let r = vv.iter().filter(|&x| x.len() >= 5).next();
        if r.is_none() {
            return None;
        }
        let r = r.unwrap();
        Category::find_straight(r.clone())
    }

    fn find_four_of_a_kind(cards: Vec<&Card>) -> Option<[&Card; 5]> {}
    fn find_full_house(cards: Vec<&Card>) -> Option<[&Card; 5]> {}
    fn find_flush(cards: Vec<&Card>) -> Option<[&Card; 5]> {}

    fn find_three_of_a_kind(cards: Vec<&Card>) -> Option<[&Card; 5]> {}
    fn find_two_pairs(cards: Vec<&Card>) -> Option<[&Card; 5]> {}
    fn find_one_pairs(cards: Vec<&Card>) -> Option<[&Card; 5]> {}
    fn find_high_card(cards: Vec<&Card>) -> [&Card; 5] {}
    pub fn category(community_cards: Vec<&Card>, player_cards: [&Card; 2]) -> Category {}
}

pub struct Round<'a> {
    players: Vec<&'a Player>,
    deck: Deck,
    flop: Option<[Card; 3]>,
    turn: Option<Card>,
    river: Option<Card>,
    playings: Vec<Playing<'a>>,
}

impl Round {
    pub fn new(players: Vec<&Player>) -> Round {
        let playings = vec![Playing; players.len()];
        for i in 0..players.len() {
            playings[i] = Playing {
                hand: Vec::new(),
                player: players[i],
                round: None,
            };
        }
        let r = Round {
            players,
            deck: Deck::new(),
            flop: None,
            turn: None,
            river: None,
            playings,
        };
        for x in playings {
            x.round = Some(&r)
        }
        r
    }

    pub fn deal_all(&mut self) {
        for p in self.players {
            Playing {}
        }
        let c = self.deal();
    }

    pub fn deal(&mut self) -> Card {
        self.deck.deal()
    }

    pub fn flop(&mut self) {
        self.deal();//discard a card
        let mut flop = [Card; 3];
        flop[0] = self.deck.deal();
        flop[1] = self.deck.deal();
        flop[2] = self.deck.deal();
        self.flop = Some(flop);
    }

    pub fn turn(&mut self) {
        self.deal();//discard a card
        self.turn = Option::from(self.deck.deal())
    }

    pub fn river(&mut self) {
        self.deal();//discard a card
        self.river = Option::from(self.deck.deal())
    }

    pub fn lookup_community_cards(&self) -> Vec<&Card> {
        let mut v = Vec::new();
        match self.river.as_ref() {
            None => {}
            Some(c) => v.push(c)
        }
        match self.turn.as_ref() {
            None => {}
            Some(c) => { v.push(c) }
        }
        match self.flop.as_ref() {
            None => {}
            Some(cs) => {
                for x in cs {
                    v.push(x)
                }
            }
        }
        v
    }

    pub fn get_winner(&self) -> &Playing {}
}
