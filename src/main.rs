use clap::{arg, ArgAction, ArgMatches, Command};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
struct Config {
    ex_dir: Vec<String>,
    ex_ext: Vec<String>,
    in_dir: Vec<String>,
    in_ext: Vec<String>,
}

struct Searched {
    stype: char,
    line: u16,
    name: PathBuf,
}

fn main() {
    let matches = Command::new("hgrep")
        .version("1.1")
        .author("llHoYall <hoya128@gmail.com>")
        .about("HoYa's grep program")
        .arg_required_else_help(true)
        .arg(arg!(-d --dir "Search <PATTERN> in directory only").action(ArgAction::SetTrue))
        .arg(arg!(-f --file "Search <PATTERN> in file only").action(ArgAction::SetTrue))
        .arg(arg!(-n --name "Search <PATTERN> in file contents").action(ArgAction::SetTrue))
        .arg(arg!(-r --recursive "Search recursively").action(ArgAction::SetTrue))
        .arg(arg!(-i --ignorecase "Search with ignoring case").action(ArgAction::SetTrue))
        .arg(arg!(-w --wholeword "Search with the whole word").action(ArgAction::SetTrue))
        .arg(arg!(-a --all "Search with all options").action(ArgAction::SetTrue))
        .arg(arg!(-c --config <OPTION> "Configure exclude/include\nAvailable Options:\n\tex_dir: Exclude directory\n\tex_ext: Exclude extension\n\tin_dir: Include directory\n\tin_ext: Include extension\n\tclear: Clear configuration").required(false))
        .arg(arg!(<PATTERN> "PATTERN string to search"))
        .arg(arg!([PATH] "Root path to search").default_value("."))
        .get_matches();

    let pattern = matches.get_one::<String>("PATTERN").expect("");
    if save_config(&matches, pattern) {
        return;
    }

    let (is_dir, is_file, is_name, is_recursive, is_ignore, is_whole) = get_args(&matches);
    let re = get_re(is_ignore, is_whole, pattern);
    let root_path = PathBuf::from(matches.get_one::<String>("PATH").expect(""));

    let searched_list = get_list(root_path, is_recursive);
    println!("──────┬────────┬──────────────────────────────────────────────────────────────");
    println!(" Type │ Line   │ Location ");
    println!("──────┼────────┼──────────────────────────────────────────────────────────────");
    for searched in searched_list {
        let full_name = &searched.name.to_string_lossy().to_string().replace('"', "");
        let target = searched.name.file_name().unwrap().to_str().unwrap();

        if is_dir && searched.stype == 'D' {
            if re.find(target) != None {
                println!(
                    "  {}   │ {:>6} │ {}",
                    searched.stype, searched.line, full_name
                );
            }
        }
        if is_file && searched.stype == 'F' {
            if re.find(target) != None {
                println!(
                    "  {}   │ {:>6} │ {}",
                    searched.stype, searched.line, full_name
                );
            }
        }
        if is_name && searched.stype == 'F' {
            let file = File::open(&searched.name).unwrap();
            let reader = BufReader::new(file);
            let mut is_first = true;
            for (nline, text) in reader.lines().enumerate() {
                let target;
                match text {
                    Ok(_) => target = text.unwrap(),
                    Err(_) => continue,
                }

                let found = re.find(&target);
                if found != None {
                    if is_first {
                        is_first = false;
                        println!(
                            "  {}   │ {:>6} │ {}",
                            searched.stype, searched.line, full_name
                        );
                    }
                    if target.len() <= 50 {
                        println!("   {}  │ {:>6} │   {}", 'N', nline + 1, target);
                    } else if target.len() - found.unwrap().start() <= 50 {
                        println!(
                            "   {}  │ {:>6} │   {}",
                            'N',
                            nline + 1,
                            target[found.unwrap().start()..].to_string() + &"...".to_string()
                        );
                    } else {
                        println!(
                            "   {}  │ {:>6} │   {}",
                            'N',
                            nline + 1,
                            target[found.unwrap().start()..found.unwrap().start() + 50].to_string()
                                + &"...".to_string()
                        );
                    }
                }
            }
        }
    }
    println!("──────┴────────┴──────────────────────────────────────────────────────────────");
}

fn save_config(args: &ArgMatches, pattern: &String) -> bool {
    let config = args.get_one::<String>("config");
    if config != None {
        let mut ret = false;
        match config {
            Some(opt) => match opt.as_str() {
                "ex_dir" => ret = config_exclude(Some(pattern), None),
                "ex_ext" => ret = config_exclude(None, Some(pattern)),
                "in_dir" => ret = config_include(Some(pattern), None),
                "in_ext" => ret = config_include(None, Some(pattern)),
                "clear" => ret = config_clear(),
                _ => println!("Error: Not supported option"),
            },
            None => println!("Error: Invalid arguments"),
        }
        ret
    } else {
        false
    }
}

