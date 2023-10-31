use clap::{crate_version, Arg, ArgMatches, Command};
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook::errors::{Error, Result};
use semver::{Version, VersionReq};
use mdbook_mathpunc::MathpuncPreprocessor;
use std::io;

/// Parse CLI options.
pub fn make_app() -> Command {
    Command::new("mdbook-mathpunc")
        .version(crate_version!())
        .about("An mdbook preprocessor that prevents line breaks between math and punctuation.")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> Result<()> {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);
    if supported {
        Ok(())
    } else {
        Err(Error::msg(format!(
            "The {} preprocessor does not support the '{}' renderer",
            pre.name(),
            renderer,
        )))
    }
}

fn main() -> Result<()> {
    let matches = make_app().get_matches();

    let preprocessor = MathpuncPreprocessor;

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        // handle cmdline supports
        handle_supports(&preprocessor, sub_args)
    } else {
        // handle preprocessing
        handle_preprocessing(&preprocessor)
    }
}