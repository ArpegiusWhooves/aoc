use lalrpop_util::ParseError;

use miette::{Diagnostic, NamedSource, SourceOffset, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("Parse error: {message}")]
#[diagnostic(code(E2202C0DE), severity(Error))]
pub struct MyBad {
    message: String,
    #[source_code]
    src: NamedSource,
    #[label]
    highlight: SourceSpan,
}

impl MyBad {
    pub fn new(
        filename: String,
        source: String,
        (row, column, length): (usize, usize, usize),
        error: String,
    ) -> MyBad {
        let offsset = SourceOffset::from_location(&source, row, column);
        MyBad {
            src: NamedSource::new(filename, source),
            highlight: SourceSpan::new(offsset, length.into()),
            message: error,
        }
    }

    pub fn with_source(mut self, name: &str, source: &str) -> Self {
        self.src = NamedSource::new(name, source.to_owned());
        dbg!(self)
    }
}

impl<T, E> From<ParseError<usize, T, E>> for MyBad
where
    ParseError<usize, T, E>: std::fmt::Display,
    E: std::fmt::Display,
{
    fn from(error: ParseError<usize, T, E>) -> Self {
        match &error {
            ParseError::InvalidToken { location } => MyBad {
                src: NamedSource::new("", ""),
                highlight: (*location, 1usize).into(),
                message: format!("{error}"),
            },
            ParseError::UnrecognizedEOF { location, .. } => MyBad {
                src: NamedSource::new("", ""),
                highlight: (*location, 1usize).into(),
                message: format!("{error}"),
            },
            ParseError::UnrecognizedToken { token, .. } => MyBad {
                src: NamedSource::new("", ""),
                highlight: (token.0, token.2).into(),
                message: format!("{error}"),
            },
            ParseError::ExtraToken { token } => MyBad {
                src: NamedSource::new("", ""),
                highlight: (token.0, token.2).into(),
                message: format!("{error}"),
            },
            ParseError::User { error } => MyBad {
                src: NamedSource::new("", ""),
                highlight: (0, 0).into(),
                message: format!("{error}"),
            },
        }
    }
}
