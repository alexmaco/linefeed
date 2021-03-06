//! Provides a configurable, concurrent, extensible, interactive input reader
//! for Unix terminals and Windows console.
//!
//! Configuration is compatible with GNU Readline.
//!
//! The main entry to interactive read operations is the [`Interface`] type.
//!
//! # Basic example
//!
//! ```no_run
//! use std::io;
//! use linefeed::{Interface, ReadResult};
//!
//! # fn run() -> io::Result<()> {
//! let mut reader = Interface::new("my-application")?;
//!
//! reader.set_prompt("my-app> ");
//!
//! while let ReadResult::Input(input) = reader.read_line()? {
//!     println!("got input {:?}", input);
//! }
//!
//! println!("Goodbye.");
//! # Ok(())
//! # }
//! ```
//!
//! [`Interface`]: interface/struct.Interface.html

#![deny(missing_docs)]

extern crate mortal;

#[cfg(windows)] #[macro_use(DEFINE_GUID)] extern crate winapi;

#[cfg(test)] #[macro_use] extern crate assert_matches;

pub use command::Command;
pub use complete::{Completer, Completion, Suffix};
pub use function::Function;
pub use interface::Interface;
pub use prompter::Prompter;
pub use reader::{Reader, ReadResult};
pub use terminal::{DefaultTerminal, Signal, Terminal};
pub use writer::Writer;

pub mod chars;
pub mod command;
pub mod complete;
pub mod function;
pub mod inputrc;
pub mod interface;
pub mod memory;
pub mod prompter;
pub mod reader;
pub mod table;
pub mod terminal;
mod util;
pub mod variables;
pub mod writer;

#[cfg(unix)]
#[path = "unix/mod.rs"]
mod sys;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod sys;

#[cfg(test)]
mod test {
    use interface::Interface;
    use terminal::{DefaultTerminal, Terminal};

    fn assert_has_traits<T: 'static + Send + Sync>() {}

    fn assert_generic_traits<T: 'static + Terminal>() {
        assert_has_traits::<Interface<T>>();
    }

    #[test]
    fn test_interface_traits() {
        assert_generic_traits::<DefaultTerminal>();
    }
}
