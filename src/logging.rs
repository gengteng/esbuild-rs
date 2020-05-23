use crate::ast::Location;
use std::fmt;
use std::ops::Range;
use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

// Logging is currently designed to look and feel like clang's error format.
// Errors are streamed asynchronously as they happen, each error contains the
// contents of the line with the error, and the error count is limited by
// default.

pub struct Log {
    pub sender: SyncSender<Msg>,
    pub receiver: Receiver<Msg>,
}

impl Default for Log {
    fn default() -> Self {
        let (sender, receiver) = sync_channel(1024);
        Self { sender, receiver }
    }
}

impl Log {
    pub fn clone_sender(&self) -> SyncSender<Msg> {
        self.sender.clone()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum MsgKind {
    Error = 0,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Msg {
    pub source: Source,
    pub start: usize,
    pub length: usize,
    pub text: String,
    pub kind: MsgKind,
}

#[derive(Debug, Clone)]
pub struct Source {
    pub index: u32,
    pub is_stdin: bool,
    pub absolute_path: PathBuf,
    pub pretty_path: PathBuf,
    pub contents: String,
}

impl Source {
    pub fn text_for_range(&self, range: Range<usize>) -> String {
        self.contents[range].to_owned()
    }

    pub fn range_of_string(&self, location: Location) -> Range<usize> {
        let bytes = self.contents.as_bytes();
        let text = &bytes[location..];

        if text.is_empty() {
            return location..location;
        }

        let quote = text[0];
        if quote == '"' as u8 || quote == '\'' as u8 {
            let mut i = 1;
            while i < text.len() {
                let c = text[i];
                if c == quote {
                    return location..i + 1;
                } else if c == '\\' as u8 {
                    i += 1;
                }
            }
        }

        location..location
    }
}

#[derive(Debug, Clone)]
pub struct MsgCounts {
    pub errors: usize,
    pub warnings: usize,
}

fn plural(prefix: &str, count: usize) -> String {
    if count == 1 {
        format!("{} {}", count, prefix)
    } else {
        format!("{} {}s", count, prefix)
    }
}

impl fmt::Display for MsgCounts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors == 0 {
            if self.warnings == 0 {
                write!(f, "no errors")
            } else {
                write!(f, "{}", plural("warning", self.warnings))
            }
        } else {
            if self.warnings == 0 {
                write!(f, "{}", plural("error", self.errors))
            } else {
                write!(
                    f,
                    "{} and {}",
                    plural("warning", self.warnings),
                    plural("error", self.errors)
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TerminalInfo {
    is_tty: bool,
    use_color_escapes: bool,
    width: usize,
}

pub const COLOR_RESET: &str = "\033[0m";
pub const COLOR_RED: &str = "\033[31m";
pub const COLOR_GREEN: &str = "\033[32m";
pub const COLOR_MAGENTA: &str = "\033[35m";
pub const COLOR_BOLD: &str = "\033[1m";
pub const COLOR_RESET_BOLD: &str = "\033[0;1m";

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum StderrColor {
    IfTerminal = 0,
    Never,
    Always,
}

#[derive(Debug, Clone)]
pub struct StderrOptions {
    pub include_source: bool,
    pub error_limit: usize,
    pub exit_when_limit_is_hit: bool,
    pub color: StderrColor,
}

#[derive(Debug, Clone)]
pub struct MsgDetail {
    pub path: PathBuf,
    pub line: usize,
    pub column: usize,
    pub kind: String,
    pub message: String,

    pub source: String,
    pub source_before: String,
    pub source_marked: String,
    pub source_after: String,

    pub indent: String,
    pub marker: String,
}
