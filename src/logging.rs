use crate::ast::Location;
use std::fmt;
use std::ops::{Range, RangeFrom, RangeTo};
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

impl fmt::Display for MsgKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MsgKind::Error => "error",
            MsgKind::Warning => "warning",
        })
    }
}

#[derive(Debug, Clone)]
pub struct Msg {
    pub source: Source,
    pub start: usize,
    pub length: usize,
    pub text: String,
    pub kind: MsgKind,
}

impl Msg {
    pub fn to_terminal_string(
        &self,
        options: &StderrOptions,
        terminal_info: &TerminalInfo,
    ) -> String {
        let (kind, kind_color) = match self.kind {
            MsgKind::Error => ("error", COLOR_RED),
            MsgKind::Warning => ("warning", COLOR_MAGENTA),
        };

        if self.source.pretty_path.is_empty() {
            if terminal_info.use_color_escapes {
                return format!(
                    "{}{}{}: {}{}{}\n",
                    COLOR_BOLD, kind_color, kind, COLOR_RESET_BOLD, self.text, COLOR_RESET
                );
            }

            return format!("{}: {}\n", kind, self.text);
        }

        if !options.include_source {
            if terminal_info.use_color_escapes {
                return format!(
                    "{}{}: {}{}: {}{}{}\n",
                    COLOR_BOLD,
                    self.source.pretty_path,
                    kind_color,
                    kind,
                    COLOR_RESET_BOLD,
                    self.text,
                    COLOR_RESET
                );
            }

            return format!("{}: {}: {}\n", self.source.pretty_path, kind, self.text);
        }

        let detail = MsgDetail::new(self, terminal_info);

        if terminal_info.use_color_escapes {
            format!(
                "{}{}:{}:{}: {}{}: {}{}\n{}{}{}{}{}{}\n{}{}{}{}\n",
                COLOR_BOLD,
                detail.path,
                detail.line,
                detail.column,
                kind_color,
                detail.kind,
                COLOR_RESET_BOLD,
                detail.message,
                COLOR_RESET,
                detail.source_before(),
                COLOR_GREEN,
                detail.source_marked(),
                COLOR_RESET,
                detail.source_after(),
                COLOR_GREEN,
                detail.indent,
                detail.marker,
                COLOR_RESET
            )
        } else {
            format!(
                "{}:{}:{}: {}: {}\n{}\n{}{}\n",
                detail.path,
                detail.line,
                detail.column,
                detail.kind,
                detail.message,
                detail.source,
                detail.indent,
                detail.marker
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct Source {
    pub index: u32,
    pub is_stdin: bool,
    pub absolute_path: String,
    pub pretty_path: String,
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
        if quote == b'"' || quote == b'\'' {
            let mut i = 1;
            while i < text.len() {
                let c = text[i];
                if c == quote {
                    return location..i + 1;
                } else if c == b'\\' {
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
        } else if self.warnings == 0 {
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

#[derive(Debug, Clone)]
pub struct TerminalInfo {
    is_tty: bool,
    use_color_escapes: bool,
    width: usize,
}

impl Default for TerminalInfo {
    fn default() -> Self {
        Self {
            is_tty: atty::is(atty::Stream::Stderr),
            use_color_escapes: true,
            width: terminal_size::terminal_size()
                .map(|(w, _)| w.0 as usize)
                .unwrap_or(0),
        }
    }
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

pub fn compute_line_and_column(text: &str) -> (usize, usize, usize) {
    let mut prev_code = '\0';
    let mut last_line_start = 0;
    let mut line_count = 0;

    for (i, code) in text.chars().enumerate() {
        match code {
            '\n' => {
                last_line_start = i + 1;
                if prev_code != '\r' {
                    line_count += 1;
                }
            }
            '\r' | '\u{2028}' | '\u{2029}' => {
                last_line_start = i + 1;
            }
            _ => {}
        }
        prev_code = code;
    }

    let column_count = text.len() - last_line_start;

    (line_count, column_count, last_line_start)
}

#[derive(Debug, Clone)]
pub struct MsgDetail {
    pub path: String,
    pub line: usize,
    pub column: usize,
    pub kind: String,
    pub message: String,

    pub source: String,
    pub source_before: RangeTo<usize>,
    pub source_marked: Range<usize>,
    pub source_after: RangeFrom<usize>,

    pub indent: String,
    pub marker: String,
}

impl MsgDetail {
    pub fn new(msg: &Msg, terminal_info: &TerminalInfo) -> Self {
        let contents = &msg.source.contents;
        let (line_count, col_count, line_start) = compute_line_and_column(&contents[0..msg.start]);
        let mut line_end = contents.len();

        'a: for (i, code) in contents[line_start..].chars().enumerate() {
            match code {
                '\r' | '\n' | '\u{2028}' | '\u{2029}' => {
                    line_end = line_start + i;
                    break 'a;
                }
                _ => {}
            }
        }

        let spaces_per_tab = 2;
        let mut line_text = render_tab_stops(&contents[line_start..line_end], spaces_per_tab);
        let mut indent = " ".repeat(render_tab_stops_len(
            &contents[line_start..msg.start],
            spaces_per_tab,
        ));
        let mut marker_start = indent.len();
        let mut marker_end = if msg.length > 0 {
            // Extend markers to cover the full range of the error
            render_tab_stops_len(&contents[line_start..msg.start], spaces_per_tab)
        } else {
            indent.len()
        };

        let line_text_len = line_text.len();

        // Clip the marker to the bounds of the line
        if marker_start > line_text_len {
            marker_start = line_text_len;
        }

        if marker_end > line_text_len {
            marker_end = line_text_len;
        }

        if marker_end < marker_start {
            marker_end = marker_start;
        }

        // Trim the line to fit the terminal width
        if terminal_info.width > 0 && line_text_len > terminal_info.width {
            // TODO: Try to center the error
            let mut slice_start = if marker_start + marker_end >= terminal_info.width {
                let slice_start = (marker_start + marker_end - terminal_info.width) / 2;
                if marker_start >= terminal_info.width / 5 {
                    let temp = marker_start - terminal_info.width / 5;
                    if slice_start > temp {
                        temp
                    } else {
                        slice_start
                    }
                } else {
                    0
                }
            } else {
                0
            };

            if slice_start > line_text_len - terminal_info.width {
                slice_start = line_text_len - terminal_info.width;
            }
            let slice_end = slice_start + terminal_info.width;

            // Slice the line
            let mut sliced_line = line_text[slice_start..slice_end].to_owned();
            marker_start = if marker_start > slice_start {
                marker_start - slice_start
            } else {
                0
            };
            if marker_end > sliced_line.len() {
                marker_end = sliced_line.len();
            }

            // Truncate the ends with "..."
            if sliced_line.len() > 3 && slice_start > 0 {
                sliced_line = "...".to_owned() + &sliced_line[3..];
            }

            // TODO: ...

            // Now we can compute the indent
            indent = " ".repeat(marker_start);
            line_text = sliced_line;
        }

        MsgDetail {
            path: msg.source.pretty_path.clone(),
            line: line_count + 1,
            column: col_count,
            kind: msg.kind.to_string(),
            message: msg.text.to_owned(),
            source: line_text,
            source_before: ..marker_start,
            source_marked: marker_start..marker_end,
            source_after: marker_end..,
            indent,
            marker: if marker_end - marker_start > 1 {
                "~".repeat(marker_end - marker_start)
            } else {
                "^".to_owned()
            },
        }
    }

    pub fn source_before(&self) -> &str {
        &self.source[self.source_before]
    }

    pub fn source_marked(&self) -> &str {
        &self.source[self.source_marked.clone()]
    }

    pub fn source_after(&self) -> &str {
        &self.source[self.source_after.clone()]
    }
}

fn render_tab_stops_len(with_tabs: &str, spaces_per_tab: usize) -> usize {
    if !with_tabs.contains('\t') {
        return with_tabs.len();
    }

    let mut count = 0;

    for c in with_tabs.chars() {
        match c {
            '\t' => {
                count += spaces_per_tab - (count % spaces_per_tab);
            }
            _ => {
                count += 1;
            }
        }
    }

    count
}

fn render_tab_stops(with_tabs: &str, spaces_per_tab: usize) -> String {
    if !with_tabs.contains('\t') {
        return with_tabs.to_owned();
    }

    let mut without_tabs = String::new();
    let mut count = 0;

    for c in with_tabs.chars() {
        match c {
            '\t' => {
                let spaces = spaces_per_tab - (count % spaces_per_tab);

                for _ in 0..spaces {
                    without_tabs.push(' ');
                    count += 1;
                }
            }
            c => {
                without_tabs.push(c);
                count += 1;
            }
        }
    }

    without_tabs
}
