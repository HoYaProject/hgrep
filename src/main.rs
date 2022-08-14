use clap::{arg, ArgAction, Command};
use regex::Regex;
use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
struct Searched {
    stype: char,
    line: u16,
    name: String,
}

fn main() {
    let matches = Command::new("hgrep")
        .version("1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("HoYa's grep program")
        .arg(arg!(-d --dir "Search <PATTERN> in directory only").action(ArgAction::SetTrue))
        .arg(arg!(-f --file "Search <PATTERN> in file only").action(ArgAction::SetTrue))
        .arg(arg!(-n --name "Search <PATTERN> in file contents").action(ArgAction::SetTrue))
        .arg(arg!(-r --recursive "Search recursively").action(ArgAction::SetTrue))
        .arg(arg!(-i --ignorecase "Search with ignoring case").action(ArgAction::SetTrue))
        .arg(arg!(-w --wholeword "Search with the whole word").action(ArgAction::SetTrue))
        .arg(arg!(-a --all "Search with all options").action(ArgAction::SetTrue))
        .arg(arg!(<PATTERN> "PATTERN string to search"))
        .arg(arg!([PATH] "Root path to search").default_value("."))
        .arg_required_else_help(true)
        .get_matches();

    let is_dir = *matches.get_one::<bool>("dir").expect("defaulted by clap");
    let is_file = *matches.get_one::<bool>("file").expect("defaulted by clap");
    let is_name = *matches.get_one::<bool>("name").expect("defaulted by clap");
    let is_recursive = *matches
        .get_one::<bool>("recursive")
        .expect("defaulted by clap");
    let is_ignore = *matches
        .get_one::<bool>("ignorecase")
        .expect("defaulted by clap");
    let is_whole = *matches
        .get_one::<bool>("wholeword")
        .expect("defaulted by clap");
    let is_all = *matches.get_one::<bool>("all").expect("defaulted by clap");
    let pattern = matches
        .get_one::<String>("PATTERN")
        .expect("defaulted by clap");
    let root_path = matches
        .get_one::<String>("PATH")
        .expect("defaulted by clap");
    println!(
        "{}, {}, {}, {}, {}, {}, {}: {}, {}",
        is_dir, is_file, is_name, is_recursive, is_ignore, is_whole, is_all, pattern, root_path
    );

    let searched_list = get_list(root_path, is_recursive);
    println!("{:?}", searched_list);

    #[allow(unused_variables)]
    let re = Regex::new(pattern).unwrap();
}

fn get_list(root_path: &String, is_recursive: bool) -> Vec<Searched> {
    let mut searched_list: Vec<Searched> = Vec::new();

    let paths = fs::read_dir(root_path).unwrap();
    for path in paths {
        let cur_path = path.unwrap().path();

        if cur_path.is_dir() {
            let searched = Searched {
                stype: 'D',
                line: 0,
                name: cur_path.display().to_string(),
            };
            searched_list.push(searched);

            if is_recursive == true {
                let mut recursive_list = get_list(&cur_path.display().to_string(), is_recursive);
                searched_list.append(&mut recursive_list);
            }
        } else if cur_path.is_file() {
            let searched = Searched {
                stype: 'F',
                line: 0,
                name: cur_path.display().to_string(),
            };
            searched_list.push(searched)
        }
    }

    return searched_list;
}
