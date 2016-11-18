use std;

///Error what occurs on start_recording, stop_recording, read methods of AudioDevice
#[derive(Debug)]
pub enum Error{
    Gen,
    NotOpen,
    Wave,
    Unknown,
}

impl From<i32> for Error {
    fn from(n:i32) -> Error {
        match n {
            -1 => Error::Gen,
            -2 => Error::NotOpen,
            -3 => Error::Wave,
            _ => Error::Unknown,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Sphinx audio device error:{}",
            match *self {
                Error::Gen => "Gen",
                Error::NotOpen => "NotOpen",
                Error::Wave => "Wave",
                Error::Unknown => "Unknown",
            }
        )
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Gen => "Gen",
            Error::NotOpen => "NotOpen",
            Error::Wave => "Wave",
            Error::Unknown => "Unknown",
        }
    }
    fn cause(&self) -> Option<&std::error::Error> { None }
}
