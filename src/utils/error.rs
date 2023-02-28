use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParserError {
    details: String,
}

#[derive(Debug)]
pub struct RendererError {
    details: String,
}

#[derive(Debug)]
pub struct FsError {
    details: String,
}

macro_rules! impl_error {
    ($ty:ty) => {
        impl $ty {
            pub fn new(msg: &str) -> Self {
                Self {
                    details: msg.to_string(),
                }
            }
        }

        impl fmt::Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.details)
            }
        }

        impl Error for $ty {
            fn description(&self) -> &str {
                &self.details
            }
        }
    };
}

impl_error!(ParserError);
impl_error!(RendererError);
impl_error!(FsError);
