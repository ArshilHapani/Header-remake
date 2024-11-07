mod utils;

use color_print::cprintln;
use std::env::args;

use utils::{print_headers_with_options, print_help_menu};

fn main() {
    let args: Vec<String> = args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help_menu();
        return;
    }

    let symbol_flag = args.iter().position(|arg| arg == "-s" || arg == "--symbol");
    let symbol =
        symbol_flag.map(|index| args.get(index + 1).map(|s| s.as_str()).unwrap_or_default());

    if symbol_flag.is_some() && symbol.is_none() {
        cprintln!("<bold><red> Error: Symbol flag is required </red></bold>");
        return;
    }

    let length_flag = args.iter().position(|arg| arg == "-l" || arg == "--length");
    let length = match length_flag {
        Some(index) => args
            .get(index + 1)
            .and_then(|len| len.parse::<usize>().ok()),
        None => None,
    };

    if length_flag.is_some() && length.is_none() {
        cprintln!("<bold><red> Error: Length flag must be a number</red></bold>");
        return;
    }

    let title = args
        .last()
        .map(|t| t.to_string())
        .unwrap_or("Title".to_string());
    let single_line_flag = args
        .iter()
        .position(|arg| arg == "-sl" || arg == "--single-line");
    let single_line = single_line_flag.is_some();

    print_headers_with_options(symbol.as_deref(), length, Some(&title), single_line);
}
