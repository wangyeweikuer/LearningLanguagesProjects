// Read me about poker first: https://cardguide.fandom.com/wiki/Pip_cards

// mod domain;

use std::cmp::Ordering;
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
  pub fn int(&self, ace_as_big: bool) -> i8 {
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
        match ace_as_big {
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

impl Card {
  fn rank(&self, ace_as_big: bool) -> int {
    self.rank.int(ace_as_big)
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
  hand: [Card; 2],
  player: &'a Player,
  round: Option<&'a Round<'a>>,
}

#[derive(Ord, Debug, Ordering)]
pub enum Category {
  StraightFlush = 0,
  FourOfAKind = 1,
  FullHouse = 2,
  Flush = 3,
  Straight = 4,
  ThreeOfAKind = 5,
  TwoPairs = 6,
  OnePair = 7,
  HighCard = 8,
}

impl PartialEq<Category> for Category {
  fn eq(&self, other: &Category) -> bool {}

  fn ne(&self, other: &Category) -> bool {
    todo!()
  }
}

struct BestMatch<'a> {
  playing: &'a Playing<'a>,
  cards: Vec<&'a Card>,
  category: Category,
}

impl<'a> BestMatch<'a> {
  fn compare(&self, other: &Self) -> Ordering {}
}

fn partition_by<'a, F, K>(cards: &'a Vec<&'a Card>, pred: F) -> Vec<Vec<&'a Card>>
where F: Fn(&Card) -> K {
  let mut map = HashMap::new();
  for &card in cards {
    let k = pred(card);
    map.entry(k).or_insert(Vec::new()).push(card);
  }
  let mut vv = Vec::new();
  for x in map.values() {
    vv.push(x.to_vec());
  }
  vv
}

fn sort_reversed(mut cards: &Vec<&Card>, ace_as_big: bool) {
  cards.sort_by(|&a, &b| b.rank(ace_as_big).cmp(&a.rank(ace_as_big)));
}

fn find_straight_inner<'a>(mut cards: &'a Vec<&'a Card>, ace_as_big: bool) -> Option<[&'a Card; 5]> {
  sort_reversed(&cards, ace_as_big);
  let mut found = [&Card; 5];
  for i in 0..cards.len() - 4 {
    let now = cards[i].rank(ace_as_big);
    found[0] = &cards[i];
    for j in 1..5 {
      if cards[j + i].rank(ace_as_big) + j != now {
        break;
      }
      found[j] = &cards[j + i];
    }
    Some(found)
  }
  None
}

fn find_same_ranks<'a, F>(mut cards: &'a Vec<&'a Card>, sort_first: bool, same_num: usize, filter: F)
                          -> Option<Vec<usize>>
where F: FnMut(usize) -> bool {
  if sort_first {
    sort_reversed(cards, true);
  }

  let mut res = vec![usize; same_num];
  for i in 0..cards.len() - same_num + 1 {
    if !filter(i) {
      continue;
    }
    res[0] = i;
    let mut ok = true;
    for j in 1..same_num {
      if cards[i].rank(true) == cards[i + j].rank(true) {
        res[j] = i + j;
      } else {
        ok = false;
        break;
      }
    }
    if ok {
      return Some(res);
    }
  }
  None
}

fn find_any<'a, F>(cards: &'a Vec<&'a Card>, same_num: usize, filter: F) -> Vec<&'a Card>
where F: FnMut(usize) -> bool {
  let mut res = vec![&Card; same_num];
  for i in 0..cards.len() {
    if filter(i) {
      res.push(cards[i]);
    }
  }
  res
}

fn find_straight_flush<'a>(mut cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let vv = partition_by(cards, |card| card.suit);
  let r = vv.iter().filter(|&x| x.len() >= 5).next();
  if r.is_none() {
    return None;
  }
  let r = r.unwrap();
  let found = find_straight_inner(r, true);
  return match found {
    None => find_straight_inner(r, false),
    Some(_) => found
  };
}

fn find_four_of_a_kind<'a>(mut cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let found = find_same_ranks(cards, true, 4, || true);
  return match found {
    None => None,
    Some(x) => {
      let other = find_any(cards, 1, |idx| !x.contains(&idx));
      Some([&cards[x[0]], &cards[x[1]], &cards[x[2]], &cards[x[3]], other[0]])
    }
  };
}

fn find_full_house<'a>(cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let found = find_same_ranks(cards, true, 3, || true);
  match found {
    None => None,
    Some(x) => {
      let other = find_same_ranks(cards, false, 2, |idx| {
        !x.contains(&idx)
      }).unwrap();
      Some([&cards[x[0]], &cards[x[1]], &cards[x[2]], &cards[other[0]], &cards[other[1]]])
    }
  }
}

fn find_flush<'a>(cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let parts = partition_by(cards, |card| { card.suit });
  let r = parts.iter().filter(|&x| x.len() >= 5).next();
  match r {
    None => None,
    Some(x) => Some([x[0], x[1], x[2], x[3], x[4]])
  }
}

