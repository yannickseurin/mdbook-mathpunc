use fancy_regex::Regex;
use mdbook::book::{Book, BookItem};
use mdbook::errors::{Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct MathpuncPreprocessor;

impl Preprocessor for MathpuncPreprocessor {
    fn name(&self) -> &str {
        "mathpunc"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = *item {
                chapter.content = find_and_replace(chapter.content.clone());
            }
        });

        Ok(book)
    }
}

/// replaces all occurrences of "$p" in `s`, where p is one of the punctuation marks {, . ; :}
/// (possibly with zero or more white spaces between the dollar sign and p)
/// by "p$", except if the dollar sign is escaped with a backslash.
fn find_and_replace(s: String) -> String {
    /*
    this regex expression means "$" followed by zero or more white spaces
    followed by one of ",", ".", ";", ":", except if "$" is escaped with a backslash
    */
    let re = Regex::new(r"(?<!\\)\$\s*(?<punc>[,,.,;,:])").unwrap();
    return re.replace_all(&s, "$punc$$").to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = String::from(r"Consider a group $\GG$, of order $p$; and a generator $G$: for example an elliptic curve $E$.");
        let output = find_and_replace(input);
        let expected = String::from(r"Consider a group $\GG,$ of order $p;$ and a generator $G:$ for example an elliptic curve $E.$");
        assert_eq!(output, expected);
    }

    #[test]
    fn escaped_dollar() {
        let input = String::from(r"This is an escaped dollar \$, buddy. This as well \& .");
        let output = find_and_replace(input.clone());
        assert_eq!(output, input);
    }

    #[test]
    fn whitespaces() {
        let input = String::from(r"Consider a group $\GG$  , of order $p$ ; and a generator $G$   : for example an elliptic curve $E$ .");
        let output = find_and_replace(input);
        let expected = r"Consider a group $\GG,$ of order $p;$ and a generator $G:$ for example an elliptic curve $E.$";
        assert_eq!(output, expected);
    }
}