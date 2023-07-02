use std::{env, thread};
use Vec;

use crate::List::{Cons, None};

// fn main() -> Result<(), String> {
// fn main() {
// let args: Vec<String> = env::args().collect();
// let search = parse_args(&args);
// let result = grep(search, file);
// print_search_result(&result);
// }

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
  Red,
  Blue,
}

struct Inventory {
  shirts: Vec<ShirtColor>,
}

struct MyList<T> {
  list: Vec<T>,
}

impl Iterator for MyList<T> {
  type Item = (T);

  fn next(&mut self) -> Option<Self::Item> {
    todo!()
  }
}


impl Inventory {
  fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }
    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}

fn main1() {
  // let store = Inventory {
  //   shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  // };
  //
  // let user_pref1 = Some(ShirtColor::Red);
  // let giveaway1 = store.giveaway(user_pref1);
  // println!(
  //   "The user with preference {:?} gets {:?}",
  //   user_pref1, giveaway1
  // );
  //
  // let user_pref2 = None;
  // let giveaway2 = store.giveaway(user_pref2);
  // println!(
  //   "The user with preference {:?} gets {:?}",
  //   user_pref2, giveaway2
  // );

  // let mut list = vec![1, 2, 3];
  // println!("Before defining closure: {:?}", list);
  //
  // let mut borrows_mutably = || list.push(7);
  //
  // println!("Before calling closure: {:?}", list);
  // borrows_mutably();
  // println!("After calling closure: {:?}", list);
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();

  println!("After: {:?}", list);
}

enum List {
  Cons(i32, Box<List>),
  None,
}

fn main() {
  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(None))))));
}