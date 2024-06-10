use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub struct NoMidiOutputsError;

impl fmt::Display for NoMidiOutputsError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "no MIDI device output ports found")
    }
}

impl Error for NoMidiOutputsError {}
