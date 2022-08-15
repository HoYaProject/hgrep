use clap::{arg, ArgAction, ArgMatches, Command};
use regex::Regex;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[allow(dead_code)]
struct Searched {
    stype: char,
    line: u16,
    name: PathBuf,
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

    let (is_dir, is_file, is_name, is_recursive, is_ignore, is_whole) = get_args(&matches);
    let pattern = matches.get_one::<String>("PATTERN").expect("");
    let root_path = PathBuf::from(matches.get_one::<String>("PATH").expect(""));

    let re: Regex;
    if is_ignore {
        let fstring;
        if is_whole {
            fstring = format!(
                "(?i)[\\-_./[[:space:]]]+{}[\\-_./[[:space:]]]+",
                pattern.to_lowercase()
            );
        } else {
            fstring = format!("(?i){}", pattern.to_lowercase());
        }
        re = Regex::new(&fstring).unwrap();
    } else {
        let fstring;
        if is_whole {
            fstring = format!(
                "(?-i)[\\-_./[[:space:]]]+{}[\\-_./[[:space:]]]+",
                pattern.to_lowercase()
            );
        } else {
            fstring = format!("(?-i){}", pattern.to_lowercase());
        }
        re = Regex::new(&fstring).unwrap();
    }

    let searched_list = get_list(root_path, is_recursive);
    println!("──────┬────────┬──────────────────────────────────────────────────────────────");
    println!(" Type │ Line   │ Location ");
    println!("──────┼────────┼──────────────────────────────────────────────────────────────");
    for searched in searched_list {
        let target = searched.name.file_name().unwrap();
        let mut is_print = false;

        if is_dir && searched.stype == 'D' {
            if re.find(target.to_str().unwrap()) != None {
                is_print = true;
            }
        } else if is_file && searched.stype == 'F' {
            if re.find(target.to_str().unwrap()) != None {
                is_print = true;
            }
        } else if is_name && searched.stype == 'F' {
            let file = File::open(&searched.name).unwrap();
            let reader = BufReader::new(file);
            let mut is_first = true;
            for (nline, text) in reader.lines().enumerate() {
                let converted_text;
                match text {
                    Ok(_) => converted_text = text.unwrap(),
                    Err(_) => continue,
                }
                if re.find(&converted_text) != None {
                    if is_first {
                        is_first = false;
                        println!(
                            "  {}   │ {:>6} │ {}",
                            searched.stype,
                            searched.line,
                            target.to_str().unwrap()
                        );
                    }
                    println!("   {}  │ {:>6} │ > {}", 'N', nline + 1, converted_text);
                }
            }
        }

        if is_print {
            println!(
                "  {}   │ {:>6} │ {}",
                searched.stype,
                searched.line,
                target.to_str().unwrap()
            );
        }
    }
    println!("──────┴────────┴──────────────────────────────────────────────────────────────");

    // #[allow(unused_variables)]
}

fn get_args(args: &ArgMatches) -> (bool, bool, bool, bool, bool, bool) {
    let mut is_dir = args.get_one::<bool>("dir").expect("");
    let mut is_file = args.get_one::<bool>("file").expect("");
    let mut is_name = args.get_one::<bool>("name").expect("");
    let mut is_recursive = args.get_one::<bool>("recursive").expect("");
    let mut is_ignore = args.get_one::<bool>("ignorecase").expect("");
    let mut is_whole = args.get_one::<bool>("wholeword").expect("");
    let is_all = args.get_one::<bool>("all").expect("");

    if *is_all || (is_dir | is_file | is_name | is_recursive | is_ignore | is_whole == false) {
        is_dir = &true;
        is_file = &true;
        is_name = &true;
        is_recursive = &true;
        is_ignore = &true;
        is_whole = &true;
    }

    (
        *is_dir,
        *is_file,
        *is_name,
        *is_recursive,
        *is_ignore,
        *is_whole,
    )
}

fn get_list(root_path: PathBuf, is_recursive: bool) -> Vec<Searched> {
    let mut searched_list: Vec<Searched> = Vec::new();

    let paths = fs::read_dir(root_path).unwrap();
    for path in paths {
        let cur_path = path.unwrap().path();

        if cur_path.is_dir() {
            let copied_path = cur_path.clone();
            let searched = Searched {
                stype: 'D',
                line: 0,
                name: cur_path,
            };
            searched_list.push(searched);

            if is_recursive == true {
                let mut recursive_list = get_list(copied_path, is_recursive);
                searched_list.append(&mut recursive_list);
            }
        } else if cur_path.is_file() {
            let searched = Searched {
                stype: 'F',
                line: 0,
                name: cur_path,
            };
            searched_list.push(searched)
        }
    }

    return searched_list;
}
