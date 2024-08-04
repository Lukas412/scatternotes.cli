use termfmt::TermStyle;

use crate::note::Tag;

pub fn pretty_print_with_tags(mut content: &str) {
    while !content.is_empty() {
        let Some(index) = content.find('#') else {
            print!("{}", content);
            break;
        };
        print!("{}", &content[..index]);
        content = &content[index..];
        if let Some((remaining, tag)) = Tag::parse_single(content) {
            content = remaining;
            print!("{}", format_args!("#{}", tag.text()).fg_blue());
        };
    }
    println!();
}
