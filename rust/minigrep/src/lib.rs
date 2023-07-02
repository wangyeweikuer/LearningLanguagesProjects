use std::fmt::{Display, Formatter};
use std::fs::{self, OpenOptions};

pub struct Search<'a> {
    word: &'a str,
    file: &'a str,
}

impl<'a> Display for Search<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "search[word={}, file={}]", self.word, self.file)
    }
}

#[derive(Debug)]
pub struct Matched<'a> {
    line: usize,
    column: usize,
    content: &'a str,
}

impl<'a> Display for Matched<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(line: {}, column: {}, content: '{}')", self.line, self.column, self.content)
    }
}

pub fn usage() -> &'static str {
    "[Usage] Bad inputs. Please input two parameters: <search_word> <file_path>"
}

pub fn parse_args(args: &Vec<String>) -> Result<(Search, String), &str> {
    dbg!(args);

    if args.len() < 2 {
        return Err(usage());
        // panic!("{}", usage());
    }
    let word = &args[0];
    let file = &args[1];
    let f = OpenOptions::new().read(true).open(file);
    if f.is_err() {
        panic!("[Error] File {} does not exist, or it can't be read.", file)
    }
    let content = fs::read_to_string(file).unwrap();
    return Ok((Search { word, file }, content));
}

pub fn do_grep<'a>(search: &Search, content: &'a str) -> Vec<Matched<'a>> {
    let mut result: Vec<Matched> = vec![];
    let mut ln = 1;
    let lines = content.lines();
    for x in lines {
        let h: Vec<_> = x.match_indices(search.word).collect();
        for y in h {
            result.push(Matched { line: ln, column: y.0, content: y.1 });
        }
        ln += 1;
    }
    result
}

pub fn print_search<'a>(search: &Search, r_list: &Vec<Matched<'a>>) {
    if r_list.is_empty() {
        println!("Sorry, no matches found for {}", search);
        return;
    }
    println!("Found {} matches for {}", r_list.len(), search);
    for x in r_list {
        println!("{}", x);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn one_result() {
//         let word = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";
//
//         let query1 = Search { word, file: "xxx" };
//         assert_eq!(vec![Matched{line:1,column:15,content:"duct"}], do_grep(&query1, contents));
//     }
// }

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    v1.into_iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