fn find_straight<'a>(cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let x = find_straight_inner(cards, true);
  match x {
    None => find_straight_inner(cards, false),
    Some(x) => Some(x)
  }
}

fn find_three_of_a_kind<'a>(cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let found = find_same_ranks(cards, true, 3, || true);
  match found {
    None => None,
    Some(x) => {
      let others = find_any(cards, 2, |idx| !x.contains(&idx));
      Some([&cards[x[0]], &cards[x[1]], &cards[x[2]], others[0], others[1]])
    }
  }
}

fn find_two_pairs<'a>(cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let found = find_same_ranks(cards, true, 2, || true);
  if found.is_none() {
    return None;
  }
  let x = found.unwrap();
  let x1 = find_same_ranks(cards, false, 2, |idx| !x.contains(&idx));
  match x1 {
    None => None,
    Some(_) => {
      let other = find_any(cards, 1, |idx| !x.contains(&idx) && !x1.contains(&idx));
      Some([&cards[x[0]], &cards[x[1]], &cards[x1[0]], &cards[x1[1]], other[0]])
    }
  }
}

fn find_one_pairs<'a>(cards: &'a Vec<&'a Card>) -> Option<[&'a Card; 5]> {
  let found = find_same_ranks(cards, true, 2, || true);
  match found {
    None => { None }
    Some(x) => {
      let others = find_any(cards, 3, |idx| !x.contains(&idx));
      Some([&cards[x[0]], &cards[x[1]], others[0], others[1], others[2]])
    }
  }
}

fn find_high_card<'a>(cards: &'a Vec<&'a Card>) -> [&'a Card; 5] {
  sort_reversed(cards, true);
  [cards[0], cards[1], cards[2], cards[3], cards[4]]
}

fn find_best_composition<'a>(community_cards: &'a Vec<Card>, player_cards: &[Card; 2]) -> (Category, [&'a Card; 5]) {
  let &mut cards = Vec::new();
  community_cards.for_each(|c| cards.push(c));
  player_cards.for_each(|c| cards.push(c));
  let x = find_straight_flush(cards);
  if x.is_some() {
    return (Category::StraightFlush, x.unwrap());
  }
  let x = find_four_of_a_kind(cards);
  if x.is_some() {
    return (Category::FourOfAKind, x.unwrap());
  }
  let x = find_full_house(cards);
  if x.is_some() {
    return (Category::FullHouse, x.unwrap());
  }
  let x = find_flush(cards);
  if x.is_some() {
    return (Category::Flush, x.unwrap());
  }
  let x = find_straight(cards);
  if x.is_some() {
    return (Category::Straight, x.unwrap());
  }
  let x = find_three_of_a_kind(cards);
  if x.is_some() {
    return (Category::ThreeOfAKind, x.unwrap());
  }
  let x = find_two_pairs(cards);
  if x.is_some() {
    return (Category::TwoPairs, x.unwrap());
  }
  let x = find_one_pairs(cards);
  if x.is_some() {
    return (Category::OnePairs, x.unwrap());
  }
  let x = find_high_card(cards);
  (Category::HighCard, x)
}

// pub fn category<'a>(community_cards: &Vec<&Card>, player_cards: [&Card; 2]) -> Category {
//
// }


pub struct Round<'a> {
  players: Vec<&'a Player>,
  deck: Deck,
  community_cards: Vec<Card>,
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
      community_cards: Vec::new(),
      playings,
    };
    for x in playings {
      x.round = Some(&r)
    }
    r
  }

  fn deal(&mut self) -> Card {
    self.deck.deal()
  }

  pub fn deal_all_players(&mut self) {
    for p in self.players {
      self.playings.push(
        Playing {
          player: p,
          hand: Vec::new(),
          round: Some(self),
        })
    }
    for _i in 0..2 {
      for x in self.playings.iter_mut() {
        x.hand.push(self.deal());
      }
    }
  }


  pub fn deal_flop(&mut self) {
    self.deal();//discard a card
    for _i in 0..3 {
      self.community_cards.push(self.deal());
    }
  }

  pub fn deal_turn(&mut self) {
    self.deal();//discard a card
    self.community_cards.push(self.deal());
  }

  pub fn deal_river(&mut self) {
    self.deal();//discard a card
    self.community_cards.push(self.deal());
  }

  pub fn lookup_community_cards(&self) -> &Vec<Card> {
    &self.community_cards
  }

  pub fn get_winners(&self) -> Vec<&Playing> {
    let h: Option<(Category, [&Card; 5])> = self.playings.iter().map(|&playing| {
      return find_best_composition(&self.community_cards, &playing.hand);
    }).max_by(|&a, &b| {
      let cmp = a.0.cmp(&b.0);
      a.0 == b.0;
    });
  }

  /**
   * @return a list of winners
   */
  pub fn play(&mut self) -> Vec<&Playing> {
    self.deal_all_players();
    self.deal_flop();
    self.deal_turn();
    self.deal_river();
    Round::get_winners(self)
  }
}
