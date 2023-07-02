// use std::fmt::{Display, Error, Formatter};
// use std::fs;
// use std::fs::OpenOptions;
// use std::io;
//
// struct Search<'a> {
//     file_path: &'a str,
//     search: &'a str,
// }
//
// fn usage() -> String {
//     String::from("Please input two parameters: 1) search word; 2) file path to be searched;")
// }
//
// fn parse_args(args: &Vec<String>) -> io::Result<Search, Error> {
//     // dbg!(args);
//     if args.len() < 3 {
//         return Err(usage());
//     }
//     let search = &args[1];
//     let path = &args[2];
//     // let content = fs::read_to_string(file);
//     // println!("{}", content.unwrap());
//     let file = OpenOptions::new().read(true).open(path);
//     return match file {
//         Ok(f) => Ok(Search { file_path: path, search }),
//         Err(_) => file.err()
//     };
// }
//
// struct Matched {
//     line: usize,
//     column: usize,
//     matched: String,
// }
//
// impl Matched {
//     fn new(line: usize, column: usize, matched: &str) -> Matched {
//         Matched { line, column, matched: String::from(matched) }
//     }
// }
//
// impl<'a> Display for Matched {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "(line:{}, column:{}, matched:{})", self.line, self.column, self.matched)
//     }
// }
//
// fn grep<'a>(search: &Search) -> Vec<Matched> {
//     let text = fs::read_to_string(search.file_path)?;
//     let mut idx: usize = 0;
//     let mut r: Vec<Matched> = vec![];
//     for x in text.lines() {
//         let v = x.match_indices(search);
//         v.for_each(|mi| {
//             r.push(Matched::new(idx, mi.0, mi.1));
//         });
//         idx += 1;
//     }
//     return r;
//     // return match r.is_empty() {
//     //     true => None,
//     //     false => Some(r)
//     // };
// }
//
// fn print_search_result(search: &Search, matched: &Vec<Matched>) {
//     if matched.len() > 0 {
//         println!("Success! We found {} matched results:", matched.len());
//         for x in matched {
//             println!("{}", x);
//         }
//     } else {
//         println!("Sorry, we can't find any matched string '{}' in the file: {}", search.search, search.file_path);
//     }
// }

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut v: Vec<&'a str> = vec![];
  let binding = query.to_lowercase();
  let lower_q = binding.as_str();
  for x in contents.lines() {
    if x.to_lowercase().contains(lower_q) {
      v.push(x);
    }
  }
  v
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut v: Vec<&'a str> = vec![];
  for x in contents.lines() {
    if x.contains(query) {
      v.push(x);
    }
  }
  v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}