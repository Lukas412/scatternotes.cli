use termfmt::TermStyle;

use crate::tag::Tag;

pub fn pretty_print_with_tags(mut content: &str) {
    while !content.is_empty() {
        match Tag::parse_next(content) {
            Ok((preceding, remaining, tag)) => {
                content = remaining;
                print!("{}{}", preceding, format_args!("{}", tag).fg_blue());
            }
            Err(remaining) => {
                print!("{}", remaining);
                break;
            }
        }
    }
    println!();
}
