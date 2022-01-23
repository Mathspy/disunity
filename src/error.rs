use std::{borrow::Cow, fmt, io};

pub struct ExpectedError {
    pub(crate) expected: Cow<'static, str>,
    pub(crate) received: Vec<u8>,
    pub(crate) source: io::Error,
}

pub struct UnexpectedIoError {
    pub(crate) context: Cow<'static, str>,
    pub(crate) source: io::Error,
}

pub enum ParseError {
    Expected(ExpectedError),
    UnexpectedIo(UnexpectedIoError),
}

impl ParseError {
    pub fn expected(what: &'static str, received: Vec<u8>, source: io::Error) -> Self {
        Self::Expected(ExpectedError {
            expected: Cow::Borrowed(what),
            received,
            source,
        })
    }

    pub fn unexpected(context: &'static str, source: io::Error) -> Self {
        Self::UnexpectedIo(UnexpectedIoError {
            context: Cow::Borrowed(context),
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

                fmt_source_debug(source, f)?;
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
            ParseError::Expected(ExpectedError { source, .. }) => Some(source),
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
                context: Cow::Borrowed(ctx),
                source,
            })
        })
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
            io::Error::from(ErrorKind::UnexpectedEof),
        );
        assert_eq!(
            format!("{error:?}"),
            "Expected unity version but received []\n\nSource: Kind(UnexpectedEof)"
        );

        let error =
            ParseError::unexpected("reading assets file", io::Error::from(ErrorKind::NotFound));
        assert_eq!(
            format!("{error:?}"),
            "Unexpected IO error while \"reading asset file\"\n\nSource: Kind(NotFound)"
        );
    }
}
