use clap::{arg, ArgAction, Command};

fn main() {
    let matches = Command::new("hgrep")
        .version("1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("HoYa's grep program")
        .arg(arg!(-d --dir "Search <PATTERN> in directory only").action(ArgAction::SetTrue))
        .arg(arg!(-f --file "Search <PATTERN> in file only").action(ArgAction::SetTrue))
        .arg(arg!(-n --name "Search <PATTERN> in file contents").action(ArgAction::SetTrue))
        .arg(arg!(-r --recursive "Search recursively").action(ArgAction::SetTrue))
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
    let pattern = matches
        .get_one::<String>("PATTERN")
        .expect("defaulted by clap");
    let path = matches
        .get_one::<String>("PATH")
        .expect("defaulted by clap");
    println!(
        "{}, {}, {}, {}: {}, {}",
        is_dir, is_file, is_name, is_recursive, pattern, path
    );
}
