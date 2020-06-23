use std::fs;

// declare modules and decide if they are public API
mod mlcomment;
mod ascii;
mod dfa;
mod token;
mod lexeme;
mod pda;
mod driver;
mod qualified_identifiers;
mod reserved;
mod regex_backend;

/// Every module has its `Error` enum that has that module-specific variants and
/// also accepts variants from the modules that it delegates work to. Example
mod a {
    enum Error {
        AFailedFirst,
        AFailedSecond,
        BFailed(b::Error),
    }

    fn work() -> Result<u8, Error> {
        match b::work() {
            Err(e) => {
                Err(Error::BFailed(e))
            },
            Ok(val) => {
                Ok(val)
            }
        }
    }

    mod b {
        pub enum Error {
            BFailedFirst,
            BFailedSecond,
        }

        pub fn work() -> Result<u8, Error> {
            Ok(b'a')
        }
    }
}
