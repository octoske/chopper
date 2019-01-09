/* Some code in this file was adapted from public domain xsv project. */
use std::io;
use std::process;

use crate::args::Args;
use crate::result::{CliResult, CliError};
use crate::process::{collate, read};

#[derive(Debug)]
pub enum Command {
    Collate,
    Read,
}

macro_rules! werr {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).unwrap();
    });
}

impl Command {
    pub fn run(self, argv: Args) {
        match self.real_run(argv) {
            Ok(()) => process::exit(0),
            Err(CliError::Flag(err)) => err.exit(),
            Err(CliError::Csv(err)) => {
                werr!("{}", err);
                process::exit(1);
            },
            Err(CliError::Io(ref err)) if err.kind() == io::ErrorKind::BrokenPipe => {
                process::exit(0);
            },
            Err(CliError::Io(err)) => {
                werr!("{}", err);
                process::exit(1);
            },
            Err(CliError::Other(msg)) => {
                werr!("{}", msg);
                process::exit(1);
            },
        }
    }

    fn real_run(self, argv: Args) -> CliResult<()> {
        match self {
            Command::Collate => collate::run(argv),
            Command::Read => read::run(argv),
        }
    }
}
