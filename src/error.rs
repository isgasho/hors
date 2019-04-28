extern crate reqwest;

use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct HorError {
    repr: Repr,
}

impl Error for HorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.repr {
            Repr::Network(network_err) => {
                return network_err.source();
            }
            Repr::Parse(_) => {
                return None;
            }
        }
    }
}

impl Display for HorError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "asdf")
    }
}

#[derive(Debug)]
enum Repr {
    Network(reqwest::Error),
    Parse(&'static str),
}

impl HorError {
    pub fn from_parse(reason: &'static str) -> HorError {
        return HorError {
            repr: Repr::Parse(reason),
        };
    }
}

// TODO: Maybe implement an converter to convert from reqwest::Error to HorError?
