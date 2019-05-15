pub type Result<O> = ::std::result::Result<O, Error>;

#[derive(Debug)]
pub enum Error {
    TerminalError(crossterm::ErrorKind)
}

impl From<crossterm::ErrorKind> for Error {
    fn from(err: crossterm::ErrorKind) -> Self {
        Error::TerminalError(err)
    }
}
