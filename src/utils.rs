use arboard::Clipboard;

use color_print::cprintln;

/// Prints the help menu for the CLI tool
/// It uses the `color_print` crate to print colored text
pub fn print_help_menu() {
    cprintln!("<bold><green>Headers Remake </></>");
    println!(
        "A CLI tool to generate well-formatted comment headers for various programming languages."
    );
    println!();
    cprintln!("<bold><cyan>Usage:</> headers [OPTIONS] [TITLE]</bold>");
    cprintln!("<bold><cyan>Notice:</cyan> enclose symbols and header title which contains space with double (\") or single (') quote</bold>");
    println!();
    cprintln!("<bold><cyan>Options:</></>");
    cprintln!("<yellow><bold>-s,  --symbol</bold></yellow>        Specific symbol to use as comment ('#' for python, '/' for javascript, etc.), <bold>default is '/'</bold> ");
    cprintln!("<yellow><bold>-l,  --length</bold></yellow>        Length of the comment header <bold>default is 50</bold>");
    cprintln!(
        "<yellow><bold>-sl, --single-line</bold></yellow>   Generates single line comment header <bold>default is false</bold>"
    );
    cprintln!("<yellow><bold>-h,  --help</bold></yellow>          Prints help information");
}

/// Prints a header with the specified options
/// It uses the `arboard` crate to copy the header to the clipboard
///
/// @param symbol: The symbol to use for the header **(default is '/')**
///
/// @param length: The length of the header **(default is 50)**
///
/// @param title: The title of the header **(default is 'Title')**
///
/// @param single_line: Whether to generate a single line header **(default is `false`)**
///
/// @return: None
///
/// # Example
/// ```
/// print_headers_with_options(Some("#"), Some(50), Some("Title".to_string()), false);
/// ```
///
pub fn print_headers_with_options(
    symbol: Option<&str>,
    length: Option<usize>,
    title: Option<&String>,
    single_line: bool,
) {
    let length = length.unwrap_or(50);
    let length = if length % 2 == 0 { length } else { length + 1 };
    let symbol = symbol.unwrap_or("/");
    let title = title.map(|t| t.as_str()).unwrap_or("Title").to_uppercase();
    let title_len = title.len();

    if length <= title_len {
        cprintln!(
            "<bold><red>Error: The specified length is too small to fit the title <u>'header length + 2'</u>.</red></bold>"
        );
        return;
    }

    let padding_length = (length - (title_len + 2)) / 2;
    let padding = symbol.repeat(padding_length);
    let title_with_symbol = format!("{} {} {}", padding, title, padding);

    let header = if single_line {
        title_with_symbol
    } else {
        format!(
            "{}\n{}\n{}",
            symbol.repeat(length),
            title_with_symbol,
            symbol.repeat(length)
        )
    };

    println!("{}", header);
    let mut clipboard = match Clipboard::new() {
        Ok(clipboard) => clipboard,
        Err(_) => {
            cprintln!("<bold><red>Error: Failed to access clipboard</red></bold>");
            return;
        }
    };
    match clipboard.set_text(header) {
        Ok(_) => {}
        Err(_) => {
            cprintln!("<bold><red>Error: Failed to copy header to clipboard</red></bold>");
            return;
        }
    }
    println!();
    cprintln!("<green><bold>Header copied to clipboard</bold></green>")
}