fn load_config() -> Config {
    let mut result = Config {
        ex_dir: ["".to_string()].to_vec(),
        ex_ext: ["".to_string()].to_vec(),
        in_dir: ["".to_string()].to_vec(),
        in_ext: ["".to_string()].to_vec(),
    };

    let f = File::open("hgrep_config.json");
    if f.is_err() {
        return result;
    }
    let mut json = String::new();
    f.unwrap().read_to_string(&mut json).unwrap();
    let config: Result<Config, serde_json::Error> = serde_json::from_str(&json);
    if config.is_err() {
        return result;
    }
    let config = config.unwrap();
    result.ex_dir = config.ex_dir;
    result.ex_ext = config.ex_ext;
    result.in_dir = config.in_dir;
    result.in_ext = config.in_ext;
    result
}

fn check_exclude(path: &PathBuf, config: &Config) -> bool {
    let full_name = &path.to_string_lossy().to_string().replace('"', "");

    if path.is_dir() {
        if config
            .ex_dir
            .iter()
            .any(|n| n != "" && full_name.contains(n))
        {
            return true;
        }
    } else if path.is_file() {
        let ext = path.extension();
        if ext.is_some()
            && config
                .ex_ext
                .iter()
                .any(|n| n != "" && ext.unwrap().to_str() == Some(n))
        {
            return true;
        }
    }

    false
}

fn check_include_directory(path: &PathBuf, config: &Config) -> bool {
    let full_name = &path.to_string_lossy().to_string().replace('"', "");

    if config
        .in_dir
        .iter()
        .any(|n| n != "" && full_name.contains(n))
    {
        return true;
    }
    false
}

fn check_include_file(path: &PathBuf, config: &Config) -> bool {
    let ext = path.extension();
    if ext.is_some()
        && config
            .in_ext
            .iter()
            .any(|n| n != "" && ext.unwrap().to_str() == Some(n))
    {
        return true;
    }
    false
}

fn config_clear() -> bool {
    let config = Config {
        ex_dir: ["".to_string()].to_vec(),
        ex_ext: ["".to_string()].to_vec(),
        in_dir: ["".to_string()].to_vec(),
        in_ext: ["".to_string()].to_vec(),
    };
    let config = serde_json::to_writer(
        &File::create("hgrep_config.json").unwrap(),
        &serde_json::to_value(config).unwrap(),
    );
    if config.is_err() {
        false;
    }

    true
}

fn config_exclude(dir: Option<&String>, ext: Option<&String>) -> bool {
    let mut config = load_config();
    if dir != None {
        config.ex_dir = dir
            .unwrap()
            .split(',')
            .map(|text| text.trim().to_string())
            .collect();
    } else if ext != None {
        config.ex_ext = ext
            .unwrap()
            .split(',')
            .map(|text| text.trim().to_string())
            .collect();
    }

    let result = serde_json::to_writer(
        &File::create("hgrep_config.json").unwrap(),
        &serde_json::to_value(config).unwrap(),
    );
    if result.is_err() {
        false;
    }

    true
}

fn config_include(dir: Option<&String>, ext: Option<&String>) -> bool {
    let mut config = load_config();
    if dir != None {
        config.in_dir = dir
            .unwrap()
            .split(',')
            .map(|text| text.trim().to_string())
            .collect();
    } else if ext != None {
        config.in_ext = ext
            .unwrap()
            .split(',')
            .map(|text| text.trim().to_string())
            .collect();
    }

    let result = serde_json::to_writer(
        &File::create("hgrep_config.json").unwrap(),
        &serde_json::to_value(config).unwrap(),
    );
    if result.is_err() {
        return false;
    }

    true
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

fn get_re(is_ignore: bool, is_whole: bool, pattern: &String) -> Regex {
    let re: Regex;
    if is_ignore {
        let fstring;
        if is_whole {
            fstring = format!(
                "(?i)([\\-_./()\\[\\][[:space:]]]+|^){}([\\-_./()\\[\\][[:space:]]]+|$)",
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
                "(?-i)([\\-_./()\\[\\][[:space:]]]+|^){}([\\-_./()\\[\\][[:space:]]]+|$)",
                pattern
            );
        } else {
            fstring = format!("(?-i){}", pattern);
        }
        re = Regex::new(&fstring).unwrap();
    }

    re
}

fn get_list(root_path: PathBuf, is_recursive: bool) -> Vec<Searched> {
    let mut searched_list: Vec<Searched> = Vec::new();
    let config = load_config();

    let paths = fs::read_dir(root_path).unwrap();
    for path in paths {
        let cur_path = path.unwrap().path();
        let copied_path = cur_path.clone();

        if check_exclude(&cur_path, &config) {
            continue;
        }

        if cur_path.is_dir() {
            if check_include_directory(&cur_path, &config) {
                let searched = Searched {
                    stype: 'D',
                    line: 0,
                    name: cur_path,
                };
                searched_list.push(searched);
            }

            if is_recursive == true {
                let mut recursive_list = get_list(copied_path, is_recursive);
                searched_list.append(&mut recursive_list);
            }
        } else if cur_path.is_file() {
            if check_include_file(&cur_path, &config) {
                let searched = Searched {
                    stype: 'F',
                    line: 0,
                    name: cur_path,
                };
                searched_list.push(searched);
            }
        }
    }

    return searched_list;
}
