use std::{fmt, io};

pub struct ExpectedError {
    pub(crate) expected: String,
    pub(crate) received: Vec<u8>,
    pub(crate) source: Option<io::Error>,
}

pub struct UnexpectedIoError {
    pub(crate) context: String,
    pub(crate) source: io::Error,
}

pub enum ParseError {
    Expected(ExpectedError),
    UnexpectedIo(UnexpectedIoError),
}

impl ParseError {
    pub fn expected<S: ToString>(what: S, received: Vec<u8>, source: Option<io::Error>) -> Self {
        Self::Expected(ExpectedError {
            expected: what.to_string(),
            received,
            source,
        })
    }

    pub fn unexpected<S: ToString>(context: S, source: io::Error) -> Self {
        Self::UnexpectedIo(UnexpectedIoError {
            context: context.to_string(),
            source,
        })
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

fn fmt_source_debug(source: &io::Error, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use fmt::Debug;

    f.write_str("\n\n")?;
    f.write_str("Source: ")?;
    source.fmt(f)?;
    Ok(())
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Expected(ExpectedError {
                expected,
                received,
                source,
            }) => {
                f.write_str(&format!("Expected {expected} but received {received:?}"))?;

                if let Some(source) = source {
                    fmt_source_debug(source, f)?;
                }
            }
            ParseError::UnexpectedIo(UnexpectedIoError { context, source }) => {
                f.write_str("Unexpected IO error while ")?;
                context.fmt(f)?;

                fmt_source_debug(source, f)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Expected(ExpectedError {
                expected, received, ..
            }) => {
                f.write_str(&format!("Expected {expected} but received {received:?}"))?;
            }
            ParseError::UnexpectedIo(UnexpectedIoError { context, .. }) => {
                f.write_str("Unexpected IO error while ")?;
                context.fmt(f)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::Expected(ExpectedError { source, .. }) => source.as_ref().map(|s| s as _),
            ParseError::UnexpectedIo(UnexpectedIoError { source, .. }) => Some(source),
        }
    }
}

pub trait ParserContext<T> {
    fn context(self, ctx: &'static str) -> ParseResult<T>;
}

impl<T> ParserContext<T> for io::Result<T> {
    fn context(self, ctx: &'static str) -> ParseResult<T> {
        self.map_err(|source| {
            ParseError::UnexpectedIo(UnexpectedIoError {
                context: ctx.to_string(),
                source,
            })
        })
    }
}

pub fn string_error_to_parse_error<'a>(
    expected: &'a str,
) -> impl Fn((io::Error, Vec<u8>)) -> ParseError + 'a {
    move |(error, bytes)| match error.kind() {
        io::ErrorKind::UnexpectedEof => ParseError::expected(
            format!("{expected} ending with a null byte"),
            bytes,
            Some(error),
        ),
        io::ErrorKind::InvalidData => {
            ParseError::expected(format!("valid utf-8 for {expected}"), bytes, Some(error))
        }
        _ => ParseError::unexpected(format!("parsing {expected} string"), error),
    }
}

#[cfg(test)]
mod tests {
    use super::ParseError;
    use std::io::{self, ErrorKind};

    #[test]
    fn debug_formatting() {
        let error = ParseError::expected(
            "unity version",
            Vec::new(),
            Some(io::Error::from(ErrorKind::UnexpectedEof)),
        );
        assert_eq!(
            format!("{error:?}"),
            "Expected unity version but received []\n\nSource: Kind(UnexpectedEof)"
        );

        let error =
            ParseError::unexpected("reading assets file", io::Error::from(ErrorKind::NotFound));
        assert_eq!(
            format!("{error:?}"),
            "Unexpected IO error while \"reading assets file\"\n\nSource: Kind(NotFound)"
        );
    }
}
