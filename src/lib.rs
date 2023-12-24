//! An mdbook preprocessor that prevents line breaks between inline math blocks and punctuation marks when using katex.

use fancy_regex::{Captures, Regex};
use lazy_static::lazy_static;
use mdbook::book::{Book, BookItem};
use mdbook::errors::Result;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

/// The preprocessor name.
const NAME: &str = "mathpunc";

/// The preprocessor.
pub struct MathpuncPreprocessor;

lazy_static! {
    /// The regex used for replacement.
    static ref RE: Regex =
        // see https://regex101.com/ for an explanation of the regex
        Regex::new(r"(?<!\\)\$\s*(?<punc>\)?[,,.,;,:,)])").unwrap();
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

/// Replaces all occurrences of "$p" in `s`, where p is zero or one closing parenthesis
/// followed by one of the five punctuation marks {, . ; : )}
/// (possibly with zero or more white spaces between the dollar sign and p)
/// by "p$", except if the dollar sign is escaped with a backslash.
fn find_and_replace(s: &str) -> String {
    // RE.replace_all(s, "$punc$$").to_string()
    RE.replace_all(s, |caps: &Captures| {
        match &caps["punc"] {
            ":" => {
                r"\!\!:$".to_string()
            }
            "):" => {
                r")\!\!:$".to_string()
            }
            _ => {
                format!("{}$", &caps["punc"])
            }
        }
    }).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let input = String::from(
            r"Consider a group $\GG$, of order $p$; and a generator $G$: for example an elliptic curve $E$.",
        );
        let output = find_and_replace(&input);
        let expected = String::from(
            r"Consider a group $\GG,$ of order $p;$ and a generator $G\!\!:$ for example an elliptic curve $E.$",
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn escaped_dollar() {
        let input = String::from(r"This is an escaped dollar \$, don't replace. This as well \& .");
        let output = find_and_replace(&input);
        assert_eq!(output, input);
    }

    #[test]
    fn whitespaces() {
        let input = String::from(
            r"Consider a group $\GG$  , of order $p$ ; and a generator $G$   : for example an elliptic curve $E$ .",
        );
        let output = find_and_replace(&input);
        let expected = String::from(
            r"Consider a group $\GG,$ of order $p;$ and a generator $G\!\!:$ for example an elliptic curve $E.$",
        );
        assert_eq!(output, expected);
    }

    #[test]
    fn parenthesis() {
        let input =
            String::from(r"Consider a group $\GG$ (of order $p$), and a generator $G$ (of $\GG$).");
        let output = find_and_replace(&input);
        let expected =
            String::from(r"Consider a group $\GG$ (of order $p),$ and a generator $G$ (of $\GG).$");
        assert_eq!(output, expected);
    }

    #[test]
    fn parenthesis_and_colon() {
        let input =
            String::from(r"Consider a group $\GG$ (of order $p$):");
        let output = find_and_replace(&input);
        let expected =
            String::from(r"Consider a group $\GG$ (of order $p)\!\!:$");
        assert_eq!(output, expected);
    }
}
