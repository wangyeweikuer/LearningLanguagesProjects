use std::env;
use minigrep::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let args = args.split_off(1);
    let args = parse_args(&args);
    if args.is_err() {
        println!("{}", args.err().unwrap());
        return;
    }
    let (search, content) = args.unwrap();
    let r_list = do_grep(&search, &content);
    print_search(&search, &r_list);
}
