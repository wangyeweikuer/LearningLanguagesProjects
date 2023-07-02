use std::fs::File;
use std::io::{self, Read};

// #[derive(Debug, Clone, Ord)]
// struct Point {
//     x: i32,
//     y: i32,
// }
//
// impl<T> Point where T: Ord {
//     fn new(x: i32, y: i32) -> Point {
//         Point { x, y }
//     }
//
//     fn product_with(&self, other: &Point) -> i32 {
//         self.x * other.x + self.y * other.y
//     }
//
//     fn compare<T: Ord>(a: &T, b: &T) -> bool {
//         if a > b{
//             true
//         }else {
//             false
//         }
//     }
// }
//
//
// fn binary_search<E>(array: &[E],  key:E) -> Option<usize> {
//     if array.len()==0 {
//         return None;
//     }
//     let  low = 0;
//     let hig = array.len()-1;
//     let  mid = (low +hig)>>1;
//     return match key.cmp(array[mid]) {
//         Ordering::Less => binary_search(&array[low+1..], key)
//         Ordering::Equal => Some(mid)
//         Ordering::Greater =>binary_search(&array[..hig-1],key)
//     }
// }

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let x = haystack.find(needle.as_str());
        return match x {
            None => -1,
            Some(x) => x as i32,
        };
    }

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Err(e) => e as i32,
            Ok(e) => e as i32,
        }
    }

    pub fn length_of_last_word(s: String) -> i32 {
        let mut sum = 0;
        let mut max = 0;
        for x in s.as_bytes() {
            match *x {
                b' ' => {
                    if sum > max {
                        max = sum;
                    }
                    sum = 0;
                }
                _ => {
                    sum += 1;
                }
            }
        }
        max
    }

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
}

fn main() {
    // println!("Welcome to play a guess game");
    //
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("bad input");
    //
    // let x = 10;
    // println!("Hello, world! {}", input);

    //    let x = Solution::str_str("leetcode".to_string(), "tc".to_string());
    //    println!("{}", x);
    //
    //    let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    //    let num = 42;
    //    println!("{}", Solution::search_insert(s, num));
    let x = read_username_from_file();
    print!("username={:?}\n", x);
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}
