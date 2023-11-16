use fancy_regex::Regex;
use lazy_static::lazy_static;
use mdbook::book::{Book, BookItem};
use mdbook::errors::{Result};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

/// the preprocessor name
const NAME: &str = "mathpunc";

pub struct MathpuncPreprocessor;

lazy_static! {
    // see https://regex101.com/ for an explanation of the regex
    static ref RE: Regex =
        Regex::new(r"(?<!\\)\$\s*(?P<punc>\)?[,,.,;,:,)])").unwrap();
}

impl MathpuncPreprocessor {
    pub fn new() -> Self {
        Self
    }
}

impl Preprocessor for MathpuncPreprocessor {
    fn name(&self) -> &str {
        NAME
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = find_and_replace(&chapter.content);
            }
        });

        Ok(book)
    }
}

/// replaces all occurrences of "$p" in `s`, where p is zero or one closing parenthesis
/// followed by one of the five punctuation marks {, . ; : )}
/// (possibly with zero or more white spaces between the dollar sign and p)
/// by "p$", except if the dollar sign is escaped with a backslash.
fn find_and_replace(s: &str) -> String {
    return RE.replace_all(s, "$punc$$").to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = String::from(r"Consider a group $\GG$, of order $p$; and a generator $G$: for example an elliptic curve $E$.");
        let output = find_and_replace(&input);
        let expected = String::from(r"Consider a group $\GG,$ of order $p;$ and a generator $G:$ for example an elliptic curve $E.$");
        assert_eq!(output, expected);
    }

    #[test]
    fn escaped_dollar() {
        let input = String::from(r"This is an escaped dollar \$, buddy. This as well \& .");
        let output = find_and_replace(&input);
        assert_eq!(output, input);
    }

    #[test]
    fn whitespaces() {
        let input = String::from(r"Consider a group $\GG$  , of order $p$ ; and a generator $G$   : for example an elliptic curve $E$ .");
        let output = find_and_replace(&input);
        let expected = String::from(r"Consider a group $\GG,$ of order $p;$ and a generator $G:$ for example an elliptic curve $E.$");
        assert_eq!(output, expected);
    }

    #[test]
    fn parenthesis() {
        let input = String::from(r"Consider a group $\GG$ (of order $p$), and a generator $G$ (of $\GG$).");
        let output = find_and_replace(&input);
        let expected = String::from(r"Consider a group $\GG$ (of order $p),$ and a generator $G$ (of $\GG).$");
        assert_eq!(output, expected);
    }
}