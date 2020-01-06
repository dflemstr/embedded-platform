#[derive(Clone, Copy, Debug)]
pub enum Error {
    Eof,
    WriteZero,
}

impl embedded_platform::io::ReadError for Error {
    fn eof() -> Self {
        Error::Eof
    }
}

impl embedded_platform::io::WriteError for Error {
    fn write_zero() -> Self {
        Error::WriteZero
    }
}
