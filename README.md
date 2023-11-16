# mdbook-mathpunc

An [mdBook](https://github.com/rust-lang/mdBook) preprocessor preventing line breaks between inline math blocks and punctuation marks.

## Installation

Assuming you have mdBook and [mdbook-katex](https://github.com/lzanini/mdbook-katex) installed, install the crate with

```console
$ cargo install --git https://github.com/yannickseurin/mdbook-mathpunc
```

Then add it as a preprocessor to your `book.toml`:

```toml
[preprocessor.mathpunc]
before = ["katex"]
```

The `before = ["katex"]` line ensures that mathpunc is run *before* the katex preprocessor.

## Implementation

This is very basic: the preprocessor simply replaces all occurrences of "\$p", where p is zero or one closing parenthesis followed by one of the five punctuation marks {, . ; : )} (possibly with zero or more white spaces between the dollar sign and p) by "p$", except if the dollar sign is escaped with a backslash.
It does not handle other punctuation marks such as ? or ! as it is uncommon to have a math block followed by these marks.
It uses the [fancy-regex](https://github.com/fancy-regex/fancy-regex) crate to do this.

## TODO

Currently the preprocessor only handles the default delimiter for inline math, namely $. The mdbook-katex preprocessor allows to define custom delimiters for inline math, e.g. `\( ... \)`. It would be nice to handle custom delimiters as well here.