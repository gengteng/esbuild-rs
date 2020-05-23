use crate::tables::Token;

#[derive(Debug, Clone)]
pub struct Json {
    pub parse: bool,
    pub allow_comments: bool,
}

pub struct Lexer {
    //     log                             logging.Log
    //     source                          logging.Source
    pub current: usize,
    pub start: usize,
    pub end: usize,
    pub token: Token,
    pub has_newline_before: bool,
    pub code_point: char,
    pub string_literal: Vec<u16>,
    pub identifier: String,
    pub number: f64,
    pub rescan_close_brace_as_template_token: bool,
    pub json: Json,

    // The log is disabled during speculative scans that may backtrack
    pub is_log_disabled: bool,
}
