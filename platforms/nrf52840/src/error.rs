#[derive(Debug)]
pub enum Error {
    Eof,
    WriteZero,
    Uarte(nrf52840_hal::uarte::Error),
    Spim(nrf52840_hal::spim::Error),
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

impl From<nrf52840_hal::uarte::Error> for Error {
    fn from(err: nrf52840_hal::uarte::Error) -> Self {
        Error::Uarte(err)
    }
}

impl From<nrf52840_hal::spim::Error> for Error {
    fn from(err: nrf52840_hal::spim::Error) -> Self {
        Error::Spim(err)
    }
}
