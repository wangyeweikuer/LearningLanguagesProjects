// Read me about poker first: https://cardguide.fandom.com/wiki/Pip_cards

// mod domain;

use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use rand::seq::SliceRandom;
// use random::Source;
// use shuffle::shuffler::Shuffler;
// use shuffle::irs::Irs;
// use rand::rngs::mock::StepRng;
use rand::thread_rng;

use crate::Rank::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};
use crate::Suit::{Clubs, Diamonds, Hearts, Spades};

mod domain;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Suit {
  Spades,
  Hearts,
  Clubs,
  Diamonds,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
  pub fn int(&self, ace_as_big: bool) -> usize {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
  fn rank(&self, ace_as_big: bool) -> usize {
    self.rank.int(ace_as_big)
  }

  fn compare(&self, other: &Card, ace_as_big: bool) -> Ordering {
    self.rank(ace_as_big).cmp(&other.rank(ace_as_big))
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

  //https://stackoverflow.com/questions/26033976/how-do-i-create-a-vec-from-a-range-and-shuffle-it
  pub fn shuffle(&mut self) {
    // let mut rng = StepRng::new(2, 13);
    // let mut irs = Irs::default();
    // irs.shuffle(&mut self.cards, &mut rng);
    self.cards.shuffle(&mut thread_rng())
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

#[derive(Copy, Clone)]
pub struct Player {
  name: &'static str,
}

#[derive(Clone)]
pub struct Playing {
  hand: Vec<Card>,
  player: Player,
  // round: Option<&'a Round>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
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

#[derive(Copy, Clone, Debug)]
struct BestHand {
  category: Category,
  reversed_cards: [Card; 5],
}

impl BestHand {
  fn compare_ranks(a: &[Card], b: &[Card]) -> Ordering {
    for i in 0..a.len() {
      let cmp = a[i].compare(&b[i], true);
      if cmp != Equal {
        return cmp;
      }
    }
    Equal
  }

  fn compare(&self, other: &Self) -> Ordering {
    let cmp = self.category.cmp(&other.category);
    if cmp != Equal {
      return cmp;
    }
    use Category::*;
    match self.category {
      StraightFlush | FourOfAKind | FullHouse | Straight | ThreeOfAKind => {
        self.reversed_cards[0].compare(&other.reversed_cards[0], true)
      }
      Flush => {
        for i in 0..5 {
          let cmp = self.reversed_cards[i].compare(&other.reversed_cards[i], true);
          if cmp != Equal {
            return cmp;
          }
        }
        return Equal;
      }
      TwoPairs => {
        let cmp = self.reversed_cards[0].compare(&other.reversed_cards[0], true);
        match cmp {
          // from 3.. -> [3,4]
          Equal => BestHand::compare_ranks(&self.reversed_cards[3..], &other.reversed_cards[3..]),
          _ => cmp
        }
      }
      OnePair => {
        let cmp = self.reversed_cards[0].compare(&other.reversed_cards[0], true);
        match cmp {
          Equal => BestHand::compare_ranks(&self.reversed_cards[2..], &other.reversed_cards[2..]),
          _ => cmp
        }
      }
      HighCard => BestHand::compare_ranks(&self.reversed_cards, &other.reversed_cards)
    }
  }
}

fn partition_by<F, K: Eq + PartialEq + Hash>(cards: &Vec<Card>, pred: F) -> Vec<Vec<Card>>
where F: Fn(Card) -> K {
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

fn sort_reversely(cards: &mut Vec<Card>, ace_as_big: bool) {
  cards.sort_by(|a, b| b.rank(ace_as_big).cmp(&a.rank(ace_as_big)));
}

fn find_straight_inner(cards: &mut Vec<Card>, ace_as_big: bool) -> Option<[usize; 5]> {
  sort_reversely(cards, ace_as_big);
  let mut found = [0; 5];
  for i in 0..cards.len() - 4 {
    let now = cards[i].rank(ace_as_big);
    found[0] = i;
    for j in 1..5 {
      if cards[j + i].rank(ace_as_big) + j != now {
        break;
      }
      found[j] = j + i;
    }
    return Some(found);
  }
  None
}

fn find_same_ranks_idx<F>(cards: &mut Vec<Card>, reversely_sort: bool, same_num: usize, mut filter: F)
                          -> Option<Vec<usize>>
where F: FnMut(usize) -> bool {
  if reversely_sort {
    sort_reversely(cards, true);
  }

  let mut res = Vec::with_capacity(same_num);
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

fn find_any<F>(cards: &Vec<Card>, same_num: usize, mut filter: F) -> Vec<Card>
where F: FnMut(usize) -> bool {
  let mut res = Vec::with_capacity(same_num);
  for i in 0..cards.len() {
    if filter(i) {
      res.push(cards[i]);
    }
  }
  res
}

fn find_straight_flush(mut cards: &Vec<Card>) -> Option<BestHand> {
  let vv = partition_by(cards, |card| card.suit);
  let r = vv.iter().filter(|&x| x.len() >= 5).next();
  if r.is_none() {
    return None;
  }
  let mut r = r.unwrap().clone();
  let found = find_straight_inner(&mut r, true);
  use Category::StraightFlush;
  let z = match found {
    Some(x) => x,
    None => {
      let x = find_straight_inner(&mut r, false);
      match x {
        None => return None,
        Some(y) => y
      }
    }
  };
  Some(BestHand {
    category: StraightFlush,
    reversed_cards: [cards[z[0]], cards[z[1]], cards[z[2]], cards[z[3]], cards[z[4]]],
  })
}

fn find_four_of_a_kind(cards: &mut Vec<Card>) -> Option<BestHand> {
  let found = find_same_ranks_idx(cards, true, 4, |_idx| true);
  return match found {
    None => None,
    Some(x) => {
      let other = find_any(cards, 1, |idx| !x.contains(&idx));
      Some(BestHand {
        category: Category::FourOfAKind,
        reversed_cards: [cards[x[0]], cards[x[1]], cards[x[2]], cards[x[3]], other[0]],
      })
    }
  };
}

fn find_full_house(cards: &mut Vec<Card>) -> Option<BestHand> {
  let found = find_same_ranks_idx(cards, true, 3, |_idx| true);
  match found {
    None => None,
    Some(x) => {
      let other = find_same_ranks_idx(cards, false, 2, |idx| !x.contains(&idx),
      ).unwrap();
      Some(BestHand {
        category: Category::FullHouse,
        reversed_cards: [cards[x[0]], cards[x[1]], cards[x[2]], cards[other[0]], cards[other[1]]],
      })
    }
  }
}

fn find_flush(cards: &mut Vec<Card>) -> Option<BestHand> {
  let parts = partition_by(cards, |card| card.suit);
  let r = parts.iter().filter(|&x| x.len() >= 5).next();
  match r {
    None => None,
    Some(x) => Some(BestHand {
      category: Category::Flush,
      reversed_cards: [x[0], x[1], x[2], x[3], x[4]],
    })
  }
}

fn find_straight(cards: &mut Vec<Card>) -> Option<BestHand> {
  let x = find_straight_inner(cards, true);
  use Category::Straight;
  let z = match x {
    Some(z) => z,
    None => {
      let y = find_straight_inner(cards, false);
      match y {
        Some(y) => y,
        None => return None
      }
    }
  };
  Some(BestHand {
    category: Straight,
    reversed_cards: [cards[z[0]], cards[z[1]], cards[z[2]], cards[z[3]], cards[z[4]]],
  })
}

fn find_three_of_a_kind(cards: &mut Vec<Card>) -> Option<BestHand> {
  let found = find_same_ranks_idx(cards, true, 3, |_idx| true);
  match found {
    None => None,
    Some(x) => {
      let others = find_any(cards, 2, |idx| !x.contains(&idx));
      Some(BestHand {
        category: Category::ThreeOfAKind,
        reversed_cards: [cards[x[0]], cards[x[1]], cards[x[2]], others[0], others[1]],
      })
    }
  }
}

fn find_two_pairs(cards: &mut Vec<Card>) -> Option<BestHand> {
  let found = find_same_ranks_idx(cards, true, 2, |_idx| true);
  match found {
    None => None,
    Some(x) => {
      let x1 = find_same_ranks_idx(cards, false, 2, |idx| !x.contains(&idx));
      match x1 {
        None => None,
        Some(x2) => {
          let other = find_any(cards, 1, |idx| !x.contains(&idx) && !x2.contains(&idx));
          Some(BestHand {
            category: Category::TwoPairs,
            reversed_cards: [cards[x[0]], cards[x[1]], cards[x2[0]], cards[x2[1]], other[0]],
          })
        }
      }
    }
  }
}

fn find_one_pairs(cards: &mut Vec<Card>) -> Option<BestHand> {
  let found = find_same_ranks_idx(cards, true, 2, |_idx| true);
  match found {
    None => None,
    Some(x) => {
      let others = find_any(cards, 3, |idx| !x.contains(&idx));
      Some(BestHand {
        category: Category::OnePair,
        reversed_cards: [cards[x[0]], cards[x[1]], others[0], others[1], others[2]],
      })
    }
  }
}

fn find_high_card(cards: &mut Vec<Card>) -> BestHand {
  sort_reversely(cards, true);
  BestHand {
    category: Category::HighCard,
    reversed_cards: [cards[0], cards[1], cards[2], cards[3], cards[4]],
  }
}

fn find_best_composition<'a>(community_cards: &'a Vec<Card>, player_cards: &'a Vec<Card>) -> BestHand {
  let mut cards: Vec<Card> = Vec::new();
  community_cards.iter().for_each(|c| cards.push(*c));
  player_cards.iter().for_each(|c| cards.push(*c));
  let x = find_straight_flush(&cards);
  if x.is_some() {
    return x.unwrap();
  }
  let x = find_four_of_a_kind(&mut cards);
  if x.is_some() {
    return x.unwrap();
  }
  let x = find_full_house(&mut cards);
  if x.is_some() {
    return x.unwrap();
  }
  let x = find_flush(&mut cards);
  if x.is_some() {
    return x.unwrap();
  }
  let x = find_straight(&mut cards);
  if x.is_some() {
    return x.unwrap();
  }
  let x = find_three_of_a_kind(&mut cards);
  if x.is_some() {
    return x.unwrap();
  }
  let x = find_two_pairs(&mut cards);
  if x.is_some() {
    return x.unwrap();
  }
  let x = find_one_pairs(&mut cards);
  if x.is_some() {
    return x.unwrap();
  }
  find_high_card(&mut cards)
}

// pub fn category<'a>(community_cards: &Vec<&Card>, player_cards: [&Card; 2]) -> Category {
//
// }

pub struct Round {
  players: Vec<Player>,
  deck: Deck,
  community_cards: Vec<Card>,
  playings: Vec<Playing>,
}

struct BestPlaying {
  playing: Playing,
  best_hand: BestHand,
}

impl BestPlaying {}

impl Round {
  pub fn new(players: Vec<Player>) -> Round {
    let mut playings = Vec::with_capacity(players.len());
    for i in 0..players.len() {
      playings[i] = Playing {
        hand: vec![],
        player: players[i],
        // round: None,
      };
    }
    let mut r = Round {
      players,
      deck: Deck::new(),
      community_cards: Vec::new(),
      playings,
    };
    r
  }

  fn deal(&mut self) -> Card {
    self.deck.deal()
  }

  pub fn deal_all_players(&mut self) {
    // for p in self.players {
    //   self.playings.push(
    //     Playing {
    //       player: p,
    //       hand: [Card; 2],
    //       round: Some(self),
    //     })
    // }
    for _i in 0..2 {
      let h = self.deal();
      self.playings.iter_mut().for_each(|p| { p.hand.push(h) });
    }
  }

  pub fn deal_flop(&mut self) {
    self.deal();//discard a card
    for _i in 0..3 {
      let c = self.deal();
      self.community_cards.push(c);
    }
  }

  pub fn deal_turn(&mut self) {
    self.deal();//discard a card
    let c = self.deal();
    self.community_cards.push(c);
  }

  pub fn deal_river(&mut self) {
    self.deal();//discard a card
    let c = self.deal();
    self.community_cards.push(c);
  }

  pub fn lookup_community_cards(&self) -> &Vec<Card> {
    &self.community_cards
  }

  pub fn get_winners(&self) -> Vec<Playing> {
    let playings: Vec<BestPlaying> = self.playings.into_iter().map(|playing| {
      let x = find_best_composition(&self.community_cards, &playing.hand);
      BestPlaying {
        playing,
        best_hand: x,
      }
    }).collect();
    let max = playings.iter().max_by(|&a, &b| {
      a.best_hand.compare(&b.best_hand)
    }).unwrap();

    playings
      .iter()
      .filter_map(|a| {
        match a.best_hand.compare(&max.best_hand) {
          Equal => { Some(a.playing.clone()) }
          _ => None,
        }
      })
      .collect()
  }

  /**
   * @return a list of winners
   */
  pub fn play(&mut self) -> Vec<Playing> {
    self.deal_all_players();
    self.deal_flop();
    self.deal_turn();
    self.deal_river();
    Round::get_winners(self)
  }
}
