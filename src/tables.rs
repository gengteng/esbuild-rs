use crate::error::Error;
use std::convert::TryFrom;
use std::ops::RangeInclusive;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Token {
    EndOfFile = 0,
    SyntaxError,

    // "#!/usr/bin/env node",
    Hashbang,

    // Literals,
    NoSubstitutionTemplateLiteral, // Contents are in lexer.StringLiteral ([]uint16),
    NumericLiteral,                // Contents are in lexer.Number (float64),
    StringLiteral,                 // Contents are in lexer.StringLiteral ([]uint16),
    BigIntegerLiteral,             // Contents are in lexer.Identifier (string),

    // Pseudo-literals,
    TemplateHead,   // Contents are in lexer.StringLiteral ([]uint16),
    TemplateMiddle, // Contents are in lexer.StringLiteral ([]uint16),
    TemplateTail,   // Contents are in lexer.StringLiteral ([]uint16),

    // Punctuation,
    Ampersand,
    AmpersandAmpersand,
    Asterisk,
    AsteriskAsterisk,
    At,
    Bar,
    BarBar,
    Caret,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Comma,
    Dot,
    DotDotDot,
    EqualsEquals,
    EqualsEqualsEquals,
    EqualsGreaterThan,
    Exclamation,
    ExclamationEquals,
    ExclamationEqualsEquals,
    GreaterThan,
    GreaterThanEquals,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThan,
    LessThan,
    LessThanEquals,
    LessThanLessThan,
    Minus,
    MinusMinus,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Percent,
    Plus,
    PlusPlus,
    Question,
    QuestionDot,
    QuestionQuestion,
    Semicolon,
    Slash,
    Tilde,

    // Assignments,
    AmpersandEquals,
    AsteriskAsteriskEquals,
    AsteriskEquals,
    BarEquals,
    CaretEquals,
    Equals,
    GreaterThanGreaterThanEquals,
    GreaterThanGreaterThanGreaterThanEquals,
    LessThanLessThanEquals,
    MinusEquals,
    PercentEquals,
    PlusEquals,
    SlashEquals,

    // Identifiers,
    Identifier,     // Contents are in lexer.Identifier (string),
    EscapedKeyword, // A keyword that has been escaped as an identifer,

    // Reserved words,
    Break,
    Case,
    Catch,
    Class,
    Const,
    Continue,
    Debugger,
    Default,
    Delete,
    Do,
    Else,
    Enum,
    Export,
    Extends,
    False,
    Finally,
    For,
    Function,
    If,
    Import,
    In,
    Instanceof,
    New,
    Null,
    Return,
    Super,
    Switch,
    This,
    Throw,
    True,
    Try,
    Typeof,
    Var,
    Void,
    While,
    With,

    // Strict mode reserved words,
    Implements,
    Interface,
    Let,
    Package,
    Private,
    Protected,
    Public,
    Static,
    Yield,
}

impl Token {
    pub fn to_str(self) -> &'static str {
        match self {
            Token::EndOfFile => "end of file",
            Token::SyntaxError => "syntax error",
            Token::Hashbang => "hashbang comment",

            // Literals
            Token::NoSubstitutionTemplateLiteral => "template literal",
            Token::NumericLiteral => "number",
            Token::StringLiteral => "string",
            Token::BigIntegerLiteral => "bigint",

            // Pseudo-literals
            Token::TemplateHead => "template literal",
            Token::TemplateMiddle => "template literal",
            Token::TemplateTail => "template literal",

            // Punctuation
            Token::Ampersand => r#""&""#,
            Token::AmpersandAmpersand => r#""&&""#,
            Token::Asterisk => r#""*""#,
            Token::AsteriskAsterisk => r#""**""#,
            Token::At => r#""@""#,
            Token::Bar => r#""|""#,
            Token::BarBar => r#""||""#,
            Token::Caret => r#""^""#,
            Token::CloseBrace => r#""}""#,
            Token::CloseBracket => r#""]""#,
            Token::CloseParen => r#"")""#,
            Token::Colon => r#"":""#,
            Token::Comma => r#"",""#,
            Token::Dot => r#"".""#,
            Token::DotDotDot => r#""...""#,
            Token::EqualsEquals => r#""==""#,
            Token::EqualsEqualsEquals => r#""===""#,
            Token::EqualsGreaterThan => r#""=>""#,
            Token::Exclamation => r#""!""#,
            Token::ExclamationEquals => r#""!=""#,
            Token::ExclamationEqualsEquals => r#""!==""#,
            Token::GreaterThan => r#"">""#,
            Token::GreaterThanEquals => r#"">=""#,
            Token::GreaterThanGreaterThan => r#"">>""#,
            Token::GreaterThanGreaterThanGreaterThan => r#"">>>""#,
            Token::LessThan => r#""<""#,
            Token::LessThanEquals => r#""<=""#,
            Token::LessThanLessThan => r#""<<""#,
            Token::Minus => r#""-""#,
            Token::MinusMinus => r#""--""#,
            Token::OpenBrace => r#""{""#,
            Token::OpenBracket => r#""[""#,
            Token::OpenParen => r#""(""#,
            Token::Percent => r#""%""#,
            Token::Plus => r#""+""#,
            Token::PlusPlus => r#""++""#,
            Token::Question => r#""?""#,
            Token::QuestionDot => r#""?.""#,
            Token::QuestionQuestion => r#""??""#,
            Token::Semicolon => r#"";""#,
            Token::Slash => r#""/""#,
            Token::Tilde => r#""~""#,

            // Assignments
            Token::AmpersandEquals => r#""&=""#,
            Token::AsteriskAsteriskEquals => r#""**=""#,
            Token::AsteriskEquals => r#""*=""#,
            Token::BarEquals => r#""|=""#,
            Token::CaretEquals => r#""^=""#,
            Token::Equals => r#""=""#,
            Token::GreaterThanGreaterThanEquals => r#"">>=""#,
            Token::GreaterThanGreaterThanGreaterThanEquals => r#"">>>=""#,
            Token::LessThanLessThanEquals => r#""<<=""#,
            Token::MinusEquals => r#""-=""#,
            Token::PercentEquals => r#""%=""#,
            Token::PlusEquals => r#""+=""#,
            Token::SlashEquals => r#""/=""#,

            // Identifiers
            Token::Identifier => "identifier",
            Token::EscapedKeyword => "escaped keyword",

            // Reserved words
            Token::Break => r#""break""#,
            Token::Case => r#""case""#,
            Token::Catch => r#""catch""#,
            Token::Class => r#""class""#,
            Token::Const => r#""const""#,
            Token::Continue => r#""continue""#,
            Token::Debugger => r#""debugger""#,
            Token::Default => r#""default""#,
            Token::Delete => r#""delete""#,
            Token::Do => r#""do""#,
            Token::Else => r#""else""#,
            Token::Enum => r#""enum""#,
            Token::Export => r#""export""#,
            Token::Extends => r#""extends""#,
            Token::False => r#""false""#,
            Token::Finally => r#""finally""#,
            Token::For => r#""for""#,
            Token::Function => r#""function""#,
            Token::If => r#""if""#,
            Token::Import => r#""import""#,
            Token::In => r#""in""#,
            Token::Instanceof => r#""instanceof""#,
            Token::New => r#""new""#,
            Token::Null => r#""null""#,
            Token::Return => r#""return""#,
            Token::Super => r#""super""#,
            Token::Switch => r#""switch""#,
            Token::This => r#""this""#,
            Token::Throw => r#""throw""#,
            Token::True => r#""true""#,
            Token::Try => r#""try""#,
            Token::Typeof => r#""typeof""#,
            Token::Var => r#""var""#,
            Token::Void => r#""void""#,
            Token::While => r#""while""#,
            Token::With => r#""with""#,

            // Strict mode reserved words
            Token::Implements => r#""implements""#,
            Token::Interface => r#""interface""#,
            Token::Let => r#""let""#,
            Token::Package => r#""package""#,
            Token::Private => r#""private""#,
            Token::Protected => r#""protected""#,
            Token::Public => r#""public""#,
            Token::Static => r#""static""#,
            Token::Yield => r#""yield""#,
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            // Reserved words
            "break" => Token::Break,
            "case" => Token::Case,
            "catch" => Token::Catch,
            "class" => Token::Class,
            "const" => Token::Const,
            "continue" => Token::Continue,
            "debugger" => Token::Debugger,
            "default" => Token::Default,
            "delete" => Token::Delete,
            "do" => Token::Do,
            "else" => Token::Else,
            "enum" => Token::Enum,
            "export" => Token::Export,
            "extends" => Token::Extends,
            "false" => Token::False,
            "finally" => Token::Finally,
            "for" => Token::For,
            "function" => Token::Function,
            "if" => Token::If,
            "import" => Token::Import,
            "in" => Token::In,
            "instanceof" => Token::Instanceof,
            "new" => Token::New,
            "null" => Token::Null,
            "return" => Token::Return,
            "super" => Token::Super,
            "switch" => Token::Switch,
            "this" => Token::This,
            "throw" => Token::Throw,
            "true" => Token::True,
            "try" => Token::Try,
            "typeof" => Token::Typeof,
            "var" => Token::Var,
            "void" => Token::Void,
            "while" => Token::While,
            "with" => Token::With,

            // Strict mode reserved words
            "implements" => Token::Implements,
            "interface" => Token::Interface,
            "let" => Token::Let,
            "package" => Token::Package,
            "private" => Token::Private,
            "protected" => Token::Protected,
            "public" => Token::Public,
            "static" => Token::Static,
            "yield" => Token::Yield,

            _ => return Err(Error::NotFound),
        })
    }
}

// This is from https://github.com/microsoft/TypeScript/blob/master/src/compiler/transformers/jsx.ts
pub fn jsx_entry(s: &str) -> Option<char> {
    match s {
        "quot" => Some(0x0022u32),
        "amp" => Some(0x0026),
        "apos" => Some(0x0027),
        "lt" => Some(0x003C),
        "gt" => Some(0x003E),
        "nbsp" => Some(0x00A0),
        "iexcl" => Some(0x00A1),
        "cent" => Some(0x00A2),
        "pound" => Some(0x00A3),
        "curren" => Some(0x00A4),
        "yen" => Some(0x00A5),
        "brvbar" => Some(0x00A6),
        "sect" => Some(0x00A7),
        "uml" => Some(0x00A8),
        "copy" => Some(0x00A9),
        "ordf" => Some(0x00AA),
        "laquo" => Some(0x00AB),
        "not" => Some(0x00AC),
        "shy" => Some(0x00AD),
        "reg" => Some(0x00AE),
        "macr" => Some(0x00AF),
        "deg" => Some(0x00B0),
        "plusmn" => Some(0x00B1),
        "sup2" => Some(0x00B2),
        "sup3" => Some(0x00B3),
        "acute" => Some(0x00B4),
        "micro" => Some(0x00B5),
        "para" => Some(0x00B6),
        "middot" => Some(0x00B7),
        "cedil" => Some(0x00B8),
        "sup1" => Some(0x00B9),
        "ordm" => Some(0x00BA),
        "raquo" => Some(0x00BB),
        "frac14" => Some(0x00BC),
        "frac12" => Some(0x00BD),
        "frac34" => Some(0x00BE),
        "iquest" => Some(0x00BF),
        "Agrave" => Some(0x00C0),
        "Aacute" => Some(0x00C1),
        "Acirc" => Some(0x00C2),
        "Atilde" => Some(0x00C3),
        "Auml" => Some(0x00C4),
        "Aring" => Some(0x00C5),
        "AElig" => Some(0x00C6),
        "Ccedil" => Some(0x00C7),
        "Egrave" => Some(0x00C8),
        "Eacute" => Some(0x00C9),
        "Ecirc" => Some(0x00CA),
        "Euml" => Some(0x00CB),
        "Igrave" => Some(0x00CC),
        "Iacute" => Some(0x00CD),
        "Icirc" => Some(0x00CE),
        "Iuml" => Some(0x00CF),
        "ETH" => Some(0x00D0),
        "Ntilde" => Some(0x00D1),
        "Ograve" => Some(0x00D2),
        "Oacute" => Some(0x00D3),
        "Ocirc" => Some(0x00D4),
        "Otilde" => Some(0x00D5),
        "Ouml" => Some(0x00D6),
        "times" => Some(0x00D7),
        "Oslash" => Some(0x00D8),
        "Ugrave" => Some(0x00D9),
        "Uacute" => Some(0x00DA),
        "Ucirc" => Some(0x00DB),
        "Uuml" => Some(0x00DC),
        "Yacute" => Some(0x00DD),
        "THORN" => Some(0x00DE),
        "szlig" => Some(0x00DF),
        "agrave" => Some(0x00E0),
        "aacute" => Some(0x00E1),
        "acirc" => Some(0x00E2),
        "atilde" => Some(0x00E3),
        "auml" => Some(0x00E4),
        "aring" => Some(0x00E5),
        "aelig" => Some(0x00E6),
        "ccedil" => Some(0x00E7),
        "egrave" => Some(0x00E8),
        "eacute" => Some(0x00E9),
        "ecirc" => Some(0x00EA),
        "euml" => Some(0x00EB),
        "igrave" => Some(0x00EC),
        "iacute" => Some(0x00ED),
        "icirc" => Some(0x00EE),
        "iuml" => Some(0x00EF),
        "eth" => Some(0x00F0),
        "ntilde" => Some(0x00F1),
        "ograve" => Some(0x00F2),
        "oacute" => Some(0x00F3),
        "ocirc" => Some(0x00F4),
        "otilde" => Some(0x00F5),
        "ouml" => Some(0x00F6),
        "divide" => Some(0x00F7),
        "oslash" => Some(0x00F8),
        "ugrave" => Some(0x00F9),
        "uacute" => Some(0x00FA),
        "ucirc" => Some(0x00FB),
        "uuml" => Some(0x00FC),
        "yacute" => Some(0x00FD),
        "thorn" => Some(0x00FE),
        "yuml" => Some(0x00FF),
        "OElig" => Some(0x0152),
        "oelig" => Some(0x0153),
        "Scaron" => Some(0x0160),
        "scaron" => Some(0x0161),
        "Yuml" => Some(0x0178),
        "fnof" => Some(0x0192),
        "circ" => Some(0x02C6),
        "tilde" => Some(0x02DC),
        "Alpha" => Some(0x0391),
        "Beta" => Some(0x0392),
        "Gamma" => Some(0x0393),
        "Delta" => Some(0x0394),
        "Epsilon" => Some(0x0395),
        "Zeta" => Some(0x0396),
        "Eta" => Some(0x0397),
        "Theta" => Some(0x0398),
        "Iota" => Some(0x0399),
        "Kappa" => Some(0x039A),
        "Lambda" => Some(0x039B),
        "Mu" => Some(0x039C),
        "Nu" => Some(0x039D),
        "Xi" => Some(0x039E),
        "Omicron" => Some(0x039F),
        "Pi" => Some(0x03A0),
        "Rho" => Some(0x03A1),
        "Sigma" => Some(0x03A3),
        "Tau" => Some(0x03A4),
        "Upsilon" => Some(0x03A5),
        "Phi" => Some(0x03A6),
        "Chi" => Some(0x03A7),
        "Psi" => Some(0x03A8),
        "Omega" => Some(0x03A9),
        "alpha" => Some(0x03B1),
        "beta" => Some(0x03B2),
        "gamma" => Some(0x03B3),
        "delta" => Some(0x03B4),
        "epsilon" => Some(0x03B5),
        "zeta" => Some(0x03B6),
        "eta" => Some(0x03B7),
        "theta" => Some(0x03B8),
        "iota" => Some(0x03B9),
        "kappa" => Some(0x03BA),
        "lambda" => Some(0x03BB),
        "mu" => Some(0x03BC),
        "nu" => Some(0x03BD),
        "xi" => Some(0x03BE),
        "omicron" => Some(0x03BF),
        "pi" => Some(0x03C0),
        "rho" => Some(0x03C1),
        "sigmaf" => Some(0x03C2),
        "sigma" => Some(0x03C3),
        "tau" => Some(0x03C4),
        "upsilon" => Some(0x03C5),
        "phi" => Some(0x03C6),
        "chi" => Some(0x03C7),
        "psi" => Some(0x03C8),
        "omega" => Some(0x03C9),
        "thetasym" => Some(0x03D1),
        "upsih" => Some(0x03D2),
        "piv" => Some(0x03D6),
        "ensp" => Some(0x2002),
        "emsp" => Some(0x2003),
        "thinsp" => Some(0x2009),
        "zwnj" => Some(0x200C),
        "zwj" => Some(0x200D),
        "lrm" => Some(0x200E),
        "rlm" => Some(0x200F),
        "ndash" => Some(0x2013),
        "mdash" => Some(0x2014),
        "lsquo" => Some(0x2018),
        "rsquo" => Some(0x2019),
        "sbquo" => Some(0x201A),
        "ldquo" => Some(0x201C),
        "rdquo" => Some(0x201D),
        "bdquo" => Some(0x201E),
        "dagger" => Some(0x2020),
        "Dagger" => Some(0x2021),
        "bull" => Some(0x2022),
        "hellip" => Some(0x2026),
        "permil" => Some(0x2030),
        "prime" => Some(0x2032),
        "Prime" => Some(0x2033),
        "lsaquo" => Some(0x2039),
        "rsaquo" => Some(0x203A),
        "oline" => Some(0x203E),
        "frasl" => Some(0x2044),
        "euro" => Some(0x20AC),
        "image" => Some(0x2111),
        "weierp" => Some(0x2118),
        "real" => Some(0x211C),
        "trade" => Some(0x2122),
        "alefsym" => Some(0x2135),
        "larr" => Some(0x2190),
        "uarr" => Some(0x2191),
        "rarr" => Some(0x2192),
        "darr" => Some(0x2193),
        "harr" => Some(0x2194),
        "crarr" => Some(0x21B5),
        "lArr" => Some(0x21D0),
        "uArr" => Some(0x21D1),
        "rArr" => Some(0x21D2),
        "dArr" => Some(0x21D3),
        "hArr" => Some(0x21D4),
        "forall" => Some(0x2200),
        "part" => Some(0x2202),
        "exist" => Some(0x2203),
        "empty" => Some(0x2205),
        "nabla" => Some(0x2207),
        "isin" => Some(0x2208),
        "notin" => Some(0x2209),
        "ni" => Some(0x220B),
        "prod" => Some(0x220F),
        "sum" => Some(0x2211),
        "minus" => Some(0x2212),
        "lowast" => Some(0x2217),
        "radic" => Some(0x221A),
        "prop" => Some(0x221D),
        "infin" => Some(0x221E),
        "ang" => Some(0x2220),
        "and" => Some(0x2227),
        "or" => Some(0x2228),
        "cap" => Some(0x2229),
        "cup" => Some(0x222A),
        "int" => Some(0x222B),
        "there4" => Some(0x2234),
        "sim" => Some(0x223C),
        "cong" => Some(0x2245),
        "asymp" => Some(0x2248),
        "ne" => Some(0x2260),
        "equiv" => Some(0x2261),
        "le" => Some(0x2264),
        "ge" => Some(0x2265),
        "sub" => Some(0x2282),
        "sup" => Some(0x2283),
        "nsub" => Some(0x2284),
        "sube" => Some(0x2286),
        "supe" => Some(0x2287),
        "oplus" => Some(0x2295),
        "otimes" => Some(0x2297),
        "perp" => Some(0x22A5),
        "sdot" => Some(0x22C5),
        "lceil" => Some(0x2308),
        "rceil" => Some(0x2309),
        "lfloor" => Some(0x230A),
        "rfloor" => Some(0x230B),
        "lang" => Some(0x2329),
        "rang" => Some(0x232A),
        "loz" => Some(0x25CA),
        "spades" => Some(0x2660),
        "clubs" => Some(0x2663),
        "hearts" => Some(0x2665),
        "diams" => Some(0x2666),
        _ => None,
    }
    .and_then(|u| char::try_from(u).ok())
}

pub trait RangeTable {
    fn latin_offset() -> usize;
    fn r16() -> &'static [RangeInclusive<u16>];
    fn r32() -> &'static [RangeInclusive<u32>];
}

pub struct IdStart;

impl RangeTable for IdStart {
    fn latin_offset() -> usize {
        117
    }

    fn r16() -> &'static [RangeInclusive<u16>] {
        &[
            0x0041..=0x005A, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            0x0061..=0x007A, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            0x00AA..=0x00AA, // Lo       FEMININE ORDINAL INDICATOR
            0x00B5..=0x00B5, // L&       MICRO SIGN
            0x00BA..=0x00BA, // Lo       MASCULINE ORDINAL INDICATOR
            0x00C0..=0x00D6, // L&  [23] LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
            0x00D8..=0x00F6, // L&  [31] LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
            0x00F8..=0x01BA, // L& [195] LATIN SMALL LETTER O WITH STROKE..LATIN SMALL LETTER EZH WITH TAIL
            0x01BB..=0x01BB, // Lo       LATIN LETTER TWO WITH STROKE
            0x01BC..=0x01BF, // L&   [4] LATIN CAPITAL LETTER TONE FIVE..LATIN LETTER WYNN
            0x01C0..=0x01C3, // Lo   [4] LATIN LETTER DENTAL CLICK..LATIN LETTER RETROFLEX CLICK
            0x01C4..=0x0293, // L& [208] LATIN CAPITAL LETTER DZ WITH CARON..LATIN SMALL LETTER EZH WITH CURL
            0x0294..=0x0294, // Lo       LATIN LETTER GLOTTAL STOP
            0x0295..=0x02AF, // L&  [27] LATIN LETTER PHARYNGEAL VOICED FRICATIVE..LATIN SMALL LETTER TURNED H WITH FISHHOOK AND TAIL
            0x02B0..=0x02C1, // Lm  [18] MODIFIER LETTER SMALL H..MODIFIER LETTER REVERSED GLOTTAL STOP
            0x02C6..=0x02D1, // Lm  [12] MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
            0x02E0..=0x02E4, // Lm   [5] MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
            0x02EC..=0x02EC, // Lm       MODIFIER LETTER VOICING
            0x02EE..=0x02EE, // Lm       MODIFIER LETTER DOUBLE APOSTROPHE
            0x0370..=0x0373, // L&   [4] GREEK CAPITAL LETTER HETA..GREEK SMALL LETTER ARCHAIC SAMPI
            0x0374..=0x0374, // Lm       GREEK NUMERAL SIGN
            0x0376..=0x0377, // L&   [2] GREEK CAPITAL LETTER PAMPHYLIAN DIGAMMA..GREEK SMALL LETTER PAMPHYLIAN DIGAMMA
            0x037A..=0x037A, // Lm       GREEK YPOGEGRAMMENI
            0x037B..=0x037D, // L&   [3] GREEK SMALL REVERSED LUNATE SIGMA SYMBOL..GREEK SMALL REVERSED DOTTED LUNATE SIGMA SYMBOL
            0x037F..=0x037F, // L&       GREEK CAPITAL LETTER YOT
            0x0386..=0x0386, // L&       GREEK CAPITAL LETTER ALPHA WITH TONOS
            0x0388..=0x038A, // L&   [3] GREEK CAPITAL LETTER EPSILON WITH TONOS..GREEK CAPITAL LETTER IOTA WITH TONOS
            0x038C..=0x038C, // L&       GREEK CAPITAL LETTER OMICRON WITH TONOS
            0x038E..=0x03A1, // L&  [20] GREEK CAPITAL LETTER UPSILON WITH TONOS..GREEK CAPITAL LETTER RHO
            0x03A3..=0x03F5, // L&  [83] GREEK CAPITAL LETTER SIGMA..GREEK LUNATE EPSILON SYMBOL
            0x03F7..=0x0481, // L& [139] GREEK CAPITAL LETTER SHO..CYRILLIC SMALL LETTER KOPPA
            0x048A..=0x052F, // L& [166] CYRILLIC CAPITAL LETTER SHORT I WITH TAIL..CYRILLIC SMALL LETTER EL WITH DESCENDER
            0x0531..=0x0556, // L&  [38] ARMENIAN CAPITAL LETTER AYB..ARMENIAN CAPITAL LETTER FEH
            0x0559..=0x0559, // Lm       ARMENIAN MODIFIER LETTER LEFT HALF RING
            0x0561..=0x0587, // L&  [39] ARMENIAN SMALL LETTER AYB..ARMENIAN SMALL LIGATURE ECH YIWN
            0x05D0..=0x05EA, // Lo  [27] HEBREW LETTER ALEF..HEBREW LETTER TAV
            0x05F0..=0x05F2, // Lo   [3] HEBREW LIGATURE YIDDISH DOUBLE VAV..HEBREW LIGATURE YIDDISH DOUBLE YOD
            0x0620..=0x063F, // Lo  [32] ARABIC LETTER KASHMIRI YEH..ARABIC LETTER FARSI YEH WITH THREE DOTS ABOVE
            0x0640..=0x0640, // Lm       ARABIC TATWEEL
            0x0641..=0x064A, // Lo  [10] ARABIC LETTER FEH..ARABIC LETTER YEH
            0x066E..=0x066F, // Lo   [2] ARABIC LETTER DOTLESS BEH..ARABIC LETTER DOTLESS QAF
            0x0671..=0x06D3, // Lo  [99] ARABIC LETTER ALEF WASLA..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
            0x06D5..=0x06D5, // Lo       ARABIC LETTER AE
            0x06E5..=0x06E6, // Lm   [2] ARABIC SMALL WAW..ARABIC SMALL YEH
            0x06EE..=0x06EF, // Lo   [2] ARABIC LETTER DAL WITH INVERTED V..ARABIC LETTER REH WITH INVERTED V
            0x06FA..=0x06FC, // Lo   [3] ARABIC LETTER SHEEN WITH DOT BELOW..ARABIC LETTER GHAIN WITH DOT BELOW
            0x06FF..=0x06FF, // Lo       ARABIC LETTER HEH WITH INVERTED V
            0x0710..=0x0710, // Lo       SYRIAC LETTER ALAPH
            0x0712..=0x072F, // Lo  [30] SYRIAC LETTER BETH..SYRIAC LETTER PERSIAN DHALATH
            0x074D..=0x07A5, // Lo  [89] SYRIAC LETTER SOGDIAN ZHAIN..THAANA LETTER WAAVU
            0x07B1..=0x07B1, // Lo       THAANA LETTER NAA
            0x07CA..=0x07EA, // Lo  [33] NKO LETTER A..NKO LETTER JONA RA
            0x07F4..=0x07F5, // Lm   [2] NKO HIGH TONE APOSTROPHE..NKO LOW TONE APOSTROPHE
            0x07FA..=0x07FA, // Lm       NKO LAJANYALAN
            0x0800..=0x0815, // Lo  [22] SAMARITAN LETTER ALAF..SAMARITAN LETTER TAAF
            0x081A..=0x081A, // Lm       SAMARITAN MODIFIER LETTER EPENTHETIC YUT
            0x0824..=0x0824, // Lm       SAMARITAN MODIFIER LETTER SHORT A
            0x0828..=0x0828, // Lm       SAMARITAN MODIFIER LETTER I
            0x0840..=0x0858, // Lo  [25] MANDAIC LETTER HALQA..MANDAIC LETTER AIN
            0x08A0..=0x08B4, // Lo  [21] ARABIC LETTER BEH WITH SMALL V BELOW..ARABIC LETTER KAF WITH DOT BELOW
            0x08B6..=0x08BD, // Lo   [8] ARABIC LETTER BEH WITH SMALL MEEM ABOVE..ARABIC LETTER AFRICAN NOON
            0x0904..=0x0939, // Lo  [54] DEVANAGARI LETTER SHORT A..DEVANAGARI LETTER HA
            0x093D..=0x093D, // Lo       DEVANAGARI SIGN AVAGRAHA
            0x0950..=0x0950, // Lo       DEVANAGARI OM
            0x0958..=0x0961, // Lo  [10] DEVANAGARI LETTER QA..DEVANAGARI LETTER VOCALIC LL
            0x0971..=0x0971, // Lm       DEVANAGARI SIGN HIGH SPACING DOT
            0x0972..=0x0980, // Lo  [15] DEVANAGARI LETTER CANDRA A..BENGALI ANJI
            0x0985..=0x098C, // Lo   [8] BENGALI LETTER A..BENGALI LETTER VOCALIC L
            0x098F..=0x0990, // Lo   [2] BENGALI LETTER E..BENGALI LETTER AI
            0x0993..=0x09A8, // Lo  [22] BENGALI LETTER O..BENGALI LETTER NA
            0x09AA..=0x09B0, // Lo   [7] BENGALI LETTER PA..BENGALI LETTER RA
            0x09B2..=0x09B2, // Lo       BENGALI LETTER LA
            0x09B6..=0x09B9, // Lo   [4] BENGALI LETTER SHA..BENGALI LETTER HA
            0x09BD..=0x09BD, // Lo       BENGALI SIGN AVAGRAHA
            0x09CE..=0x09CE, // Lo       BENGALI LETTER KHANDA TA
            0x09DC..=0x09DD, // Lo   [2] BENGALI LETTER RRA..BENGALI LETTER RHA
            0x09DF..=0x09E1, // Lo   [3] BENGALI LETTER YYA..BENGALI LETTER VOCALIC LL
            0x09F0..=0x09F1, // Lo   [2] BENGALI LETTER RA WITH MIDDLE DIAGONAL..BENGALI LETTER RA WITH LOWER DIAGONAL
            0x0A05..=0x0A0A, // Lo   [6] GURMUKHI LETTER A..GURMUKHI LETTER UU
            0x0A0F..=0x0A10, // Lo   [2] GURMUKHI LETTER EE..GURMUKHI LETTER AI
            0x0A13..=0x0A28, // Lo  [22] GURMUKHI LETTER OO..GURMUKHI LETTER NA
            0x0A2A..=0x0A30, // Lo   [7] GURMUKHI LETTER PA..GURMUKHI LETTER RA
            0x0A32..=0x0A33, // Lo   [2] GURMUKHI LETTER LA..GURMUKHI LETTER LLA
            0x0A35..=0x0A36, // Lo   [2] GURMUKHI LETTER VA..GURMUKHI LETTER SHA
            0x0A38..=0x0A39, // Lo   [2] GURMUKHI LETTER SA..GURMUKHI LETTER HA
            0x0A59..=0x0A5C, // Lo   [4] GURMUKHI LETTER KHHA..GURMUKHI LETTER RRA
            0x0A5E..=0x0A5E, // Lo       GURMUKHI LETTER FA
            0x0A72..=0x0A74, // Lo   [3] GURMUKHI IRI..GURMUKHI EK ONKAR
            0x0A85..=0x0A8D, // Lo   [9] GUJARATI LETTER A..GUJARATI VOWEL CANDRA E
            0x0A8F..=0x0A91, // Lo   [3] GUJARATI LETTER E..GUJARATI VOWEL CANDRA O
            0x0A93..=0x0AA8, // Lo  [22] GUJARATI LETTER O..GUJARATI LETTER NA
            0x0AAA..=0x0AB0, // Lo   [7] GUJARATI LETTER PA..GUJARATI LETTER RA
            0x0AB2..=0x0AB3, // Lo   [2] GUJARATI LETTER LA..GUJARATI LETTER LLA
            0x0AB5..=0x0AB9, // Lo   [5] GUJARATI LETTER VA..GUJARATI LETTER HA
            0x0ABD..=0x0ABD, // Lo       GUJARATI SIGN AVAGRAHA
            0x0AD0..=0x0AD0, // Lo       GUJARATI OM
            0x0AE0..=0x0AE1, // Lo   [2] GUJARATI LETTER VOCALIC RR..GUJARATI LETTER VOCALIC LL
            0x0AF9..=0x0AF9, // Lo       GUJARATI LETTER ZHA
            0x0B05..=0x0B0C, // Lo   [8] ORIYA LETTER A..ORIYA LETTER VOCALIC L
            0x0B0F..=0x0B10, // Lo   [2] ORIYA LETTER E..ORIYA LETTER AI
            0x0B13..=0x0B28, // Lo  [22] ORIYA LETTER O..ORIYA LETTER NA
            0x0B2A..=0x0B30, // Lo   [7] ORIYA LETTER PA..ORIYA LETTER RA
            0x0B32..=0x0B33, // Lo   [2] ORIYA LETTER LA..ORIYA LETTER LLA
            0x0B35..=0x0B39, // Lo   [5] ORIYA LETTER VA..ORIYA LETTER HA
            0x0B3D..=0x0B3D, // Lo       ORIYA SIGN AVAGRAHA
            0x0B5C..=0x0B5D, // Lo   [2] ORIYA LETTER RRA..ORIYA LETTER RHA
            0x0B5F..=0x0B61, // Lo   [3] ORIYA LETTER YYA..ORIYA LETTER VOCALIC LL
            0x0B71..=0x0B71, // Lo       ORIYA LETTER WA
            0x0B83..=0x0B83, // Lo       TAMIL SIGN VISARGA
            0x0B85..=0x0B8A, // Lo   [6] TAMIL LETTER A..TAMIL LETTER UU
            0x0B8E..=0x0B90, // Lo   [3] TAMIL LETTER E..TAMIL LETTER AI
            0x0B92..=0x0B95, // Lo   [4] TAMIL LETTER O..TAMIL LETTER KA
            0x0B99..=0x0B9A, // Lo   [2] TAMIL LETTER NGA..TAMIL LETTER CA
            0x0B9C..=0x0B9C, // Lo       TAMIL LETTER JA
            0x0B9E..=0x0B9F, // Lo   [2] TAMIL LETTER NYA..TAMIL LETTER TTA
            0x0BA3..=0x0BA4, // Lo   [2] TAMIL LETTER NNA..TAMIL LETTER TA
            0x0BA8..=0x0BAA, // Lo   [3] TAMIL LETTER NA..TAMIL LETTER PA
            0x0BAE..=0x0BB9, // Lo  [12] TAMIL LETTER MA..TAMIL LETTER HA
            0x0BD0..=0x0BD0, // Lo       TAMIL OM
            0x0C05..=0x0C0C, // Lo   [8] TELUGU LETTER A..TELUGU LETTER VOCALIC L
            0x0C0E..=0x0C10, // Lo   [3] TELUGU LETTER E..TELUGU LETTER AI
            0x0C12..=0x0C28, // Lo  [23] TELUGU LETTER O..TELUGU LETTER NA
            0x0C2A..=0x0C39, // Lo  [16] TELUGU LETTER PA..TELUGU LETTER HA
            0x0C3D..=0x0C3D, // Lo       TELUGU SIGN AVAGRAHA
            0x0C58..=0x0C5A, // Lo   [3] TELUGU LETTER TSA..TELUGU LETTER RRRA
            0x0C60..=0x0C61, // Lo   [2] TELUGU LETTER VOCALIC RR..TELUGU LETTER VOCALIC LL
            0x0C80..=0x0C80, // Lo       KANNADA SIGN SPACING CANDRABINDU
            0x0C85..=0x0C8C, // Lo   [8] KANNADA LETTER A..KANNADA LETTER VOCALIC L
            0x0C8E..=0x0C90, // Lo   [3] KANNADA LETTER E..KANNADA LETTER AI
            0x0C92..=0x0CA8, // Lo  [23] KANNADA LETTER O..KANNADA LETTER NA
            0x0CAA..=0x0CB3, // Lo  [10] KANNADA LETTER PA..KANNADA LETTER LLA
            0x0CB5..=0x0CB9, // Lo   [5] KANNADA LETTER VA..KANNADA LETTER HA
            0x0CBD..=0x0CBD, // Lo       KANNADA SIGN AVAGRAHA
            0x0CDE..=0x0CDE, // Lo       KANNADA LETTER FA
            0x0CE0..=0x0CE1, // Lo   [2] KANNADA LETTER VOCALIC RR..KANNADA LETTER VOCALIC LL
            0x0CF1..=0x0CF2, // Lo   [2] KANNADA SIGN JIHVAMULIYA..KANNADA SIGN UPADHMANIYA
            0x0D05..=0x0D0C, // Lo   [8] MALAYALAM LETTER A..MALAYALAM LETTER VOCALIC L
            0x0D0E..=0x0D10, // Lo   [3] MALAYALAM LETTER E..MALAYALAM LETTER AI
            0x0D12..=0x0D3A, // Lo  [41] MALAYALAM LETTER O..MALAYALAM LETTER TTTA
            0x0D3D..=0x0D3D, // Lo       MALAYALAM SIGN AVAGRAHA
            0x0D4E..=0x0D4E, // Lo       MALAYALAM LETTER DOT REPH
            0x0D54..=0x0D56, // Lo   [3] MALAYALAM LETTER CHILLU M..MALAYALAM LETTER CHILLU LLL
            0x0D5F..=0x0D61, // Lo   [3] MALAYALAM LETTER ARCHAIC II..MALAYALAM LETTER VOCALIC LL
            0x0D7A..=0x0D7F, // Lo   [6] MALAYALAM LETTER CHILLU NN..MALAYALAM LETTER CHILLU K
            0x0D85..=0x0D96, // Lo  [18] SINHALA LETTER AYANNA..SINHALA LETTER AUYANNA
            0x0D9A..=0x0DB1, // Lo  [24] SINHALA LETTER ALPAPRAANA KAYANNA..SINHALA LETTER DANTAJA NAYANNA
            0x0DB3..=0x0DBB, // Lo   [9] SINHALA LETTER SANYAKA DAYANNA..SINHALA LETTER RAYANNA
            0x0DBD..=0x0DBD, // Lo       SINHALA LETTER DANTAJA LAYANNA
            0x0DC0..=0x0DC6, // Lo   [7] SINHALA LETTER VAYANNA..SINHALA LETTER FAYANNA
            0x0E01..=0x0E30, // Lo  [48] THAI CHARACTER KO KAI..THAI CHARACTER SARA A
            0x0E32..=0x0E33, // Lo   [2] THAI CHARACTER SARA AA..THAI CHARACTER SARA AM
            0x0E40..=0x0E45, // Lo   [6] THAI CHARACTER SARA E..THAI CHARACTER LAKKHANGYAO
            0x0E46..=0x0E46, // Lm       THAI CHARACTER MAIYAMOK
            0x0E81..=0x0E82, // Lo   [2] LAO LETTER KO..LAO LETTER KHO SUNG
            0x0E84..=0x0E84, // Lo       LAO LETTER KHO TAM
            0x0E87..=0x0E88, // Lo   [2] LAO LETTER NGO..LAO LETTER CO
            0x0E8A..=0x0E8A, // Lo       LAO LETTER SO TAM
            0x0E8D..=0x0E8D, // Lo       LAO LETTER NYO
            0x0E94..=0x0E97, // Lo   [4] LAO LETTER DO..LAO LETTER THO TAM
            0x0E99..=0x0E9F, // Lo   [7] LAO LETTER NO..LAO LETTER FO SUNG
            0x0EA1..=0x0EA3, // Lo   [3] LAO LETTER MO..LAO LETTER LO LING
            0x0EA5..=0x0EA5, // Lo       LAO LETTER LO LOOT
            0x0EA7..=0x0EA7, // Lo       LAO LETTER WO
            0x0EAA..=0x0EAB, // Lo   [2] LAO LETTER SO SUNG..LAO LETTER HO SUNG
            0x0EAD..=0x0EB0, // Lo   [4] LAO LETTER O..LAO VOWEL SIGN A
            0x0EB2..=0x0EB3, // Lo   [2] LAO VOWEL SIGN AA..LAO VOWEL SIGN AM
            0x0EBD..=0x0EBD, // Lo       LAO SEMIVOWEL SIGN NYO
            0x0EC0..=0x0EC4, // Lo   [5] LAO VOWEL SIGN E..LAO VOWEL SIGN AI
            0x0EC6..=0x0EC6, // Lm       LAO KO LA
            0x0EDC..=0x0EDF, // Lo   [4] LAO HO NO..LAO LETTER KHMU NYO
            0x0F00..=0x0F00, // Lo       TIBETAN SYLLABLE OM
            0x0F40..=0x0F47, // Lo   [8] TIBETAN LETTER KA..TIBETAN LETTER JA
            0x0F49..=0x0F6C, // Lo  [36] TIBETAN LETTER NYA..TIBETAN LETTER RRA
            0x0F88..=0x0F8C, // Lo   [5] TIBETAN SIGN LCE TSA CAN..TIBETAN SIGN INVERTED MCHU CAN
            0x1000..=0x102A, // Lo  [43] MYANMAR LETTER KA..MYANMAR LETTER AU
            0x103F..=0x103F, // Lo       MYANMAR LETTER GREAT SA
            0x1050..=0x1055, // Lo   [6] MYANMAR LETTER SHA..MYANMAR LETTER VOCALIC LL
            0x105A..=0x105D, // Lo   [4] MYANMAR LETTER MON NGA..MYANMAR LETTER MON BBE
            0x1061..=0x1061, // Lo       MYANMAR LETTER SGAW KAREN SHA
            0x1065..=0x1066, // Lo   [2] MYANMAR LETTER WESTERN PWO KAREN THA..MYANMAR LETTER WESTERN PWO KAREN PWA
            0x106E..=0x1070, // Lo   [3] MYANMAR LETTER EASTERN PWO KAREN NNA..MYANMAR LETTER EASTERN PWO KAREN GHWA
            0x1075..=0x1081, // Lo  [13] MYANMAR LETTER SHAN KA..MYANMAR LETTER SHAN HA
            0x108E..=0x108E, // Lo       MYANMAR LETTER RUMAI PALAUNG FA
            0x10A0..=0x10C5, // L&  [38] GEORGIAN CAPITAL LETTER AN..GEORGIAN CAPITAL LETTER HOE
            0x10C7..=0x10C7, // L&       GEORGIAN CAPITAL LETTER YN
            0x10CD..=0x10CD, // L&       GEORGIAN CAPITAL LETTER AEN
            0x10D0..=0x10FA, // Lo  [43] GEORGIAN LETTER AN..GEORGIAN LETTER AIN
            0x10FC..=0x10FC, // Lm       MODIFIER LETTER GEORGIAN NAR
            0x10FD..=0x1248, // Lo [332] GEORGIAN LETTER AEN..ETHIOPIC SYLLABLE QWA
            0x124A..=0x124D, // Lo   [4] ETHIOPIC SYLLABLE QWI..ETHIOPIC SYLLABLE QWE
            0x1250..=0x1256, // Lo   [7] ETHIOPIC SYLLABLE QHA..ETHIOPIC SYLLABLE QHO
            0x1258..=0x1258, // Lo       ETHIOPIC SYLLABLE QHWA
            0x125A..=0x125D, // Lo   [4] ETHIOPIC SYLLABLE QHWI..ETHIOPIC SYLLABLE QHWE
            0x1260..=0x1288, // Lo  [41] ETHIOPIC SYLLABLE BA..ETHIOPIC SYLLABLE XWA
            0x128A..=0x128D, // Lo   [4] ETHIOPIC SYLLABLE XWI..ETHIOPIC SYLLABLE XWE
            0x1290..=0x12B0, // Lo  [33] ETHIOPIC SYLLABLE NA..ETHIOPIC SYLLABLE KWA
            0x12B2..=0x12B5, // Lo   [4] ETHIOPIC SYLLABLE KWI..ETHIOPIC SYLLABLE KWE
            0x12B8..=0x12BE, // Lo   [7] ETHIOPIC SYLLABLE KXA..ETHIOPIC SYLLABLE KXO
            0x12C0..=0x12C0, // Lo       ETHIOPIC SYLLABLE KXWA
            0x12C2..=0x12C5, // Lo   [4] ETHIOPIC SYLLABLE KXWI..ETHIOPIC SYLLABLE KXWE
            0x12C8..=0x12D6, // Lo  [15] ETHIOPIC SYLLABLE WA..ETHIOPIC SYLLABLE PHARYNGEAL O
            0x12D8..=0x1310, // Lo  [57] ETHIOPIC SYLLABLE ZA..ETHIOPIC SYLLABLE GWA
            0x1312..=0x1315, // Lo   [4] ETHIOPIC SYLLABLE GWI..ETHIOPIC SYLLABLE GWE
            0x1318..=0x135A, // Lo  [67] ETHIOPIC SYLLABLE GGA..ETHIOPIC SYLLABLE FYA
            0x1380..=0x138F, // Lo  [16] ETHIOPIC SYLLABLE SEBATBEIT MWA..ETHIOPIC SYLLABLE PWE
            0x13A0..=0x13F5, // L&  [86] CHEROKEE LETTER A..CHEROKEE LETTER MV
            0x13F8..=0x13FD, // L&   [6] CHEROKEE SMALL LETTER YE..CHEROKEE SMALL LETTER MV
            0x1401..=0x166C, // Lo [620] CANADIAN SYLLABICS E..CANADIAN SYLLABICS CARRIER TTSA
            0x166F..=0x167F, // Lo  [17] CANADIAN SYLLABICS QAI..CANADIAN SYLLABICS BLACKFOOT W
            0x1681..=0x169A, // Lo  [26] OGHAM LETTER BEITH..OGHAM LETTER PEITH
            0x16A0..=0x16EA, // Lo  [75] RUNIC LETTER FEHU FEOH FE F..RUNIC LETTER X
            0x16EE..=0x16F0, // Nl   [3] RUNIC ARLAUG SYMBOL..RUNIC BELGTHOR SYMBOL
            0x16F1..=0x16F8, // Lo   [8] RUNIC LETTER K..RUNIC LETTER FRANKS CASKET AESC
            0x1700..=0x170C, // Lo  [13] TAGALOG LETTER A..TAGALOG LETTER YA
            0x170E..=0x1711, // Lo   [4] TAGALOG LETTER LA..TAGALOG LETTER HA
            0x1720..=0x1731, // Lo  [18] HANUNOO LETTER A..HANUNOO LETTER HA
            0x1740..=0x1751, // Lo  [18] BUHID LETTER A..BUHID LETTER HA
            0x1760..=0x176C, // Lo  [13] TAGBANWA LETTER A..TAGBANWA LETTER YA
            0x176E..=0x1770, // Lo   [3] TAGBANWA LETTER LA..TAGBANWA LETTER SA
            0x1780..=0x17B3, // Lo  [52] KHMER LETTER KA..KHMER INDEPENDENT VOWEL QAU
            0x17D7..=0x17D7, // Lm       KHMER SIGN LEK TOO
            0x17DC..=0x17DC, // Lo       KHMER SIGN AVAKRAHASANYA
            0x1820..=0x1842, // Lo  [35] MONGOLIAN LETTER A..MONGOLIAN LETTER CHI
            0x1843..=0x1843, // Lm       MONGOLIAN LETTER TODO LONG VOWEL SIGN
            0x1844..=0x1877, // Lo  [52] MONGOLIAN LETTER TODO E..MONGOLIAN LETTER MANCHU ZHA
            0x1880..=0x1884, // Lo   [5] MONGOLIAN LETTER ALI GALI ANUSVARA ONE..MONGOLIAN LETTER ALI GALI INVERTED UBADAMA
            0x1885..=0x1886, // Mn   [2] MONGOLIAN LETTER ALI GALI BALUDA..MONGOLIAN LETTER ALI GALI THREE BALUDA
            0x1887..=0x18A8, // Lo  [34] MONGOLIAN LETTER ALI GALI A..MONGOLIAN LETTER MANCHU ALI GALI BHA
            0x18AA..=0x18AA, // Lo       MONGOLIAN LETTER MANCHU ALI GALI LHA
            0x18B0..=0x18F5, // Lo  [70] CANADIAN SYLLABICS OY..CANADIAN SYLLABICS CARRIER DENTAL S
            0x1900..=0x191E, // Lo  [31] LIMBU VOWEL-CARRIER LETTER..LIMBU LETTER TRA
            0x1950..=0x196D, // Lo  [30] TAI LE LETTER KA..TAI LE LETTER AI
            0x1970..=0x1974, // Lo   [5] TAI LE LETTER TONE-2..TAI LE LETTER TONE-6
            0x1980..=0x19AB, // Lo  [44] NEW TAI LUE LETTER HIGH QA..NEW TAI LUE LETTER LOW SUA
            0x19B0..=0x19C9, // Lo  [26] NEW TAI LUE VOWEL SIGN VOWEL SHORTENER..NEW TAI LUE TONE MARK-2
            0x1A00..=0x1A16, // Lo  [23] BUGINESE LETTER KA..BUGINESE LETTER HA
            0x1A20..=0x1A54, // Lo  [53] TAI THAM LETTER HIGH KA..TAI THAM LETTER GREAT SA
            0x1AA7..=0x1AA7, // Lm       TAI THAM SIGN MAI YAMOK
            0x1B05..=0x1B33, // Lo  [47] BALINESE LETTER AKARA..BALINESE LETTER HA
            0x1B45..=0x1B4B, // Lo   [7] BALINESE LETTER KAF SASAK..BALINESE LETTER ASYURA SASAK
            0x1B83..=0x1BA0, // Lo  [30] SUNDANESE LETTER A..SUNDANESE LETTER HA
            0x1BAE..=0x1BAF, // Lo   [2] SUNDANESE LETTER KHA..SUNDANESE LETTER SYA
            0x1BBA..=0x1BE5, // Lo  [44] SUNDANESE AVAGRAHA..BATAK LETTER U
            0x1C00..=0x1C23, // Lo  [36] LEPCHA LETTER KA..LEPCHA LETTER A
            0x1C4D..=0x1C4F, // Lo   [3] LEPCHA LETTER TTA..LEPCHA LETTER DDA
            0x1C5A..=0x1C77, // Lo  [30] OL CHIKI LETTER LA..OL CHIKI LETTER OH
            0x1C78..=0x1C7D, // Lm   [6] OL CHIKI MU TTUDDAG..OL CHIKI AHAD
            0x1C80..=0x1C88, // L&   [9] CYRILLIC SMALL LETTER ROUNDED VE..CYRILLIC SMALL LETTER UNBLENDED UK
            0x1CE9..=0x1CEC, // Lo   [4] VEDIC SIGN ANUSVARA ANTARGOMUKHA..VEDIC SIGN ANUSVARA VAMAGOMUKHA WITH TAIL
            0x1CEE..=0x1CF1, // Lo   [4] VEDIC SIGN HEXIFORM LONG ANUSVARA..VEDIC SIGN ANUSVARA UBHAYATO MUKHA
            0x1CF5..=0x1CF6, // Lo   [2] VEDIC SIGN JIHVAMULIYA..VEDIC SIGN UPADHMANIYA
            0x1D00..=0x1D2B, // L&  [44] LATIN LETTER SMALL CAPITAL A..CYRILLIC LETTER SMALL CAPITAL EL
            0x1D2C..=0x1D6A, // Lm  [63] MODIFIER LETTER CAPITAL A..GREEK SUBSCRIPT SMALL LETTER CHI
            0x1D6B..=0x1D77, // L&  [13] LATIN SMALL LETTER UE..LATIN SMALL LETTER TURNED G
            0x1D78..=0x1D78, // Lm       MODIFIER LETTER CYRILLIC EN
            0x1D79..=0x1D9A, // L&  [34] LATIN SMALL LETTER INSULAR G..LATIN SMALL LETTER EZH WITH RETROFLEX HOOK
            0x1D9B..=0x1DBF, // Lm  [37] MODIFIER LETTER SMALL TURNED ALPHA..MODIFIER LETTER SMALL THETA
            0x1E00..=0x1F15, // L& [278] LATIN CAPITAL LETTER A WITH RING BELOW..GREEK SMALL LETTER EPSILON WITH DASIA AND OXIA
            0x1F18..=0x1F1D, // L&   [6] GREEK CAPITAL LETTER EPSILON WITH PSILI..GREEK CAPITAL LETTER EPSILON WITH DASIA AND OXIA
            0x1F20..=0x1F45, // L&  [38] GREEK SMALL LETTER ETA WITH PSILI..GREEK SMALL LETTER OMICRON WITH DASIA AND OXIA
            0x1F48..=0x1F4D, // L&   [6] GREEK CAPITAL LETTER OMICRON WITH PSILI..GREEK CAPITAL LETTER OMICRON WITH DASIA AND OXIA
            0x1F50..=0x1F57, // L&   [8] GREEK SMALL LETTER UPSILON WITH PSILI..GREEK SMALL LETTER UPSILON WITH DASIA AND PERISPOMENI
            0x1F59..=0x1F59, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA
            0x1F5B..=0x1F5B, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND VARIA
            0x1F5D..=0x1F5D, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND OXIA
            0x1F5F..=0x1F7D, // L&  [31] GREEK CAPITAL LETTER UPSILON WITH DASIA AND PERISPOMENI..GREEK SMALL LETTER OMEGA WITH OXIA
            0x1F80..=0x1FB4, // L&  [53] GREEK SMALL LETTER ALPHA WITH PSILI AND YPOGEGRAMMENI..GREEK SMALL LETTER ALPHA WITH OXIA AND YPOGEGRAMMENI
            0x1FB6..=0x1FBC, // L&   [7] GREEK SMALL LETTER ALPHA WITH PERISPOMENI..GREEK CAPITAL LETTER ALPHA WITH PROSGEGRAMMENI
            0x1FBE..=0x1FBE, // L&       GREEK PROSGEGRAMMENI
            0x1FC2..=0x1FC4, // L&   [3] GREEK SMALL LETTER ETA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER ETA WITH OXIA AND YPOGEGRAMMENI
            0x1FC6..=0x1FCC, // L&   [7] GREEK SMALL LETTER ETA WITH PERISPOMENI..GREEK CAPITAL LETTER ETA WITH PROSGEGRAMMENI
            0x1FD0..=0x1FD3, // L&   [4] GREEK SMALL LETTER IOTA WITH VRACHY..GREEK SMALL LETTER IOTA WITH DIALYTIKA AND OXIA
            0x1FD6..=0x1FDB, // L&   [6] GREEK SMALL LETTER IOTA WITH PERISPOMENI..GREEK CAPITAL LETTER IOTA WITH OXIA
            0x1FE0..=0x1FEC, // L&  [13] GREEK SMALL LETTER UPSILON WITH VRACHY..GREEK CAPITAL LETTER RHO WITH DASIA
            0x1FF2..=0x1FF4, // L&   [3] GREEK SMALL LETTER OMEGA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER OMEGA WITH OXIA AND YPOGEGRAMMENI
            0x1FF6..=0x1FFC, // L&   [7] GREEK SMALL LETTER OMEGA WITH PERISPOMENI..GREEK CAPITAL LETTER OMEGA WITH PROSGEGRAMMENI
            0x2071..=0x2071, // Lm       SUPERSCRIPT LATIN SMALL LETTER I
            0x207F..=0x207F, // Lm       SUPERSCRIPT LATIN SMALL LETTER N
            0x2090..=0x209C, // Lm  [13] LATIN SUBSCRIPT SMALL LETTER A..LATIN SUBSCRIPT SMALL LETTER T
            0x2102..=0x2102, // L&       DOUBLE-STRUCK CAPITAL C
            0x2107..=0x2107, // L&       EULER CONSTANT
            0x210A..=0x2113, // L&  [10] SCRIPT SMALL G..SCRIPT SMALL L
            0x2115..=0x2115, // L&       DOUBLE-STRUCK CAPITAL N
            0x2118..=0x2118, // Sm       SCRIPT CAPITAL P
            0x2119..=0x211D, // L&   [5] DOUBLE-STRUCK CAPITAL P..DOUBLE-STRUCK CAPITAL R
            0x2124..=0x2124, // L&       DOUBLE-STRUCK CAPITAL Z
            0x2126..=0x2126, // L&       OHM SIGN
            0x2128..=0x2128, // L&       BLACK-LETTER CAPITAL Z
            0x212A..=0x212D, // L&   [4] KELVIN SIGN..BLACK-LETTER CAPITAL C
            0x212E..=0x212E, // So       ESTIMATED SYMBOL
            0x212F..=0x2134, // L&   [6] SCRIPT SMALL E..SCRIPT SMALL O
            0x2135..=0x2138, // Lo   [4] ALEF SYMBOL..DALET SYMBOL
            0x2139..=0x2139, // L&       INFORMATION SOURCE
            0x213C..=0x213F, // L&   [4] DOUBLE-STRUCK SMALL PI..DOUBLE-STRUCK CAPITAL PI
            0x2145..=0x2149, // L&   [5] DOUBLE-STRUCK ITALIC CAPITAL D..DOUBLE-STRUCK ITALIC SMALL J
            0x214E..=0x214E, // L&       TURNED SMALL F
            0x2160..=0x2182, // Nl  [35] ROMAN NUMERAL ONE..ROMAN NUMERAL TEN THOUSAND
            0x2183..=0x2184, // L&   [2] ROMAN NUMERAL REVERSED ONE HUNDRED..LATIN SMALL LETTER REVERSED C
            0x2185..=0x2188, // Nl   [4] ROMAN NUMERAL SIX LATE FORM..ROMAN NUMERAL ONE HUNDRED THOUSAND
            0x2C00..=0x2C2E, // L&  [47] GLAGOLITIC CAPITAL LETTER AZU..GLAGOLITIC CAPITAL LETTER LATINATE MYSLITE
            0x2C30..=0x2C5E, // L&  [47] GLAGOLITIC SMALL LETTER AZU..GLAGOLITIC SMALL LETTER LATINATE MYSLITE
            0x2C60..=0x2C7B, // L&  [28] LATIN CAPITAL LETTER L WITH DOUBLE BAR..LATIN LETTER SMALL CAPITAL TURNED E
            0x2C7C..=0x2C7D, // Lm   [2] LATIN SUBSCRIPT SMALL LETTER J..MODIFIER LETTER CAPITAL V
            0x2C7E..=0x2CE4, // L& [103] LATIN CAPITAL LETTER S WITH SWASH TAIL..COPTIC SYMBOL KAI
            0x2CEB..=0x2CEE, // L&   [4] COPTIC CAPITAL LETTER CRYPTOGRAMMIC SHEI..COPTIC SMALL LETTER CRYPTOGRAMMIC GANGIA
            0x2CF2..=0x2CF3, // L&   [2] COPTIC CAPITAL LETTER BOHAIRIC KHEI..COPTIC SMALL LETTER BOHAIRIC KHEI
            0x2D00..=0x2D25, // L&  [38] GEORGIAN SMALL LETTER AN..GEORGIAN SMALL LETTER HOE
            0x2D27..=0x2D27, // L&       GEORGIAN SMALL LETTER YN
            0x2D2D..=0x2D2D, // L&       GEORGIAN SMALL LETTER AEN
            0x2D30..=0x2D67, // Lo  [56] TIFINAGH LETTER YA..TIFINAGH LETTER YO
            0x2D6F..=0x2D6F, // Lm       TIFINAGH MODIFIER LETTER LABIALIZATION MARK
            0x2D80..=0x2D96, // Lo  [23] ETHIOPIC SYLLABLE LOA..ETHIOPIC SYLLABLE GGWE
            0x2DA0..=0x2DA6, // Lo   [7] ETHIOPIC SYLLABLE SSA..ETHIOPIC SYLLABLE SSO
            0x2DA8..=0x2DAE, // Lo   [7] ETHIOPIC SYLLABLE CCA..ETHIOPIC SYLLABLE CCO
            0x2DB0..=0x2DB6, // Lo   [7] ETHIOPIC SYLLABLE ZZA..ETHIOPIC SYLLABLE ZZO
            0x2DB8..=0x2DBE, // Lo   [7] ETHIOPIC SYLLABLE CCHA..ETHIOPIC SYLLABLE CCHO
            0x2DC0..=0x2DC6, // Lo   [7] ETHIOPIC SYLLABLE QYA..ETHIOPIC SYLLABLE QYO
            0x2DC8..=0x2DCE, // Lo   [7] ETHIOPIC SYLLABLE KYA..ETHIOPIC SYLLABLE KYO
            0x2DD0..=0x2DD6, // Lo   [7] ETHIOPIC SYLLABLE XYA..ETHIOPIC SYLLABLE XYO
            0x2DD8..=0x2DDE, // Lo   [7] ETHIOPIC SYLLABLE GYA..ETHIOPIC SYLLABLE GYO
            0x3005..=0x3005, // Lm       IDEOGRAPHIC ITERATION MARK
            0x3006..=0x3006, // Lo       IDEOGRAPHIC CLOSING MARK
            0x3007..=0x3007, // Nl       IDEOGRAPHIC NUMBER ZERO
            0x3021..=0x3029, // Nl   [9] HANGZHOU NUMERAL ONE..HANGZHOU NUMERAL NINE
            0x3031..=0x3035, // Lm   [5] VERTICAL KANA REPEAT MARK..VERTICAL KANA REPEAT MARK LOWER HALF
            0x3038..=0x303A, // Nl   [3] HANGZHOU NUMERAL TEN..HANGZHOU NUMERAL THIRTY
            0x303B..=0x303B, // Lm       VERTICAL IDEOGRAPHIC ITERATION MARK
            0x303C..=0x303C, // Lo       MASU MARK
            0x3041..=0x3096, // Lo  [86] HIRAGANA LETTER SMALL A..HIRAGANA LETTER SMALL KE
            0x309B..=0x309C, // Sk   [2] KATAKANA-HIRAGANA VOICED SOUND MARK..KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
            0x309D..=0x309E, // Lm   [2] HIRAGANA ITERATION MARK..HIRAGANA VOICED ITERATION MARK
            0x309F..=0x309F, // Lo       HIRAGANA DIGRAPH YORI
            0x30A1..=0x30FA, // Lo  [90] KATAKANA LETTER SMALL A..KATAKANA LETTER VO
            0x30FC..=0x30FE, // Lm   [3] KATAKANA-HIRAGANA PROLONGED SOUND MARK..KATAKANA VOICED ITERATION MARK
            0x30FF..=0x30FF, // Lo       KATAKANA DIGRAPH KOTO
            0x3105..=0x312D, // Lo  [41] BOPOMOFO LETTER B..BOPOMOFO LETTER IH
            0x3131..=0x318E, // Lo  [94] HANGUL LETTER KIYEOK..HANGUL LETTER ARAEAE
            0x31A0..=0x31BA, // Lo  [27] BOPOMOFO LETTER BU..BOPOMOFO LETTER ZY
            0x31F0..=0x31FF, // Lo  [16] KATAKANA LETTER SMALL KU..KATAKANA LETTER SMALL RO
            0x3400..=0x4DB5, // Lo [6582] CJK UNIFIED IDEOGRAPH-3400..CJK UNIFIED IDEOGRAPH-4DB5
            0x4E00..=0x9FD5, // Lo [20950] CJK UNIFIED IDEOGRAPH-4E00..CJK UNIFIED IDEOGRAPH-9FD5
            0xA000..=0xA014, // Lo  [21] YI SYLLABLE IT..YI SYLLABLE E
            0xA015..=0xA015, // Lm       YI SYLLABLE WU
            0xA016..=0xA48C, // Lo [1143] YI SYLLABLE BIT..YI SYLLABLE YYR
            0xA4D0..=0xA4F7, // Lo  [40] LISU LETTER BA..LISU LETTER OE
            0xA4F8..=0xA4FD, // Lm   [6] LISU LETTER TONE MYA TI..LISU LETTER TONE MYA JEU
            0xA500..=0xA60B, // Lo [268] VAI SYLLABLE EE..VAI SYLLABLE NG
            0xA60C..=0xA60C, // Lm       VAI SYLLABLE LENGTHENER
            0xA610..=0xA61F, // Lo  [16] VAI SYLLABLE NDOLE FA..VAI SYMBOL JONG
            0xA62A..=0xA62B, // Lo   [2] VAI SYLLABLE NDOLE MA..VAI SYLLABLE NDOLE DO
            0xA640..=0xA66D, // L&  [46] CYRILLIC CAPITAL LETTER ZEMLYA..CYRILLIC SMALL LETTER DOUBLE MONOCULAR O
            0xA66E..=0xA66E, // Lo       CYRILLIC LETTER MULTIOCULAR O
            0xA67F..=0xA67F, // Lm       CYRILLIC PAYEROK
            0xA680..=0xA69B, // L&  [28] CYRILLIC CAPITAL LETTER DWE..CYRILLIC SMALL LETTER CROSSED O
            0xA69C..=0xA69D, // Lm   [2] MODIFIER LETTER CYRILLIC HARD SIGN..MODIFIER LETTER CYRILLIC SOFT SIGN
            0xA6A0..=0xA6E5, // Lo  [70] BAMUM LETTER A..BAMUM LETTER KI
            0xA6E6..=0xA6EF, // Nl  [10] BAMUM LETTER MO..BAMUM LETTER KOGHOM
            0xA717..=0xA71F, // Lm   [9] MODIFIER LETTER DOT VERTICAL BAR..MODIFIER LETTER LOW INVERTED EXCLAMATION MARK
            0xA722..=0xA76F, // L&  [78] LATIN CAPITAL LETTER EGYPTOLOGICAL ALEF..LATIN SMALL LETTER CON
            0xA770..=0xA770, // Lm       MODIFIER LETTER US
            0xA771..=0xA787, // L&  [23] LATIN SMALL LETTER DUM..LATIN SMALL LETTER INSULAR T
            0xA788..=0xA788, // Lm       MODIFIER LETTER LOW CIRCUMFLEX ACCENT
            0xA78B..=0xA78E, // L&   [4] LATIN CAPITAL LETTER SALTILLO..LATIN SMALL LETTER L WITH RETROFLEX HOOK AND BELT
            0xA78F..=0xA78F, // Lo       LATIN LETTER SINOLOGICAL DOT
            0xA790..=0xA7AE, // L&  [31] LATIN CAPITAL LETTER N WITH DESCENDER..LATIN CAPITAL LETTER SMALL CAPITAL I
            0xA7B0..=0xA7B7, // L&   [8] LATIN CAPITAL LETTER TURNED K..LATIN SMALL LETTER OMEGA
            0xA7F7..=0xA7F7, // Lo       LATIN EPIGRAPHIC LETTER SIDEWAYS I
            0xA7F8..=0xA7F9, // Lm   [2] MODIFIER LETTER CAPITAL H WITH STROKE..MODIFIER LETTER SMALL LIGATURE OE
            0xA7FA..=0xA7FA, // L&       LATIN LETTER SMALL CAPITAL TURNED M
            0xA7FB..=0xA801, // Lo   [7] LATIN EPIGRAPHIC LETTER REVERSED F..SYLOTI NAGRI LETTER I
            0xA803..=0xA805, // Lo   [3] SYLOTI NAGRI LETTER U..SYLOTI NAGRI LETTER O
            0xA807..=0xA80A, // Lo   [4] SYLOTI NAGRI LETTER KO..SYLOTI NAGRI LETTER GHO
            0xA80C..=0xA822, // Lo  [23] SYLOTI NAGRI LETTER CO..SYLOTI NAGRI LETTER HO
            0xA840..=0xA873, // Lo  [52] PHAGS-PA LETTER KA..PHAGS-PA LETTER CANDRABINDU
            0xA882..=0xA8B3, // Lo  [50] SAURASHTRA LETTER A..SAURASHTRA LETTER LLA
            0xA8F2..=0xA8F7, // Lo   [6] DEVANAGARI SIGN SPACING CANDRABINDU..DEVANAGARI SIGN CANDRABINDU AVAGRAHA
            0xA8FB..=0xA8FB, // Lo       DEVANAGARI HEADSTROKE
            0xA8FD..=0xA8FD, // Lo       DEVANAGARI JAIN OM
            0xA90A..=0xA925, // Lo  [28] KAYAH LI LETTER KA..KAYAH LI LETTER OO
            0xA930..=0xA946, // Lo  [23] REJANG LETTER KA..REJANG LETTER A
            0xA960..=0xA97C, // Lo  [29] HANGUL CHOSEONG TIKEUT-MIEUM..HANGUL CHOSEONG SSANGYEORINHIEUH
            0xA984..=0xA9B2, // Lo  [47] JAVANESE LETTER A..JAVANESE LETTER HA
            0xA9CF..=0xA9CF, // Lm       JAVANESE PANGRANGKEP
            0xA9E0..=0xA9E4, // Lo   [5] MYANMAR LETTER SHAN GHA..MYANMAR LETTER SHAN BHA
            0xA9E6..=0xA9E6, // Lm       MYANMAR MODIFIER LETTER SHAN REDUPLICATION
            0xA9E7..=0xA9EF, // Lo   [9] MYANMAR LETTER TAI LAING NYA..MYANMAR LETTER TAI LAING NNA
            0xA9FA..=0xA9FE, // Lo   [5] MYANMAR LETTER TAI LAING LLA..MYANMAR LETTER TAI LAING BHA
            0xAA00..=0xAA28, // Lo  [41] CHAM LETTER A..CHAM LETTER HA
            0xAA40..=0xAA42, // Lo   [3] CHAM LETTER FINAL K..CHAM LETTER FINAL NG
            0xAA44..=0xAA4B, // Lo   [8] CHAM LETTER FINAL CH..CHAM LETTER FINAL SS
            0xAA60..=0xAA6F, // Lo  [16] MYANMAR LETTER KHAMTI GA..MYANMAR LETTER KHAMTI FA
            0xAA70..=0xAA70, // Lm       MYANMAR MODIFIER LETTER KHAMTI REDUPLICATION
            0xAA71..=0xAA76, // Lo   [6] MYANMAR LETTER KHAMTI XA..MYANMAR LOGOGRAM KHAMTI HM
            0xAA7A..=0xAA7A, // Lo       MYANMAR LETTER AITON RA
            0xAA7E..=0xAAAF, // Lo  [50] MYANMAR LETTER SHWE PALAUNG CHA..TAI VIET LETTER HIGH O
            0xAAB1..=0xAAB1, // Lo       TAI VIET VOWEL AA
            0xAAB5..=0xAAB6, // Lo   [2] TAI VIET VOWEL E..TAI VIET VOWEL O
            0xAAB9..=0xAABD, // Lo   [5] TAI VIET VOWEL UEA..TAI VIET VOWEL AN
            0xAAC0..=0xAAC0, // Lo       TAI VIET TONE MAI NUENG
            0xAAC2..=0xAAC2, // Lo       TAI VIET TONE MAI SONG
            0xAADB..=0xAADC, // Lo   [2] TAI VIET SYMBOL KON..TAI VIET SYMBOL NUENG
            0xAADD..=0xAADD, // Lm       TAI VIET SYMBOL SAM
            0xAAE0..=0xAAEA, // Lo  [11] MEETEI MAYEK LETTER E..MEETEI MAYEK LETTER SSA
            0xAAF2..=0xAAF2, // Lo       MEETEI MAYEK ANJI
            0xAAF3..=0xAAF4, // Lm   [2] MEETEI MAYEK SYLLABLE REPETITION MARK..MEETEI MAYEK WORD REPETITION MARK
            0xAB01..=0xAB06, // Lo   [6] ETHIOPIC SYLLABLE TTHU..ETHIOPIC SYLLABLE TTHO
            0xAB09..=0xAB0E, // Lo   [6] ETHIOPIC SYLLABLE DDHU..ETHIOPIC SYLLABLE DDHO
            0xAB11..=0xAB16, // Lo   [6] ETHIOPIC SYLLABLE DZU..ETHIOPIC SYLLABLE DZO
            0xAB20..=0xAB26, // Lo   [7] ETHIOPIC SYLLABLE CCHHA..ETHIOPIC SYLLABLE CCHHO
            0xAB28..=0xAB2E, // Lo   [7] ETHIOPIC SYLLABLE BBA..ETHIOPIC SYLLABLE BBO
            0xAB30..=0xAB5A, // L&  [43] LATIN SMALL LETTER BARRED ALPHA..LATIN SMALL LETTER Y WITH SHORT RIGHT LEG
            0xAB5C..=0xAB5F, // Lm   [4] MODIFIER LETTER SMALL HENG..MODIFIER LETTER SMALL U WITH LEFT HOOK
            0xAB60..=0xAB65, // L&   [6] LATIN SMALL LETTER SAKHA YAT..GREEK LETTER SMALL CAPITAL OMEGA
            0xAB70..=0xABBF, // L&  [80] CHEROKEE SMALL LETTER A..CHEROKEE SMALL LETTER YA
            0xABC0..=0xABE2, // Lo  [35] MEETEI MAYEK LETTER KOK..MEETEI MAYEK LETTER I LONSUM
            0xAC00..=0xD7A3, // Lo [11172] HANGUL SYLLABLE GA..HANGUL SYLLABLE HIH
            0xD7B0..=0xD7C6, // Lo  [23] HANGUL JUNGSEONG O-YEO..HANGUL JUNGSEONG ARAEA-E
            0xD7CB..=0xD7FB, // Lo  [49] HANGUL JONGSEONG NIEUN-RIEUL..HANGUL JONGSEONG PHIEUPH-THIEUTH
            0xF900..=0xFA6D, // Lo [366] CJK COMPATIBILITY IDEOGRAPH-F900..CJK COMPATIBILITY IDEOGRAPH-FA6D
            0xFA70..=0xFAD9, // Lo [106] CJK COMPATIBILITY IDEOGRAPH-FA70..CJK COMPATIBILITY IDEOGRAPH-FAD9
            0xFB00..=0xFB06, // L&   [7] LATIN SMALL LIGATURE FF..LATIN SMALL LIGATURE ST
            0xFB13..=0xFB17, // L&   [5] ARMENIAN SMALL LIGATURE MEN NOW..ARMENIAN SMALL LIGATURE MEN XEH
            0xFB1D..=0xFB1D, // Lo       HEBREW LETTER YOD WITH HIRIQ
            0xFB1F..=0xFB28, // Lo  [10] HEBREW LIGATURE YIDDISH YOD YOD PATAH..HEBREW LETTER WIDE TAV
            0xFB2A..=0xFB36, // Lo  [13] HEBREW LETTER SHIN WITH SHIN DOT..HEBREW LETTER ZAYIN WITH DAGESH
            0xFB38..=0xFB3C, // Lo   [5] HEBREW LETTER TET WITH DAGESH..HEBREW LETTER LAMED WITH DAGESH
            0xFB3E..=0xFB3E, // Lo       HEBREW LETTER MEM WITH DAGESH
            0xFB40..=0xFB41, // Lo   [2] HEBREW LETTER NUN WITH DAGESH..HEBREW LETTER SAMEKH WITH DAGESH
            0xFB43..=0xFB44, // Lo   [2] HEBREW LETTER FINAL PE WITH DAGESH..HEBREW LETTER PE WITH DAGESH
            0xFB46..=0xFBB1, // Lo [108] HEBREW LETTER TSADI WITH DAGESH..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE FINAL FORM
            0xFBD3..=0xFD3D, // Lo [363] ARABIC LETTER NG ISOLATED FORM..ARABIC LIGATURE ALEF WITH FATHATAN ISOLATED FORM
            0xFD50..=0xFD8F, // Lo  [64] ARABIC LIGATURE TEH WITH JEEM WITH MEEM INITIAL FORM..ARABIC LIGATURE MEEM WITH KHAH WITH MEEM INITIAL FORM
            0xFD92..=0xFDC7, // Lo  [54] ARABIC LIGATURE MEEM WITH JEEM WITH KHAH INITIAL FORM..ARABIC LIGATURE NOON WITH JEEM WITH YEH FINAL FORM
            0xFDF0..=0xFDFB, // Lo  [12] ARABIC LIGATURE SALLA USED AS KORANIC STOP SIGN ISOLATED FORM..ARABIC LIGATURE JALLAJALALOUHOU
            0xFE70..=0xFE74, // Lo   [5] ARABIC FATHATAN ISOLATED FORM..ARABIC KASRATAN ISOLATED FORM
            0xFE76..=0xFEFC, // Lo [135] ARABIC FATHA ISOLATED FORM..ARABIC LIGATURE LAM WITH ALEF FINAL FORM
            0xFF21..=0xFF3A, // L&  [26] FULLWIDTH LATIN CAPITAL LETTER A..FULLWIDTH LATIN CAPITAL LETTER Z
            0xFF41..=0xFF5A, // L&  [26] FULLWIDTH LATIN SMALL LETTER A..FULLWIDTH LATIN SMALL LETTER Z
            0xFF66..=0xFF6F, // Lo  [10] HALFWIDTH KATAKANA LETTER WO..HALFWIDTH KATAKANA LETTER SMALL TU
            0xFF70..=0xFF70, // Lm       HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK
            0xFF71..=0xFF9D, // Lo  [45] HALFWIDTH KATAKANA LETTER A..HALFWIDTH KATAKANA LETTER N
            0xFF9E..=0xFF9F, // Lm   [2] HALFWIDTH KATAKANA VOICED SOUND MARK..HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK
            0xFFA0..=0xFFBE, // Lo  [31] HALFWIDTH HANGUL FILLER..HALFWIDTH HANGUL LETTER HIEUH
            0xFFC2..=0xFFC7, // Lo   [6] HALFWIDTH HANGUL LETTER A..HALFWIDTH HANGUL LETTER E
            0xFFCA..=0xFFCF, // Lo   [6] HALFWIDTH HANGUL LETTER YEO..HALFWIDTH HANGUL LETTER OE
            0xFFD2..=0xFFD7, // Lo   [6] HALFWIDTH HANGUL LETTER YO..HALFWIDTH HANGUL LETTER YU
            0xFFDA..=0xFFDC, // Lo   [3] HALFWIDTH HANGUL LETTER EU..HALFWIDTH HANGUL LETTER I
        ][..]
    }

    fn r32() -> &'static [RangeInclusive<u32>] {
        &[
            0x10000..=0x1000B, // Lo  [12] LINEAR B SYLLABLE B008 A..LINEAR B SYLLABLE B046 JE
            0x1000D..=0x10026, // Lo  [26] LINEAR B SYLLABLE B036 JO..LINEAR B SYLLABLE B032 QO
            0x10028..=0x1003A, // Lo  [19] LINEAR B SYLLABLE B060 RA..LINEAR B SYLLABLE B042 WO
            0x1003C..=0x1003D, // Lo   [2] LINEAR B SYLLABLE B017 ZA..LINEAR B SYLLABLE B074 ZE
            0x1003F..=0x1004D, // Lo  [15] LINEAR B SYLLABLE B020 ZO..LINEAR B SYLLABLE B091 TWO
            0x10050..=0x1005D, // Lo  [14] LINEAR B SYMBOL B018..LINEAR B SYMBOL B089
            0x10080..=0x100FA, // Lo [123] LINEAR B IDEOGRAM B100 MAN..LINEAR B IDEOGRAM VESSEL B305
            0x10140..=0x10174, // Nl  [53] GREEK ACROPHONIC ATTIC ONE QUARTER..GREEK ACROPHONIC STRATIAN FIFTY MNAS
            0x10280..=0x1029C, // Lo  [29] LYCIAN LETTER A..LYCIAN LETTER X
            0x102A0..=0x102D0, // Lo  [49] CARIAN LETTER A..CARIAN LETTER UUU3
            0x10300..=0x1031F, // Lo  [32] OLD ITALIC LETTER A..OLD ITALIC LETTER ESS
            0x10330..=0x10340, // Lo  [17] GOTHIC LETTER AHSA..GOTHIC LETTER PAIRTHRA
            0x10341..=0x10341, // Nl       GOTHIC LETTER NINETY
            0x10342..=0x10349, // Lo   [8] GOTHIC LETTER RAIDA..GOTHIC LETTER OTHAL
            0x1034A..=0x1034A, // Nl       GOTHIC LETTER NINE HUNDRED
            0x10350..=0x10375, // Lo  [38] OLD PERMIC LETTER AN..OLD PERMIC LETTER IA
            0x10380..=0x1039D, // Lo  [30] UGARITIC LETTER ALPA..UGARITIC LETTER SSU
            0x103A0..=0x103C3, // Lo  [36] OLD PERSIAN SIGN A..OLD PERSIAN SIGN HA
            0x103C8..=0x103CF, // Lo   [8] OLD PERSIAN SIGN AURAMAZDAA..OLD PERSIAN SIGN BUUMISH
            0x103D1..=0x103D5, // Nl   [5] OLD PERSIAN NUMBER ONE..OLD PERSIAN NUMBER HUNDRED
            0x10400..=0x1044F, // L&  [80] DESERET CAPITAL LETTER LONG I..DESERET SMALL LETTER EW
            0x10450..=0x1049D, // Lo  [78] SHAVIAN LETTER PEEP..OSMANYA LETTER OO
            0x104B0..=0x104D3, // L&  [36] OSAGE CAPITAL LETTER A..OSAGE CAPITAL LETTER ZHA
            0x104D8..=0x104FB, // L&  [36] OSAGE SMALL LETTER A..OSAGE SMALL LETTER ZHA
            0x10500..=0x10527, // Lo  [40] ELBASAN LETTER A..ELBASAN LETTER KHE
            0x10530..=0x10563, // Lo  [52] CAUCASIAN ALBANIAN LETTER ALT..CAUCASIAN ALBANIAN LETTER KIW
            0x10600..=0x10736, // Lo [311] LINEAR A SIGN AB001..LINEAR A SIGN A664
            0x10740..=0x10755, // Lo  [22] LINEAR A SIGN A701 A..LINEAR A SIGN A732 JE
            0x10760..=0x10767, // Lo   [8] LINEAR A SIGN A800..LINEAR A SIGN A807
            0x10800..=0x10805, // Lo   [6] CYPRIOT SYLLABLE A..CYPRIOT SYLLABLE JA
            0x10808..=0x10808, // Lo       CYPRIOT SYLLABLE JO
            0x1080A..=0x10835, // Lo  [44] CYPRIOT SYLLABLE KA..CYPRIOT SYLLABLE WO
            0x10837..=0x10838, // Lo   [2] CYPRIOT SYLLABLE XA..CYPRIOT SYLLABLE XE
            0x1083C..=0x1083C, // Lo       CYPRIOT SYLLABLE ZA
            0x1083F..=0x10855, // Lo  [23] CYPRIOT SYLLABLE ZO..IMPERIAL ARAMAIC LETTER TAW
            0x10860..=0x10876, // Lo  [23] PALMYRENE LETTER ALEPH..PALMYRENE LETTER TAW
            0x10880..=0x1089E, // Lo  [31] NABATAEAN LETTER FINAL ALEPH..NABATAEAN LETTER TAW
            0x108E0..=0x108F2, // Lo  [19] HATRAN LETTER ALEPH..HATRAN LETTER QOPH
            0x108F4..=0x108F5, // Lo   [2] HATRAN LETTER SHIN..HATRAN LETTER TAW
            0x10900..=0x10915, // Lo  [22] PHOENICIAN LETTER ALF..PHOENICIAN LETTER TAU
            0x10920..=0x10939, // Lo  [26] LYDIAN LETTER A..LYDIAN LETTER C
            0x10980..=0x109B7, // Lo  [56] MEROITIC HIEROGLYPHIC LETTER A..MEROITIC CURSIVE LETTER DA
            0x109BE..=0x109BF, // Lo   [2] MEROITIC CURSIVE LOGOGRAM RMT..MEROITIC CURSIVE LOGOGRAM IMN
            0x10A00..=0x10A00, // Lo       KHAROSHTHI LETTER A
            0x10A10..=0x10A13, // Lo   [4] KHAROSHTHI LETTER KA..KHAROSHTHI LETTER GHA
            0x10A15..=0x10A17, // Lo   [3] KHAROSHTHI LETTER CA..KHAROSHTHI LETTER JA
            0x10A19..=0x10A33, // Lo  [27] KHAROSHTHI LETTER NYA..KHAROSHTHI LETTER TTTHA
            0x10A60..=0x10A7C, // Lo  [29] OLD SOUTH ARABIAN LETTER HE..OLD SOUTH ARABIAN LETTER THETH
            0x10A80..=0x10A9C, // Lo  [29] OLD NORTH ARABIAN LETTER HEH..OLD NORTH ARABIAN LETTER ZAH
            0x10AC0..=0x10AC7, // Lo   [8] MANICHAEAN LETTER ALEPH..MANICHAEAN LETTER WAW
            0x10AC9..=0x10AE4, // Lo  [28] MANICHAEAN LETTER ZAYIN..MANICHAEAN LETTER TAW
            0x10B00..=0x10B35, // Lo  [54] AVESTAN LETTER A..AVESTAN LETTER HE
            0x10B40..=0x10B55, // Lo  [22] INSCRIPTIONAL PARTHIAN LETTER ALEPH..INSCRIPTIONAL PARTHIAN LETTER TAW
            0x10B60..=0x10B72, // Lo  [19] INSCRIPTIONAL PAHLAVI LETTER ALEPH..INSCRIPTIONAL PAHLAVI LETTER TAW
            0x10B80..=0x10B91, // Lo  [18] PSALTER PAHLAVI LETTER ALEPH..PSALTER PAHLAVI LETTER TAW
            0x10C00..=0x10C48, // Lo  [73] OLD TURKIC LETTER ORKHON A..OLD TURKIC LETTER ORKHON BASH
            0x10C80..=0x10CB2, // L&  [51] OLD HUNGARIAN CAPITAL LETTER A..OLD HUNGARIAN CAPITAL LETTER US
            0x10CC0..=0x10CF2, // L&  [51] OLD HUNGARIAN SMALL LETTER A..OLD HUNGARIAN SMALL LETTER US
            0x11003..=0x11037, // Lo  [53] BRAHMI SIGN JIHVAMULIYA..BRAHMI LETTER OLD TAMIL NNNA
            0x11083..=0x110AF, // Lo  [45] KAITHI LETTER A..KAITHI LETTER HA
            0x110D0..=0x110E8, // Lo  [25] SORA SOMPENG LETTER SAH..SORA SOMPENG LETTER MAE
            0x11103..=0x11126, // Lo  [36] CHAKMA LETTER AA..CHAKMA LETTER HAA
            0x11150..=0x11172, // Lo  [35] MAHAJANI LETTER A..MAHAJANI LETTER RRA
            0x11176..=0x11176, // Lo       MAHAJANI LIGATURE SHRI
            0x11183..=0x111B2, // Lo  [48] SHARADA LETTER A..SHARADA LETTER HA
            0x111C1..=0x111C4, // Lo   [4] SHARADA SIGN AVAGRAHA..SHARADA OM
            0x111DA..=0x111DA, // Lo       SHARADA EKAM
            0x111DC..=0x111DC, // Lo       SHARADA HEADSTROKE
            0x11200..=0x11211, // Lo  [18] KHOJKI LETTER A..KHOJKI LETTER JJA
            0x11213..=0x1122B, // Lo  [25] KHOJKI LETTER NYA..KHOJKI LETTER LLA
            0x11280..=0x11286, // Lo   [7] MULTANI LETTER A..MULTANI LETTER GA
            0x11288..=0x11288, // Lo       MULTANI LETTER GHA
            0x1128A..=0x1128D, // Lo   [4] MULTANI LETTER CA..MULTANI LETTER JJA
            0x1128F..=0x1129D, // Lo  [15] MULTANI LETTER NYA..MULTANI LETTER BA
            0x1129F..=0x112A8, // Lo  [10] MULTANI LETTER BHA..MULTANI LETTER RHA
            0x112B0..=0x112DE, // Lo  [47] KHUDAWADI LETTER A..KHUDAWADI LETTER HA
            0x11305..=0x1130C, // Lo   [8] GRANTHA LETTER A..GRANTHA LETTER VOCALIC L
            0x1130F..=0x11310, // Lo   [2] GRANTHA LETTER EE..GRANTHA LETTER AI
            0x11313..=0x11328, // Lo  [22] GRANTHA LETTER OO..GRANTHA LETTER NA
            0x1132A..=0x11330, // Lo   [7] GRANTHA LETTER PA..GRANTHA LETTER RA
            0x11332..=0x11333, // Lo   [2] GRANTHA LETTER LA..GRANTHA LETTER LLA
            0x11335..=0x11339, // Lo   [5] GRANTHA LETTER VA..GRANTHA LETTER HA
            0x1133D..=0x1133D, // Lo       GRANTHA SIGN AVAGRAHA
            0x11350..=0x11350, // Lo       GRANTHA OM
            0x1135D..=0x11361, // Lo   [5] GRANTHA SIGN PLUTA..GRANTHA LETTER VOCALIC LL
            0x11400..=0x11434, // Lo  [53] NEWA LETTER A..NEWA LETTER HA
            0x11447..=0x1144A, // Lo   [4] NEWA SIGN AVAGRAHA..NEWA SIDDHI
            0x11480..=0x114AF, // Lo  [48] TIRHUTA ANJI..TIRHUTA LETTER HA
            0x114C4..=0x114C5, // Lo   [2] TIRHUTA SIGN AVAGRAHA..TIRHUTA GVANG
            0x114C7..=0x114C7, // Lo       TIRHUTA OM
            0x11580..=0x115AE, // Lo  [47] SIDDHAM LETTER A..SIDDHAM LETTER HA
            0x115D8..=0x115DB, // Lo   [4] SIDDHAM LETTER THREE-CIRCLE ALTERNATE I..SIDDHAM LETTER ALTERNATE U
            0x11600..=0x1162F, // Lo  [48] MODI LETTER A..MODI LETTER LLA
            0x11644..=0x11644, // Lo       MODI SIGN HUVA
            0x11680..=0x116AA, // Lo  [43] TAKRI LETTER A..TAKRI LETTER RRA
            0x11700..=0x11719, // Lo  [26] AHOM LETTER KA..AHOM LETTER JHA
            0x118A0..=0x118DF, // L&  [64] WARANG CITI CAPITAL LETTER NGAA..WARANG CITI SMALL LETTER VIYO
            0x118FF..=0x118FF, // Lo       WARANG CITI OM
            0x11AC0..=0x11AF8, // Lo  [57] PAU CIN HAU LETTER PA..PAU CIN HAU GLOTTAL STOP FINAL
            0x11C00..=0x11C08, // Lo   [9] BHAIKSUKI LETTER A..BHAIKSUKI LETTER VOCALIC L
            0x11C0A..=0x11C2E, // Lo  [37] BHAIKSUKI LETTER E..BHAIKSUKI LETTER HA
            0x11C40..=0x11C40, // Lo       BHAIKSUKI SIGN AVAGRAHA
            0x11C72..=0x11C8F, // Lo  [30] MARCHEN LETTER KA..MARCHEN LETTER A
            0x12000..=0x12399, // Lo [922] CUNEIFORM SIGN A..CUNEIFORM SIGN U U
            0x12400..=0x1246E, // Nl [111] CUNEIFORM NUMERIC SIGN TWO ASH..CUNEIFORM NUMERIC SIGN NINE U VARIANT FORM
            0x12480..=0x12543, // Lo [196] CUNEIFORM SIGN AB TIMES NUN TENU..CUNEIFORM SIGN ZU5 TIMES THREE DISH TENU
            0x13000..=0x1342E, // Lo [1071] EGYPTIAN HIEROGLYPH A001..EGYPTIAN HIEROGLYPH AA032
            0x14400..=0x14646, // Lo [583] ANATOLIAN HIEROGLYPH A001..ANATOLIAN HIEROGLYPH A530
            0x16800..=0x16A38, // Lo [569] BAMUM LETTER PHASE-A NGKUE MFON..BAMUM LETTER PHASE-F VUEQ
            0x16A40..=0x16A5E, // Lo  [31] MRO LETTER TA..MRO LETTER TEK
            0x16AD0..=0x16AED, // Lo  [30] BASSA VAH LETTER ENNI..BASSA VAH LETTER I
            0x16B00..=0x16B2F, // Lo  [48] PAHAWH HMONG VOWEL KEEB..PAHAWH HMONG CONSONANT CAU
            0x16B40..=0x16B43, // Lm   [4] PAHAWH HMONG SIGN VOS SEEV..PAHAWH HMONG SIGN IB YAM
            0x16B63..=0x16B77, // Lo  [21] PAHAWH HMONG SIGN VOS LUB..PAHAWH HMONG SIGN CIM NRES TOS
            0x16B7D..=0x16B8F, // Lo  [19] PAHAWH HMONG CLAN SIGN TSHEEJ..PAHAWH HMONG CLAN SIGN VWJ
            0x16F00..=0x16F44, // Lo  [69] MIAO LETTER PA..MIAO LETTER HHA
            0x16F50..=0x16F50, // Lo       MIAO LETTER NASALIZATION
            0x16F93..=0x16F9F, // Lm  [13] MIAO LETTER TONE-2..MIAO LETTER REFORMED TONE-8
            0x16FE0..=0x16FE0, // Lm       TANGUT ITERATION MARK
            0x17000..=0x187EC, // Lo [6125] TANGUT IDEOGRAPH-17000..TANGUT IDEOGRAPH-187EC
            0x18800..=0x18AF2, // Lo [755] TANGUT COMPONENT-001..TANGUT COMPONENT-755
            0x1B000..=0x1B001, // Lo   [2] KATAKANA LETTER ARCHAIC E..HIRAGANA LETTER ARCHAIC YE
            0x1BC00..=0x1BC6A, // Lo [107] DUPLOYAN LETTER H..DUPLOYAN LETTER VOCALIC M
            0x1BC70..=0x1BC7C, // Lo  [13] DUPLOYAN AFFIX LEFT HORIZONTAL SECANT..DUPLOYAN AFFIX ATTACHED TANGENT HOOK
            0x1BC80..=0x1BC88, // Lo   [9] DUPLOYAN AFFIX HIGH ACUTE..DUPLOYAN AFFIX HIGH VERTICAL
            0x1BC90..=0x1BC99, // Lo  [10] DUPLOYAN AFFIX LOW ACUTE..DUPLOYAN AFFIX LOW ARROW
            0x1D400..=0x1D454, // L&  [85] MATHEMATICAL BOLD CAPITAL A..MATHEMATICAL ITALIC SMALL G
            0x1D456..=0x1D49C, // L&  [71] MATHEMATICAL ITALIC SMALL I..MATHEMATICAL SCRIPT CAPITAL A
            0x1D49E..=0x1D49F, // L&   [2] MATHEMATICAL SCRIPT CAPITAL C..MATHEMATICAL SCRIPT CAPITAL D
            0x1D4A2..=0x1D4A2, // L&       MATHEMATICAL SCRIPT CAPITAL G
            0x1D4A5..=0x1D4A6, // L&   [2] MATHEMATICAL SCRIPT CAPITAL J..MATHEMATICAL SCRIPT CAPITAL K
            0x1D4A9..=0x1D4AC, // L&   [4] MATHEMATICAL SCRIPT CAPITAL N..MATHEMATICAL SCRIPT CAPITAL Q
            0x1D4AE..=0x1D4B9, // L&  [12] MATHEMATICAL SCRIPT CAPITAL S..MATHEMATICAL SCRIPT SMALL D
            0x1D4BB..=0x1D4BB, // L&       MATHEMATICAL SCRIPT SMALL F
            0x1D4BD..=0x1D4C3, // L&   [7] MATHEMATICAL SCRIPT SMALL H..MATHEMATICAL SCRIPT SMALL N
            0x1D4C5..=0x1D505, // L&  [65] MATHEMATICAL SCRIPT SMALL P..MATHEMATICAL FRAKTUR CAPITAL B
            0x1D507..=0x1D50A, // L&   [4] MATHEMATICAL FRAKTUR CAPITAL D..MATHEMATICAL FRAKTUR CAPITAL G
            0x1D50D..=0x1D514, // L&   [8] MATHEMATICAL FRAKTUR CAPITAL J..MATHEMATICAL FRAKTUR CAPITAL Q
            0x1D516..=0x1D51C, // L&   [7] MATHEMATICAL FRAKTUR CAPITAL S..MATHEMATICAL FRAKTUR CAPITAL Y
            0x1D51E..=0x1D539, // L&  [28] MATHEMATICAL FRAKTUR SMALL A..MATHEMATICAL DOUBLE-STRUCK CAPITAL B
            0x1D53B..=0x1D53E, // L&   [4] MATHEMATICAL DOUBLE-STRUCK CAPITAL D..MATHEMATICAL DOUBLE-STRUCK CAPITAL G
            0x1D540..=0x1D544, // L&   [5] MATHEMATICAL DOUBLE-STRUCK CAPITAL I..MATHEMATICAL DOUBLE-STRUCK CAPITAL M
            0x1D546..=0x1D546, // L&       MATHEMATICAL DOUBLE-STRUCK CAPITAL O
            0x1D54A..=0x1D550, // L&   [7] MATHEMATICAL DOUBLE-STRUCK CAPITAL S..MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
            0x1D552..=0x1D6A5, // L& [340] MATHEMATICAL DOUBLE-STRUCK SMALL A..MATHEMATICAL ITALIC SMALL DOTLESS J
            0x1D6A8..=0x1D6C0, // L&  [25] MATHEMATICAL BOLD CAPITAL ALPHA..MATHEMATICAL BOLD CAPITAL OMEGA
            0x1D6C2..=0x1D6DA, // L&  [25] MATHEMATICAL BOLD SMALL ALPHA..MATHEMATICAL BOLD SMALL OMEGA
            0x1D6DC..=0x1D6FA, // L&  [31] MATHEMATICAL BOLD EPSILON SYMBOL..MATHEMATICAL ITALIC CAPITAL OMEGA
            0x1D6FC..=0x1D714, // L&  [25] MATHEMATICAL ITALIC SMALL ALPHA..MATHEMATICAL ITALIC SMALL OMEGA
            0x1D716..=0x1D734, // L&  [31] MATHEMATICAL ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD ITALIC CAPITAL OMEGA
            0x1D736..=0x1D74E, // L&  [25] MATHEMATICAL BOLD ITALIC SMALL ALPHA..MATHEMATICAL BOLD ITALIC SMALL OMEGA
            0x1D750..=0x1D76E, // L&  [31] MATHEMATICAL BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD CAPITAL OMEGA
            0x1D770..=0x1D788, // L&  [25] MATHEMATICAL SANS-SERIF BOLD SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD SMALL OMEGA
            0x1D78A..=0x1D7A8, // L&  [31] MATHEMATICAL SANS-SERIF BOLD EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD ITALIC CAPITAL OME
            0x1D7AA..=0x1D7C2, // L&  [25] MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL O
            0x1D7C4..=0x1D7CB, // L&   [8] MATHEMATICAL SANS-SERIF BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD SMALL DIGAMMA
            0x1E800..=0x1E8C4, // Lo [197] MENDE KIKAKUI SYLLABLE M001 KI..MENDE KIKAKUI SYLLABLE M060 NYON
            0x1E900..=0x1E943, // L&  [68] ADLAM CAPITAL LETTER ALIF..ADLAM SMALL LETTER SHA
            0x1EE00..=0x1EE03, // Lo   [4] ARABIC MATHEMATICAL ALEF..ARABIC MATHEMATICAL DAL
            0x1EE05..=0x1EE1F, // Lo  [27] ARABIC MATHEMATICAL WAW..ARABIC MATHEMATICAL DOTLESS QAF
            0x1EE21..=0x1EE22, // Lo   [2] ARABIC MATHEMATICAL INITIAL BEH..ARABIC MATHEMATICAL INITIAL JEEM
            0x1EE24..=0x1EE24, // Lo       ARABIC MATHEMATICAL INITIAL HEH
            0x1EE27..=0x1EE27, // Lo       ARABIC MATHEMATICAL INITIAL HAH
            0x1EE29..=0x1EE32, // Lo  [10] ARABIC MATHEMATICAL INITIAL YEH..ARABIC MATHEMATICAL INITIAL QAF
            0x1EE34..=0x1EE37, // Lo   [4] ARABIC MATHEMATICAL INITIAL SHEEN..ARABIC MATHEMATICAL INITIAL KHAH
            0x1EE39..=0x1EE39, // Lo       ARABIC MATHEMATICAL INITIAL DAD
            0x1EE3B..=0x1EE3B, // Lo       ARABIC MATHEMATICAL INITIAL GHAIN
            0x1EE42..=0x1EE42, // Lo       ARABIC MATHEMATICAL TAILED JEEM
            0x1EE47..=0x1EE47, // Lo       ARABIC MATHEMATICAL TAILED HAH
            0x1EE49..=0x1EE49, // Lo       ARABIC MATHEMATICAL TAILED YEH
            0x1EE4B..=0x1EE4B, // Lo       ARABIC MATHEMATICAL TAILED LAM
            0x1EE4D..=0x1EE4F, // Lo   [3] ARABIC MATHEMATICAL TAILED NOON..ARABIC MATHEMATICAL TAILED AIN
            0x1EE51..=0x1EE52, // Lo   [2] ARABIC MATHEMATICAL TAILED SAD..ARABIC MATHEMATICAL TAILED QAF
            0x1EE54..=0x1EE54, // Lo       ARABIC MATHEMATICAL TAILED SHEEN
            0x1EE57..=0x1EE57, // Lo       ARABIC MATHEMATICAL TAILED KHAH
            0x1EE59..=0x1EE59, // Lo       ARABIC MATHEMATICAL TAILED DAD
            0x1EE5B..=0x1EE5B, // Lo       ARABIC MATHEMATICAL TAILED GHAIN
            0x1EE5D..=0x1EE5D, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS NOON
            0x1EE5F..=0x1EE5F, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS QAF
            0x1EE61..=0x1EE62, // Lo   [2] ARABIC MATHEMATICAL STRETCHED BEH..ARABIC MATHEMATICAL STRETCHED JEEM
            0x1EE64..=0x1EE64, // Lo       ARABIC MATHEMATICAL STRETCHED HEH
            0x1EE67..=0x1EE6A, // Lo   [4] ARABIC MATHEMATICAL STRETCHED HAH..ARABIC MATHEMATICAL STRETCHED KAF
            0x1EE6C..=0x1EE72, // Lo   [7] ARABIC MATHEMATICAL STRETCHED MEEM..ARABIC MATHEMATICAL STRETCHED QAF
            0x1EE74..=0x1EE77, // Lo   [4] ARABIC MATHEMATICAL STRETCHED SHEEN..ARABIC MATHEMATICAL STRETCHED KHAH
            0x1EE79..=0x1EE7C, // Lo   [4] ARABIC MATHEMATICAL STRETCHED DAD..ARABIC MATHEMATICAL STRETCHED DOTLESS BEH
            0x1EE7E..=0x1EE7E, // Lo       ARABIC MATHEMATICAL STRETCHED DOTLESS FEH
            0x1EE80..=0x1EE89, // Lo  [10] ARABIC MATHEMATICAL LOOPED ALEF..ARABIC MATHEMATICAL LOOPED YEH
            0x1EE8B..=0x1EE9B, // Lo  [17] ARABIC MATHEMATICAL LOOPED LAM..ARABIC MATHEMATICAL LOOPED GHAIN
            0x1EEA1..=0x1EEA3, // Lo   [3] ARABIC MATHEMATICAL DOUBLE-STRUCK BEH..ARABIC MATHEMATICAL DOUBLE-STRUCK DAL
            0x1EEA5..=0x1EEA9, // Lo   [5] ARABIC MATHEMATICAL DOUBLE-STRUCK WAW..ARABIC MATHEMATICAL DOUBLE-STRUCK YEH
            0x1EEAB..=0x1EEBB, // Lo  [17] ARABIC MATHEMATICAL DOUBLE-STRUCK LAM..ARABIC MATHEMATICAL DOUBLE-STRUCK GHAIN
            0x20000..=0x2A6D6, // Lo [42711] CJK UNIFIED IDEOGRAPH-20000..CJK UNIFIED IDEOGRAPH-2A6D6
            0x2A700..=0x2B734, // Lo [4149] CJK UNIFIED IDEOGRAPH-2A700..CJK UNIFIED IDEOGRAPH-2B734
            0x2B740..=0x2B81D, // Lo [222] CJK UNIFIED IDEOGRAPH-2B740..CJK UNIFIED IDEOGRAPH-2B81D
            0x2B820..=0x2CEA1, // Lo [5762] CJK UNIFIED IDEOGRAPH-2B820..CJK UNIFIED IDEOGRAPH-2CEA1
            0x2F800..=0x2FA1D, // Lo [542] CJK COMPATIBILITY IDEOGRAPH-2F800..CJK COMPATIBILITY IDEOGRAPH-2FA1D
        ][..]
    }
}

pub struct IdContinue;

impl RangeTable for IdContinue {
    fn latin_offset() -> usize {
        129
    }

    fn r16() -> &'static [RangeInclusive<u16>] {
        &[
            0x0041..=0x005A, // L&  [26] LATIN CAPITAL LETTER A..LATIN CAPITAL LETTER Z
            0x0061..=0x007A, // L&  [26] LATIN SMALL LETTER A..LATIN SMALL LETTER Z
            0x00AA..=0x00AA, // Lo       FEMININE ORDINAL INDICATOR
            0x00B5..=0x00B5, // L&       MICRO SIGN
            0x00BA..=0x00BA, // Lo       MASCULINE ORDINAL INDICATOR
            0x00C0..=0x00D6, // L&  [23] LATIN CAPITAL LETTER A WITH GRAVE..LATIN CAPITAL LETTER O WITH DIAERESIS
            0x00D8..=0x00F6, // L&  [31] LATIN CAPITAL LETTER O WITH STROKE..LATIN SMALL LETTER O WITH DIAERESIS
            0x00F8..=0x01BA, // L& [195] LATIN SMALL LETTER O WITH STROKE..LATIN SMALL LETTER EZH WITH TAIL
            0x01BB..=0x01BB, // Lo       LATIN LETTER TWO WITH STROKE
            0x01BC..=0x01BF, // L&   [4] LATIN CAPITAL LETTER TONE FIVE..LATIN LETTER WYNN
            0x01C0..=0x01C3, // Lo   [4] LATIN LETTER DENTAL CLICK..LATIN LETTER RETROFLEX CLICK
            0x01C4..=0x0293, // L& [208] LATIN CAPITAL LETTER DZ WITH CARON..LATIN SMALL LETTER EZH WITH CURL
            0x0294..=0x0294, // Lo       LATIN LETTER GLOTTAL STOP
            0x0295..=0x02AF, // L&  [27] LATIN LETTER PHARYNGEAL VOICED FRICATIVE..LATIN SMALL LETTER TURNED H WITH FISHHOOK AND TAIL
            0x02B0..=0x02C1, // Lm  [18] MODIFIER LETTER SMALL H..MODIFIER LETTER REVERSED GLOTTAL STOP
            0x02C6..=0x02D1, // Lm  [12] MODIFIER LETTER CIRCUMFLEX ACCENT..MODIFIER LETTER HALF TRIANGULAR COLON
            0x02E0..=0x02E4, // Lm   [5] MODIFIER LETTER SMALL GAMMA..MODIFIER LETTER SMALL REVERSED GLOTTAL STOP
            0x02EC..=0x02EC, // Lm       MODIFIER LETTER VOICING
            0x02EE..=0x02EE, // Lm       MODIFIER LETTER DOUBLE APOSTROPHE
            0x0370..=0x0373, // L&   [4] GREEK CAPITAL LETTER HETA..GREEK SMALL LETTER ARCHAIC SAMPI
            0x0374..=0x0374, // Lm       GREEK NUMERAL SIGN
            0x0376..=0x0377, // L&   [2] GREEK CAPITAL LETTER PAMPHYLIAN DIGAMMA..GREEK SMALL LETTER PAMPHYLIAN DIGAMMA
            0x037A..=0x037A, // Lm       GREEK YPOGEGRAMMENI
            0x037B..=0x037D, // L&   [3] GREEK SMALL REVERSED LUNATE SIGMA SYMBOL..GREEK SMALL REVERSED DOTTED LUNATE SIGMA SYMBOL
            0x037F..=0x037F, // L&       GREEK CAPITAL LETTER YOT
            0x0386..=0x0386, // L&       GREEK CAPITAL LETTER ALPHA WITH TONOS
            0x0388..=0x038A, // L&   [3] GREEK CAPITAL LETTER EPSILON WITH TONOS..GREEK CAPITAL LETTER IOTA WITH TONOS
            0x038C..=0x038C, // L&       GREEK CAPITAL LETTER OMICRON WITH TONOS
            0x038E..=0x03A1, // L&  [20] GREEK CAPITAL LETTER UPSILON WITH TONOS..GREEK CAPITAL LETTER RHO
            0x03A3..=0x03F5, // L&  [83] GREEK CAPITAL LETTER SIGMA..GREEK LUNATE EPSILON SYMBOL
            0x03F7..=0x0481, // L& [139] GREEK CAPITAL LETTER SHO..CYRILLIC SMALL LETTER KOPPA
            0x048A..=0x052F, // L& [166] CYRILLIC CAPITAL LETTER SHORT I WITH TAIL..CYRILLIC SMALL LETTER EL WITH DESCENDER
            0x0531..=0x0556, // L&  [38] ARMENIAN CAPITAL LETTER AYB..ARMENIAN CAPITAL LETTER FEH
            0x0559..=0x0559, // Lm       ARMENIAN MODIFIER LETTER LEFT HALF RING
            0x0561..=0x0587, // L&  [39] ARMENIAN SMALL LETTER AYB..ARMENIAN SMALL LIGATURE ECH YIWN
            0x05D0..=0x05EA, // Lo  [27] HEBREW LETTER ALEF..HEBREW LETTER TAV
            0x05F0..=0x05F2, // Lo   [3] HEBREW LIGATURE YIDDISH DOUBLE VAV..HEBREW LIGATURE YIDDISH DOUBLE YOD
            0x0620..=0x063F, // Lo  [32] ARABIC LETTER KASHMIRI YEH..ARABIC LETTER FARSI YEH WITH THREE DOTS ABOVE
            0x0640..=0x0640, // Lm       ARABIC TATWEEL
            0x0641..=0x064A, // Lo  [10] ARABIC LETTER FEH..ARABIC LETTER YEH
            0x066E..=0x066F, // Lo   [2] ARABIC LETTER DOTLESS BEH..ARABIC LETTER DOTLESS QAF
            0x0671..=0x06D3, // Lo  [99] ARABIC LETTER ALEF WASLA..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE
            0x06D5..=0x06D5, // Lo       ARABIC LETTER AE
            0x06E5..=0x06E6, // Lm   [2] ARABIC SMALL WAW..ARABIC SMALL YEH
            0x06EE..=0x06EF, // Lo   [2] ARABIC LETTER DAL WITH INVERTED V..ARABIC LETTER REH WITH INVERTED V
            0x06FA..=0x06FC, // Lo   [3] ARABIC LETTER SHEEN WITH DOT BELOW..ARABIC LETTER GHAIN WITH DOT BELOW
            0x06FF..=0x06FF, // Lo       ARABIC LETTER HEH WITH INVERTED V
            0x0710..=0x0710, // Lo       SYRIAC LETTER ALAPH
            0x0712..=0x072F, // Lo  [30] SYRIAC LETTER BETH..SYRIAC LETTER PERSIAN DHALATH
            0x074D..=0x07A5, // Lo  [89] SYRIAC LETTER SOGDIAN ZHAIN..THAANA LETTER WAAVU
            0x07B1..=0x07B1, // Lo       THAANA LETTER NAA
            0x07CA..=0x07EA, // Lo  [33] NKO LETTER A..NKO LETTER JONA RA
            0x07F4..=0x07F5, // Lm   [2] NKO HIGH TONE APOSTROPHE..NKO LOW TONE APOSTROPHE
            0x07FA..=0x07FA, // Lm       NKO LAJANYALAN
            0x0800..=0x0815, // Lo  [22] SAMARITAN LETTER ALAF..SAMARITAN LETTER TAAF
            0x081A..=0x081A, // Lm       SAMARITAN MODIFIER LETTER EPENTHETIC YUT
            0x0824..=0x0824, // Lm       SAMARITAN MODIFIER LETTER SHORT A
            0x0828..=0x0828, // Lm       SAMARITAN MODIFIER LETTER I
            0x0840..=0x0858, // Lo  [25] MANDAIC LETTER HALQA..MANDAIC LETTER AIN
            0x08A0..=0x08B4, // Lo  [21] ARABIC LETTER BEH WITH SMALL V BELOW..ARABIC LETTER KAF WITH DOT BELOW
            0x08B6..=0x08BD, // Lo   [8] ARABIC LETTER BEH WITH SMALL MEEM ABOVE..ARABIC LETTER AFRICAN NOON
            0x0904..=0x0939, // Lo  [54] DEVANAGARI LETTER SHORT A..DEVANAGARI LETTER HA
            0x093D..=0x093D, // Lo       DEVANAGARI SIGN AVAGRAHA
            0x0950..=0x0950, // Lo       DEVANAGARI OM
            0x0958..=0x0961, // Lo  [10] DEVANAGARI LETTER QA..DEVANAGARI LETTER VOCALIC LL
            0x0971..=0x0971, // Lm       DEVANAGARI SIGN HIGH SPACING DOT
            0x0972..=0x0980, // Lo  [15] DEVANAGARI LETTER CANDRA A..BENGALI ANJI
            0x0985..=0x098C, // Lo   [8] BENGALI LETTER A..BENGALI LETTER VOCALIC L
            0x098F..=0x0990, // Lo   [2] BENGALI LETTER E..BENGALI LETTER AI
            0x0993..=0x09A8, // Lo  [22] BENGALI LETTER O..BENGALI LETTER NA
            0x09AA..=0x09B0, // Lo   [7] BENGALI LETTER PA..BENGALI LETTER RA
            0x09B2..=0x09B2, // Lo       BENGALI LETTER LA
            0x09B6..=0x09B9, // Lo   [4] BENGALI LETTER SHA..BENGALI LETTER HA
            0x09BD..=0x09BD, // Lo       BENGALI SIGN AVAGRAHA
            0x09CE..=0x09CE, // Lo       BENGALI LETTER KHANDA TA
            0x09DC..=0x09DD, // Lo   [2] BENGALI LETTER RRA..BENGALI LETTER RHA
            0x09DF..=0x09E1, // Lo   [3] BENGALI LETTER YYA..BENGALI LETTER VOCALIC LL
            0x09F0..=0x09F1, // Lo   [2] BENGALI LETTER RA WITH MIDDLE DIAGONAL..BENGALI LETTER RA WITH LOWER DIAGONAL
            0x0A05..=0x0A0A, // Lo   [6] GURMUKHI LETTER A..GURMUKHI LETTER UU
            0x0A0F..=0x0A10, // Lo   [2] GURMUKHI LETTER EE..GURMUKHI LETTER AI
            0x0A13..=0x0A28, // Lo  [22] GURMUKHI LETTER OO..GURMUKHI LETTER NA
            0x0A2A..=0x0A30, // Lo   [7] GURMUKHI LETTER PA..GURMUKHI LETTER RA
            0x0A32..=0x0A33, // Lo   [2] GURMUKHI LETTER LA..GURMUKHI LETTER LLA
            0x0A35..=0x0A36, // Lo   [2] GURMUKHI LETTER VA..GURMUKHI LETTER SHA
            0x0A38..=0x0A39, // Lo   [2] GURMUKHI LETTER SA..GURMUKHI LETTER HA
            0x0A59..=0x0A5C, // Lo   [4] GURMUKHI LETTER KHHA..GURMUKHI LETTER RRA
            0x0A5E..=0x0A5E, // Lo       GURMUKHI LETTER FA
            0x0A72..=0x0A74, // Lo   [3] GURMUKHI IRI..GURMUKHI EK ONKAR
            0x0A85..=0x0A8D, // Lo   [9] GUJARATI LETTER A..GUJARATI VOWEL CANDRA E
            0x0A8F..=0x0A91, // Lo   [3] GUJARATI LETTER E..GUJARATI VOWEL CANDRA O
            0x0A93..=0x0AA8, // Lo  [22] GUJARATI LETTER O..GUJARATI LETTER NA
            0x0AAA..=0x0AB0, // Lo   [7] GUJARATI LETTER PA..GUJARATI LETTER RA
            0x0AB2..=0x0AB3, // Lo   [2] GUJARATI LETTER LA..GUJARATI LETTER LLA
            0x0AB5..=0x0AB9, // Lo   [5] GUJARATI LETTER VA..GUJARATI LETTER HA
            0x0ABD..=0x0ABD, // Lo       GUJARATI SIGN AVAGRAHA
            0x0AD0..=0x0AD0, // Lo       GUJARATI OM
            0x0AE0..=0x0AE1, // Lo   [2] GUJARATI LETTER VOCALIC RR..GUJARATI LETTER VOCALIC LL
            0x0AF9..=0x0AF9, // Lo       GUJARATI LETTER ZHA
            0x0B05..=0x0B0C, // Lo   [8] ORIYA LETTER A..ORIYA LETTER VOCALIC L
            0x0B0F..=0x0B10, // Lo   [2] ORIYA LETTER E..ORIYA LETTER AI
            0x0B13..=0x0B28, // Lo  [22] ORIYA LETTER O..ORIYA LETTER NA
            0x0B2A..=0x0B30, // Lo   [7] ORIYA LETTER PA..ORIYA LETTER RA
            0x0B32..=0x0B33, // Lo   [2] ORIYA LETTER LA..ORIYA LETTER LLA
            0x0B35..=0x0B39, // Lo   [5] ORIYA LETTER VA..ORIYA LETTER HA
            0x0B3D..=0x0B3D, // Lo       ORIYA SIGN AVAGRAHA
            0x0B5C..=0x0B5D, // Lo   [2] ORIYA LETTER RRA..ORIYA LETTER RHA
            0x0B5F..=0x0B61, // Lo   [3] ORIYA LETTER YYA..ORIYA LETTER VOCALIC LL
            0x0B71..=0x0B71, // Lo       ORIYA LETTER WA
            0x0B83..=0x0B83, // Lo       TAMIL SIGN VISARGA
            0x0B85..=0x0B8A, // Lo   [6] TAMIL LETTER A..TAMIL LETTER UU
            0x0B8E..=0x0B90, // Lo   [3] TAMIL LETTER E..TAMIL LETTER AI
            0x0B92..=0x0B95, // Lo   [4] TAMIL LETTER O..TAMIL LETTER KA
            0x0B99..=0x0B9A, // Lo   [2] TAMIL LETTER NGA..TAMIL LETTER CA
            0x0B9C..=0x0B9C, // Lo       TAMIL LETTER JA
            0x0B9E..=0x0B9F, // Lo   [2] TAMIL LETTER NYA..TAMIL LETTER TTA
            0x0BA3..=0x0BA4, // Lo   [2] TAMIL LETTER NNA..TAMIL LETTER TA
            0x0BA8..=0x0BAA, // Lo   [3] TAMIL LETTER NA..TAMIL LETTER PA
            0x0BAE..=0x0BB9, // Lo  [12] TAMIL LETTER MA..TAMIL LETTER HA
            0x0BD0..=0x0BD0, // Lo       TAMIL OM
            0x0C05..=0x0C0C, // Lo   [8] TELUGU LETTER A..TELUGU LETTER VOCALIC L
            0x0C0E..=0x0C10, // Lo   [3] TELUGU LETTER E..TELUGU LETTER AI
            0x0C12..=0x0C28, // Lo  [23] TELUGU LETTER O..TELUGU LETTER NA
            0x0C2A..=0x0C39, // Lo  [16] TELUGU LETTER PA..TELUGU LETTER HA
            0x0C3D..=0x0C3D, // Lo       TELUGU SIGN AVAGRAHA
            0x0C58..=0x0C5A, // Lo   [3] TELUGU LETTER TSA..TELUGU LETTER RRRA
            0x0C60..=0x0C61, // Lo   [2] TELUGU LETTER VOCALIC RR..TELUGU LETTER VOCALIC LL
            0x0C80..=0x0C80, // Lo       KANNADA SIGN SPACING CANDRABINDU
            0x0C85..=0x0C8C, // Lo   [8] KANNADA LETTER A..KANNADA LETTER VOCALIC L
            0x0C8E..=0x0C90, // Lo   [3] KANNADA LETTER E..KANNADA LETTER AI
            0x0C92..=0x0CA8, // Lo  [23] KANNADA LETTER O..KANNADA LETTER NA
            0x0CAA..=0x0CB3, // Lo  [10] KANNADA LETTER PA..KANNADA LETTER LLA
            0x0CB5..=0x0CB9, // Lo   [5] KANNADA LETTER VA..KANNADA LETTER HA
            0x0CBD..=0x0CBD, // Lo       KANNADA SIGN AVAGRAHA
            0x0CDE..=0x0CDE, // Lo       KANNADA LETTER FA
            0x0CE0..=0x0CE1, // Lo   [2] KANNADA LETTER VOCALIC RR..KANNADA LETTER VOCALIC LL
            0x0CF1..=0x0CF2, // Lo   [2] KANNADA SIGN JIHVAMULIYA..KANNADA SIGN UPADHMANIYA
            0x0D05..=0x0D0C, // Lo   [8] MALAYALAM LETTER A..MALAYALAM LETTER VOCALIC L
            0x0D0E..=0x0D10, // Lo   [3] MALAYALAM LETTER E..MALAYALAM LETTER AI
            0x0D12..=0x0D3A, // Lo  [41] MALAYALAM LETTER O..MALAYALAM LETTER TTTA
            0x0D3D..=0x0D3D, // Lo       MALAYALAM SIGN AVAGRAHA
            0x0D4E..=0x0D4E, // Lo       MALAYALAM LETTER DOT REPH
            0x0D54..=0x0D56, // Lo   [3] MALAYALAM LETTER CHILLU M..MALAYALAM LETTER CHILLU LLL
            0x0D5F..=0x0D61, // Lo   [3] MALAYALAM LETTER ARCHAIC II..MALAYALAM LETTER VOCALIC LL
            0x0D7A..=0x0D7F, // Lo   [6] MALAYALAM LETTER CHILLU NN..MALAYALAM LETTER CHILLU K
            0x0D85..=0x0D96, // Lo  [18] SINHALA LETTER AYANNA..SINHALA LETTER AUYANNA
            0x0D9A..=0x0DB1, // Lo  [24] SINHALA LETTER ALPAPRAANA KAYANNA..SINHALA LETTER DANTAJA NAYANNA
            0x0DB3..=0x0DBB, // Lo   [9] SINHALA LETTER SANYAKA DAYANNA..SINHALA LETTER RAYANNA
            0x0DBD..=0x0DBD, // Lo       SINHALA LETTER DANTAJA LAYANNA
            0x0DC0..=0x0DC6, // Lo   [7] SINHALA LETTER VAYANNA..SINHALA LETTER FAYANNA
            0x0E01..=0x0E30, // Lo  [48] THAI CHARACTER KO KAI..THAI CHARACTER SARA A
            0x0E32..=0x0E33, // Lo   [2] THAI CHARACTER SARA AA..THAI CHARACTER SARA AM
            0x0E40..=0x0E45, // Lo   [6] THAI CHARACTER SARA E..THAI CHARACTER LAKKHANGYAO
            0x0E46..=0x0E46, // Lm       THAI CHARACTER MAIYAMOK
            0x0E81..=0x0E82, // Lo   [2] LAO LETTER KO..LAO LETTER KHO SUNG
            0x0E84..=0x0E84, // Lo       LAO LETTER KHO TAM
            0x0E87..=0x0E88, // Lo   [2] LAO LETTER NGO..LAO LETTER CO
            0x0E8A..=0x0E8A, // Lo       LAO LETTER SO TAM
            0x0E8D..=0x0E8D, // Lo       LAO LETTER NYO
            0x0E94..=0x0E97, // Lo   [4] LAO LETTER DO..LAO LETTER THO TAM
            0x0E99..=0x0E9F, // Lo   [7] LAO LETTER NO..LAO LETTER FO SUNG
            0x0EA1..=0x0EA3, // Lo   [3] LAO LETTER MO..LAO LETTER LO LING
            0x0EA5..=0x0EA5, // Lo       LAO LETTER LO LOOT
            0x0EA7..=0x0EA7, // Lo       LAO LETTER WO
            0x0EAA..=0x0EAB, // Lo   [2] LAO LETTER SO SUNG..LAO LETTER HO SUNG
            0x0EAD..=0x0EB0, // Lo   [4] LAO LETTER O..LAO VOWEL SIGN A
            0x0EB2..=0x0EB3, // Lo   [2] LAO VOWEL SIGN AA..LAO VOWEL SIGN AM
            0x0EBD..=0x0EBD, // Lo       LAO SEMIVOWEL SIGN NYO
            0x0EC0..=0x0EC4, // Lo   [5] LAO VOWEL SIGN E..LAO VOWEL SIGN AI
            0x0EC6..=0x0EC6, // Lm       LAO KO LA
            0x0EDC..=0x0EDF, // Lo   [4] LAO HO NO..LAO LETTER KHMU NYO
            0x0F00..=0x0F00, // Lo       TIBETAN SYLLABLE OM
            0x0F40..=0x0F47, // Lo   [8] TIBETAN LETTER KA..TIBETAN LETTER JA
            0x0F49..=0x0F6C, // Lo  [36] TIBETAN LETTER NYA..TIBETAN LETTER RRA
            0x0F88..=0x0F8C, // Lo   [5] TIBETAN SIGN LCE TSA CAN..TIBETAN SIGN INVERTED MCHU CAN
            0x1000..=0x102A, // Lo  [43] MYANMAR LETTER KA..MYANMAR LETTER AU
            0x103F..=0x103F, // Lo       MYANMAR LETTER GREAT SA
            0x1050..=0x1055, // Lo   [6] MYANMAR LETTER SHA..MYANMAR LETTER VOCALIC LL
            0x105A..=0x105D, // Lo   [4] MYANMAR LETTER MON NGA..MYANMAR LETTER MON BBE
            0x1061..=0x1061, // Lo       MYANMAR LETTER SGAW KAREN SHA
            0x1065..=0x1066, // Lo   [2] MYANMAR LETTER WESTERN PWO KAREN THA..MYANMAR LETTER WESTERN PWO KAREN PWA
            0x106E..=0x1070, // Lo   [3] MYANMAR LETTER EASTERN PWO KAREN NNA..MYANMAR LETTER EASTERN PWO KAREN GHWA
            0x1075..=0x1081, // Lo  [13] MYANMAR LETTER SHAN KA..MYANMAR LETTER SHAN HA
            0x108E..=0x108E, // Lo       MYANMAR LETTER RUMAI PALAUNG FA
            0x10A0..=0x10C5, // L&  [38] GEORGIAN CAPITAL LETTER AN..GEORGIAN CAPITAL LETTER HOE
            0x10C7..=0x10C7, // L&       GEORGIAN CAPITAL LETTER YN
            0x10CD..=0x10CD, // L&       GEORGIAN CAPITAL LETTER AEN
            0x10D0..=0x10FA, // Lo  [43] GEORGIAN LETTER AN..GEORGIAN LETTER AIN
            0x10FC..=0x10FC, // Lm       MODIFIER LETTER GEORGIAN NAR
            0x10FD..=0x1248, // Lo [332] GEORGIAN LETTER AEN..ETHIOPIC SYLLABLE QWA
            0x124A..=0x124D, // Lo   [4] ETHIOPIC SYLLABLE QWI..ETHIOPIC SYLLABLE QWE
            0x1250..=0x1256, // Lo   [7] ETHIOPIC SYLLABLE QHA..ETHIOPIC SYLLABLE QHO
            0x1258..=0x1258, // Lo       ETHIOPIC SYLLABLE QHWA
            0x125A..=0x125D, // Lo   [4] ETHIOPIC SYLLABLE QHWI..ETHIOPIC SYLLABLE QHWE
            0x1260..=0x1288, // Lo  [41] ETHIOPIC SYLLABLE BA..ETHIOPIC SYLLABLE XWA
            0x128A..=0x128D, // Lo   [4] ETHIOPIC SYLLABLE XWI..ETHIOPIC SYLLABLE XWE
            0x1290..=0x12B0, // Lo  [33] ETHIOPIC SYLLABLE NA..ETHIOPIC SYLLABLE KWA
            0x12B2..=0x12B5, // Lo   [4] ETHIOPIC SYLLABLE KWI..ETHIOPIC SYLLABLE KWE
            0x12B8..=0x12BE, // Lo   [7] ETHIOPIC SYLLABLE KXA..ETHIOPIC SYLLABLE KXO
            0x12C0..=0x12C0, // Lo       ETHIOPIC SYLLABLE KXWA
            0x12C2..=0x12C5, // Lo   [4] ETHIOPIC SYLLABLE KXWI..ETHIOPIC SYLLABLE KXWE
            0x12C8..=0x12D6, // Lo  [15] ETHIOPIC SYLLABLE WA..ETHIOPIC SYLLABLE PHARYNGEAL O
            0x12D8..=0x1310, // Lo  [57] ETHIOPIC SYLLABLE ZA..ETHIOPIC SYLLABLE GWA
            0x1312..=0x1315, // Lo   [4] ETHIOPIC SYLLABLE GWI..ETHIOPIC SYLLABLE GWE
            0x1318..=0x135A, // Lo  [67] ETHIOPIC SYLLABLE GGA..ETHIOPIC SYLLABLE FYA
            0x1380..=0x138F, // Lo  [16] ETHIOPIC SYLLABLE SEBATBEIT MWA..ETHIOPIC SYLLABLE PWE
            0x13A0..=0x13F5, // L&  [86] CHEROKEE LETTER A..CHEROKEE LETTER MV
            0x13F8..=0x13FD, // L&   [6] CHEROKEE SMALL LETTER YE..CHEROKEE SMALL LETTER MV
            0x1401..=0x166C, // Lo [620] CANADIAN SYLLABICS E..CANADIAN SYLLABICS CARRIER TTSA
            0x166F..=0x167F, // Lo  [17] CANADIAN SYLLABICS QAI..CANADIAN SYLLABICS BLACKFOOT W
            0x1681..=0x169A, // Lo  [26] OGHAM LETTER BEITH..OGHAM LETTER PEITH
            0x16A0..=0x16EA, // Lo  [75] RUNIC LETTER FEHU FEOH FE F..RUNIC LETTER X
            0x16EE..=0x16F0, // Nl   [3] RUNIC ARLAUG SYMBOL..RUNIC BELGTHOR SYMBOL
            0x16F1..=0x16F8, // Lo   [8] RUNIC LETTER K..RUNIC LETTER FRANKS CASKET AESC
            0x1700..=0x170C, // Lo  [13] TAGALOG LETTER A..TAGALOG LETTER YA
            0x170E..=0x1711, // Lo   [4] TAGALOG LETTER LA..TAGALOG LETTER HA
            0x1720..=0x1731, // Lo  [18] HANUNOO LETTER A..HANUNOO LETTER HA
            0x1740..=0x1751, // Lo  [18] BUHID LETTER A..BUHID LETTER HA
            0x1760..=0x176C, // Lo  [13] TAGBANWA LETTER A..TAGBANWA LETTER YA
            0x176E..=0x1770, // Lo   [3] TAGBANWA LETTER LA..TAGBANWA LETTER SA
            0x1780..=0x17B3, // Lo  [52] KHMER LETTER KA..KHMER INDEPENDENT VOWEL QAU
            0x17D7..=0x17D7, // Lm       KHMER SIGN LEK TOO
            0x17DC..=0x17DC, // Lo       KHMER SIGN AVAKRAHASANYA
            0x1820..=0x1842, // Lo  [35] MONGOLIAN LETTER A..MONGOLIAN LETTER CHI
            0x1843..=0x1843, // Lm       MONGOLIAN LETTER TODO LONG VOWEL SIGN
            0x1844..=0x1877, // Lo  [52] MONGOLIAN LETTER TODO E..MONGOLIAN LETTER MANCHU ZHA
            0x1880..=0x1884, // Lo   [5] MONGOLIAN LETTER ALI GALI ANUSVARA ONE..MONGOLIAN LETTER ALI GALI INVERTED UBADAMA
            0x1885..=0x1886, // Mn   [2] MONGOLIAN LETTER ALI GALI BALUDA..MONGOLIAN LETTER ALI GALI THREE BALUDA
            0x1887..=0x18A8, // Lo  [34] MONGOLIAN LETTER ALI GALI A..MONGOLIAN LETTER MANCHU ALI GALI BHA
            0x18AA..=0x18AA, // Lo       MONGOLIAN LETTER MANCHU ALI GALI LHA
            0x18B0..=0x18F5, // Lo  [70] CANADIAN SYLLABICS OY..CANADIAN SYLLABICS CARRIER DENTAL S
            0x1900..=0x191E, // Lo  [31] LIMBU VOWEL-CARRIER LETTER..LIMBU LETTER TRA
            0x1950..=0x196D, // Lo  [30] TAI LE LETTER KA..TAI LE LETTER AI
            0x1970..=0x1974, // Lo   [5] TAI LE LETTER TONE-2..TAI LE LETTER TONE-6
            0x1980..=0x19AB, // Lo  [44] NEW TAI LUE LETTER HIGH QA..NEW TAI LUE LETTER LOW SUA
            0x19B0..=0x19C9, // Lo  [26] NEW TAI LUE VOWEL SIGN VOWEL SHORTENER..NEW TAI LUE TONE MARK-2
            0x1A00..=0x1A16, // Lo  [23] BUGINESE LETTER KA..BUGINESE LETTER HA
            0x1A20..=0x1A54, // Lo  [53] TAI THAM LETTER HIGH KA..TAI THAM LETTER GREAT SA
            0x1AA7..=0x1AA7, // Lm       TAI THAM SIGN MAI YAMOK
            0x1B05..=0x1B33, // Lo  [47] BALINESE LETTER AKARA..BALINESE LETTER HA
            0x1B45..=0x1B4B, // Lo   [7] BALINESE LETTER KAF SASAK..BALINESE LETTER ASYURA SASAK
            0x1B83..=0x1BA0, // Lo  [30] SUNDANESE LETTER A..SUNDANESE LETTER HA
            0x1BAE..=0x1BAF, // Lo   [2] SUNDANESE LETTER KHA..SUNDANESE LETTER SYA
            0x1BBA..=0x1BE5, // Lo  [44] SUNDANESE AVAGRAHA..BATAK LETTER U
            0x1C00..=0x1C23, // Lo  [36] LEPCHA LETTER KA..LEPCHA LETTER A
            0x1C4D..=0x1C4F, // Lo   [3] LEPCHA LETTER TTA..LEPCHA LETTER DDA
            0x1C5A..=0x1C77, // Lo  [30] OL CHIKI LETTER LA..OL CHIKI LETTER OH
            0x1C78..=0x1C7D, // Lm   [6] OL CHIKI MU TTUDDAG..OL CHIKI AHAD
            0x1C80..=0x1C88, // L&   [9] CYRILLIC SMALL LETTER ROUNDED VE..CYRILLIC SMALL LETTER UNBLENDED UK
            0x1CE9..=0x1CEC, // Lo   [4] VEDIC SIGN ANUSVARA ANTARGOMUKHA..VEDIC SIGN ANUSVARA VAMAGOMUKHA WITH TAIL
            0x1CEE..=0x1CF1, // Lo   [4] VEDIC SIGN HEXIFORM LONG ANUSVARA..VEDIC SIGN ANUSVARA UBHAYATO MUKHA
            0x1CF5..=0x1CF6, // Lo   [2] VEDIC SIGN JIHVAMULIYA..VEDIC SIGN UPADHMANIYA
            0x1D00..=0x1D2B, // L&  [44] LATIN LETTER SMALL CAPITAL A..CYRILLIC LETTER SMALL CAPITAL EL
            0x1D2C..=0x1D6A, // Lm  [63] MODIFIER LETTER CAPITAL A..GREEK SUBSCRIPT SMALL LETTER CHI
            0x1D6B..=0x1D77, // L&  [13] LATIN SMALL LETTER UE..LATIN SMALL LETTER TURNED G
            0x1D78..=0x1D78, // Lm       MODIFIER LETTER CYRILLIC EN
            0x1D79..=0x1D9A, // L&  [34] LATIN SMALL LETTER INSULAR G..LATIN SMALL LETTER EZH WITH RETROFLEX HOOK
            0x1D9B..=0x1DBF, // Lm  [37] MODIFIER LETTER SMALL TURNED ALPHA..MODIFIER LETTER SMALL THETA
            0x1E00..=0x1F15, // L& [278] LATIN CAPITAL LETTER A WITH RING BELOW..GREEK SMALL LETTER EPSILON WITH DASIA AND OXIA
            0x1F18..=0x1F1D, // L&   [6] GREEK CAPITAL LETTER EPSILON WITH PSILI..GREEK CAPITAL LETTER EPSILON WITH DASIA AND OXIA
            0x1F20..=0x1F45, // L&  [38] GREEK SMALL LETTER ETA WITH PSILI..GREEK SMALL LETTER OMICRON WITH DASIA AND OXIA
            0x1F48..=0x1F4D, // L&   [6] GREEK CAPITAL LETTER OMICRON WITH PSILI..GREEK CAPITAL LETTER OMICRON WITH DASIA AND OXIA
            0x1F50..=0x1F57, // L&   [8] GREEK SMALL LETTER UPSILON WITH PSILI..GREEK SMALL LETTER UPSILON WITH DASIA AND PERISPOMENI
            0x1F59..=0x1F59, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA
            0x1F5B..=0x1F5B, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND VARIA
            0x1F5D..=0x1F5D, // L&       GREEK CAPITAL LETTER UPSILON WITH DASIA AND OXIA
            0x1F5F..=0x1F7D, // L&  [31] GREEK CAPITAL LETTER UPSILON WITH DASIA AND PERISPOMENI..GREEK SMALL LETTER OMEGA WITH OXIA
            0x1F80..=0x1FB4, // L&  [53] GREEK SMALL LETTER ALPHA WITH PSILI AND YPOGEGRAMMENI..GREEK SMALL LETTER ALPHA WITH OXIA AND YPOGEGRAMMENI
            0x1FB6..=0x1FBC, // L&   [7] GREEK SMALL LETTER ALPHA WITH PERISPOMENI..GREEK CAPITAL LETTER ALPHA WITH PROSGEGRAMMENI
            0x1FBE..=0x1FBE, // L&       GREEK PROSGEGRAMMENI
            0x1FC2..=0x1FC4, // L&   [3] GREEK SMALL LETTER ETA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER ETA WITH OXIA AND YPOGEGRAMMENI
            0x1FC6..=0x1FCC, // L&   [7] GREEK SMALL LETTER ETA WITH PERISPOMENI..GREEK CAPITAL LETTER ETA WITH PROSGEGRAMMENI
            0x1FD0..=0x1FD3, // L&   [4] GREEK SMALL LETTER IOTA WITH VRACHY..GREEK SMALL LETTER IOTA WITH DIALYTIKA AND OXIA
            0x1FD6..=0x1FDB, // L&   [6] GREEK SMALL LETTER IOTA WITH PERISPOMENI..GREEK CAPITAL LETTER IOTA WITH OXIA
            0x1FE0..=0x1FEC, // L&  [13] GREEK SMALL LETTER UPSILON WITH VRACHY..GREEK CAPITAL LETTER RHO WITH DASIA
            0x1FF2..=0x1FF4, // L&   [3] GREEK SMALL LETTER OMEGA WITH VARIA AND YPOGEGRAMMENI..GREEK SMALL LETTER OMEGA WITH OXIA AND YPOGEGRAMMENI
            0x1FF6..=0x1FFC, // L&   [7] GREEK SMALL LETTER OMEGA WITH PERISPOMENI..GREEK CAPITAL LETTER OMEGA WITH PROSGEGRAMMENI
            0x2071..=0x2071, // Lm       SUPERSCRIPT LATIN SMALL LETTER I
            0x207F..=0x207F, // Lm       SUPERSCRIPT LATIN SMALL LETTER N
            0x2090..=0x209C, // Lm  [13] LATIN SUBSCRIPT SMALL LETTER A..LATIN SUBSCRIPT SMALL LETTER T
            0x2102..=0x2102, // L&       DOUBLE-STRUCK CAPITAL C
            0x2107..=0x2107, // L&       EULER CONSTANT
            0x210A..=0x2113, // L&  [10] SCRIPT SMALL G..SCRIPT SMALL L
            0x2115..=0x2115, // L&       DOUBLE-STRUCK CAPITAL N
            0x2118..=0x2118, // Sm       SCRIPT CAPITAL P
            0x2119..=0x211D, // L&   [5] DOUBLE-STRUCK CAPITAL P..DOUBLE-STRUCK CAPITAL R
            0x2124..=0x2124, // L&       DOUBLE-STRUCK CAPITAL Z
            0x2126..=0x2126, // L&       OHM SIGN
            0x2128..=0x2128, // L&       BLACK-LETTER CAPITAL Z
            0x212A..=0x212D, // L&   [4] KELVIN SIGN..BLACK-LETTER CAPITAL C
            0x212E..=0x212E, // So       ESTIMATED SYMBOL
            0x212F..=0x2134, // L&   [6] SCRIPT SMALL E..SCRIPT SMALL O
            0x2135..=0x2138, // Lo   [4] ALEF SYMBOL..DALET SYMBOL
            0x2139..=0x2139, // L&       INFORMATION SOURCE
            0x213C..=0x213F, // L&   [4] DOUBLE-STRUCK SMALL PI..DOUBLE-STRUCK CAPITAL PI
            0x2145..=0x2149, // L&   [5] DOUBLE-STRUCK ITALIC CAPITAL D..DOUBLE-STRUCK ITALIC SMALL J
            0x214E..=0x214E, // L&       TURNED SMALL F
            0x2160..=0x2182, // Nl  [35] ROMAN NUMERAL ONE..ROMAN NUMERAL TEN THOUSAND
            0x2183..=0x2184, // L&   [2] ROMAN NUMERAL REVERSED ONE HUNDRED..LATIN SMALL LETTER REVERSED C
            0x2185..=0x2188, // Nl   [4] ROMAN NUMERAL SIX LATE FORM..ROMAN NUMERAL ONE HUNDRED THOUSAND
            0x2C00..=0x2C2E, // L&  [47] GLAGOLITIC CAPITAL LETTER AZU..GLAGOLITIC CAPITAL LETTER LATINATE MYSLITE
            0x2C30..=0x2C5E, // L&  [47] GLAGOLITIC SMALL LETTER AZU..GLAGOLITIC SMALL LETTER LATINATE MYSLITE
            0x2C60..=0x2C7B, // L&  [28] LATIN CAPITAL LETTER L WITH DOUBLE BAR..LATIN LETTER SMALL CAPITAL TURNED E
            0x2C7C..=0x2C7D, // Lm   [2] LATIN SUBSCRIPT SMALL LETTER J..MODIFIER LETTER CAPITAL V
            0x2C7E..=0x2CE4, // L& [103] LATIN CAPITAL LETTER S WITH SWASH TAIL..COPTIC SYMBOL KAI
            0x2CEB..=0x2CEE, // L&   [4] COPTIC CAPITAL LETTER CRYPTOGRAMMIC SHEI..COPTIC SMALL LETTER CRYPTOGRAMMIC GANGIA
            0x2CF2..=0x2CF3, // L&   [2] COPTIC CAPITAL LETTER BOHAIRIC KHEI..COPTIC SMALL LETTER BOHAIRIC KHEI
            0x2D00..=0x2D25, // L&  [38] GEORGIAN SMALL LETTER AN..GEORGIAN SMALL LETTER HOE
            0x2D27..=0x2D27, // L&       GEORGIAN SMALL LETTER YN
            0x2D2D..=0x2D2D, // L&       GEORGIAN SMALL LETTER AEN
            0x2D30..=0x2D67, // Lo  [56] TIFINAGH LETTER YA..TIFINAGH LETTER YO
            0x2D6F..=0x2D6F, // Lm       TIFINAGH MODIFIER LETTER LABIALIZATION MARK
            0x2D80..=0x2D96, // Lo  [23] ETHIOPIC SYLLABLE LOA..ETHIOPIC SYLLABLE GGWE
            0x2DA0..=0x2DA6, // Lo   [7] ETHIOPIC SYLLABLE SSA..ETHIOPIC SYLLABLE SSO
            0x2DA8..=0x2DAE, // Lo   [7] ETHIOPIC SYLLABLE CCA..ETHIOPIC SYLLABLE CCO
            0x2DB0..=0x2DB6, // Lo   [7] ETHIOPIC SYLLABLE ZZA..ETHIOPIC SYLLABLE ZZO
            0x2DB8..=0x2DBE, // Lo   [7] ETHIOPIC SYLLABLE CCHA..ETHIOPIC SYLLABLE CCHO
            0x2DC0..=0x2DC6, // Lo   [7] ETHIOPIC SYLLABLE QYA..ETHIOPIC SYLLABLE QYO
            0x2DC8..=0x2DCE, // Lo   [7] ETHIOPIC SYLLABLE KYA..ETHIOPIC SYLLABLE KYO
            0x2DD0..=0x2DD6, // Lo   [7] ETHIOPIC SYLLABLE XYA..ETHIOPIC SYLLABLE XYO
            0x2DD8..=0x2DDE, // Lo   [7] ETHIOPIC SYLLABLE GYA..ETHIOPIC SYLLABLE GYO
            0x3005..=0x3005, // Lm       IDEOGRAPHIC ITERATION MARK
            0x3006..=0x3006, // Lo       IDEOGRAPHIC CLOSING MARK
            0x3007..=0x3007, // Nl       IDEOGRAPHIC NUMBER ZERO
            0x3021..=0x3029, // Nl   [9] HANGZHOU NUMERAL ONE..HANGZHOU NUMERAL NINE
            0x3031..=0x3035, // Lm   [5] VERTICAL KANA REPEAT MARK..VERTICAL KANA REPEAT MARK LOWER HALF
            0x3038..=0x303A, // Nl   [3] HANGZHOU NUMERAL TEN..HANGZHOU NUMERAL THIRTY
            0x303B..=0x303B, // Lm       VERTICAL IDEOGRAPHIC ITERATION MARK
            0x303C..=0x303C, // Lo       MASU MARK
            0x3041..=0x3096, // Lo  [86] HIRAGANA LETTER SMALL A..HIRAGANA LETTER SMALL KE
            0x309B..=0x309C, // Sk   [2] KATAKANA-HIRAGANA VOICED SOUND MARK..KATAKANA-HIRAGANA SEMI-VOICED SOUND MARK
            0x309D..=0x309E, // Lm   [2] HIRAGANA ITERATION MARK..HIRAGANA VOICED ITERATION MARK
            0x309F..=0x309F, // Lo       HIRAGANA DIGRAPH YORI
            0x30A1..=0x30FA, // Lo  [90] KATAKANA LETTER SMALL A..KATAKANA LETTER VO
            0x30FC..=0x30FE, // Lm   [3] KATAKANA-HIRAGANA PROLONGED SOUND MARK..KATAKANA VOICED ITERATION MARK
            0x30FF..=0x30FF, // Lo       KATAKANA DIGRAPH KOTO
            0x3105..=0x312D, // Lo  [41] BOPOMOFO LETTER B..BOPOMOFO LETTER IH
            0x3131..=0x318E, // Lo  [94] HANGUL LETTER KIYEOK..HANGUL LETTER ARAEAE
            0x31A0..=0x31BA, // Lo  [27] BOPOMOFO LETTER BU..BOPOMOFO LETTER ZY
            0x31F0..=0x31FF, // Lo  [16] KATAKANA LETTER SMALL KU..KATAKANA LETTER SMALL RO
            0x3400..=0x4DB5, // Lo [6582] CJK UNIFIED IDEOGRAPH-3400..CJK UNIFIED IDEOGRAPH-4DB5
            0x4E00..=0x9FD5, // Lo [20950] CJK UNIFIED IDEOGRAPH-4E00..CJK UNIFIED IDEOGRAPH-9FD5
            0xA000..=0xA014, // Lo  [21] YI SYLLABLE IT..YI SYLLABLE E
            0xA015..=0xA015, // Lm       YI SYLLABLE WU
            0xA016..=0xA48C, // Lo [1143] YI SYLLABLE BIT..YI SYLLABLE YYR
            0xA4D0..=0xA4F7, // Lo  [40] LISU LETTER BA..LISU LETTER OE
            0xA4F8..=0xA4FD, // Lm   [6] LISU LETTER TONE MYA TI..LISU LETTER TONE MYA JEU
            0xA500..=0xA60B, // Lo [268] VAI SYLLABLE EE..VAI SYLLABLE NG
            0xA60C..=0xA60C, // Lm       VAI SYLLABLE LENGTHENER
            0xA610..=0xA61F, // Lo  [16] VAI SYLLABLE NDOLE FA..VAI SYMBOL JONG
            0xA62A..=0xA62B, // Lo   [2] VAI SYLLABLE NDOLE MA..VAI SYLLABLE NDOLE DO
            0xA640..=0xA66D, // L&  [46] CYRILLIC CAPITAL LETTER ZEMLYA..CYRILLIC SMALL LETTER DOUBLE MONOCULAR O
            0xA66E..=0xA66E, // Lo       CYRILLIC LETTER MULTIOCULAR O
            0xA67F..=0xA67F, // Lm       CYRILLIC PAYEROK
            0xA680..=0xA69B, // L&  [28] CYRILLIC CAPITAL LETTER DWE..CYRILLIC SMALL LETTER CROSSED O
            0xA69C..=0xA69D, // Lm   [2] MODIFIER LETTER CYRILLIC HARD SIGN..MODIFIER LETTER CYRILLIC SOFT SIGN
            0xA6A0..=0xA6E5, // Lo  [70] BAMUM LETTER A..BAMUM LETTER KI
            0xA6E6..=0xA6EF, // Nl  [10] BAMUM LETTER MO..BAMUM LETTER KOGHOM
            0xA717..=0xA71F, // Lm   [9] MODIFIER LETTER DOT VERTICAL BAR..MODIFIER LETTER LOW INVERTED EXCLAMATION MARK
            0xA722..=0xA76F, // L&  [78] LATIN CAPITAL LETTER EGYPTOLOGICAL ALEF..LATIN SMALL LETTER CON
            0xA770..=0xA770, // Lm       MODIFIER LETTER US
            0xA771..=0xA787, // L&  [23] LATIN SMALL LETTER DUM..LATIN SMALL LETTER INSULAR T
            0xA788..=0xA788, // Lm       MODIFIER LETTER LOW CIRCUMFLEX ACCENT
            0xA78B..=0xA78E, // L&   [4] LATIN CAPITAL LETTER SALTILLO..LATIN SMALL LETTER L WITH RETROFLEX HOOK AND BELT
            0xA78F..=0xA78F, // Lo       LATIN LETTER SINOLOGICAL DOT
            0xA790..=0xA7AE, // L&  [31] LATIN CAPITAL LETTER N WITH DESCENDER..LATIN CAPITAL LETTER SMALL CAPITAL I
            0xA7B0..=0xA7B7, // L&   [8] LATIN CAPITAL LETTER TURNED K..LATIN SMALL LETTER OMEGA
            0xA7F7..=0xA7F7, // Lo       LATIN EPIGRAPHIC LETTER SIDEWAYS I
            0xA7F8..=0xA7F9, // Lm   [2] MODIFIER LETTER CAPITAL H WITH STROKE..MODIFIER LETTER SMALL LIGATURE OE
            0xA7FA..=0xA7FA, // L&       LATIN LETTER SMALL CAPITAL TURNED M
            0xA7FB..=0xA801, // Lo   [7] LATIN EPIGRAPHIC LETTER REVERSED F..SYLOTI NAGRI LETTER I
            0xA803..=0xA805, // Lo   [3] SYLOTI NAGRI LETTER U..SYLOTI NAGRI LETTER O
            0xA807..=0xA80A, // Lo   [4] SYLOTI NAGRI LETTER KO..SYLOTI NAGRI LETTER GHO
            0xA80C..=0xA822, // Lo  [23] SYLOTI NAGRI LETTER CO..SYLOTI NAGRI LETTER HO
            0xA840..=0xA873, // Lo  [52] PHAGS-PA LETTER KA..PHAGS-PA LETTER CANDRABINDU
            0xA882..=0xA8B3, // Lo  [50] SAURASHTRA LETTER A..SAURASHTRA LETTER LLA
            0xA8F2..=0xA8F7, // Lo   [6] DEVANAGARI SIGN SPACING CANDRABINDU..DEVANAGARI SIGN CANDRABINDU AVAGRAHA
            0xA8FB..=0xA8FB, // Lo       DEVANAGARI HEADSTROKE
            0xA8FD..=0xA8FD, // Lo       DEVANAGARI JAIN OM
            0xA90A..=0xA925, // Lo  [28] KAYAH LI LETTER KA..KAYAH LI LETTER OO
            0xA930..=0xA946, // Lo  [23] REJANG LETTER KA..REJANG LETTER A
            0xA960..=0xA97C, // Lo  [29] HANGUL CHOSEONG TIKEUT-MIEUM..HANGUL CHOSEONG SSANGYEORINHIEUH
            0xA984..=0xA9B2, // Lo  [47] JAVANESE LETTER A..JAVANESE LETTER HA
            0xA9CF..=0xA9CF, // Lm       JAVANESE PANGRANGKEP
            0xA9E0..=0xA9E4, // Lo   [5] MYANMAR LETTER SHAN GHA..MYANMAR LETTER SHAN BHA
            0xA9E6..=0xA9E6, // Lm       MYANMAR MODIFIER LETTER SHAN REDUPLICATION
            0xA9E7..=0xA9EF, // Lo   [9] MYANMAR LETTER TAI LAING NYA..MYANMAR LETTER TAI LAING NNA
            0xA9FA..=0xA9FE, // Lo   [5] MYANMAR LETTER TAI LAING LLA..MYANMAR LETTER TAI LAING BHA
            0xAA00..=0xAA28, // Lo  [41] CHAM LETTER A..CHAM LETTER HA
            0xAA40..=0xAA42, // Lo   [3] CHAM LETTER FINAL K..CHAM LETTER FINAL NG
            0xAA44..=0xAA4B, // Lo   [8] CHAM LETTER FINAL CH..CHAM LETTER FINAL SS
            0xAA60..=0xAA6F, // Lo  [16] MYANMAR LETTER KHAMTI GA..MYANMAR LETTER KHAMTI FA
            0xAA70..=0xAA70, // Lm       MYANMAR MODIFIER LETTER KHAMTI REDUPLICATION
            0xAA71..=0xAA76, // Lo   [6] MYANMAR LETTER KHAMTI XA..MYANMAR LOGOGRAM KHAMTI HM
            0xAA7A..=0xAA7A, // Lo       MYANMAR LETTER AITON RA
            0xAA7E..=0xAAAF, // Lo  [50] MYANMAR LETTER SHWE PALAUNG CHA..TAI VIET LETTER HIGH O
            0xAAB1..=0xAAB1, // Lo       TAI VIET VOWEL AA
            0xAAB5..=0xAAB6, // Lo   [2] TAI VIET VOWEL E..TAI VIET VOWEL O
            0xAAB9..=0xAABD, // Lo   [5] TAI VIET VOWEL UEA..TAI VIET VOWEL AN
            0xAAC0..=0xAAC0, // Lo       TAI VIET TONE MAI NUENG
            0xAAC2..=0xAAC2, // Lo       TAI VIET TONE MAI SONG
            0xAADB..=0xAADC, // Lo   [2] TAI VIET SYMBOL KON..TAI VIET SYMBOL NUENG
            0xAADD..=0xAADD, // Lm       TAI VIET SYMBOL SAM
            0xAAE0..=0xAAEA, // Lo  [11] MEETEI MAYEK LETTER E..MEETEI MAYEK LETTER SSA
            0xAAF2..=0xAAF2, // Lo       MEETEI MAYEK ANJI
            0xAAF3..=0xAAF4, // Lm   [2] MEETEI MAYEK SYLLABLE REPETITION MARK..MEETEI MAYEK WORD REPETITION MARK
            0xAB01..=0xAB06, // Lo   [6] ETHIOPIC SYLLABLE TTHU..ETHIOPIC SYLLABLE TTHO
            0xAB09..=0xAB0E, // Lo   [6] ETHIOPIC SYLLABLE DDHU..ETHIOPIC SYLLABLE DDHO
            0xAB11..=0xAB16, // Lo   [6] ETHIOPIC SYLLABLE DZU..ETHIOPIC SYLLABLE DZO
            0xAB20..=0xAB26, // Lo   [7] ETHIOPIC SYLLABLE CCHHA..ETHIOPIC SYLLABLE CCHHO
            0xAB28..=0xAB2E, // Lo   [7] ETHIOPIC SYLLABLE BBA..ETHIOPIC SYLLABLE BBO
            0xAB30..=0xAB5A, // L&  [43] LATIN SMALL LETTER BARRED ALPHA..LATIN SMALL LETTER Y WITH SHORT RIGHT LEG
            0xAB5C..=0xAB5F, // Lm   [4] MODIFIER LETTER SMALL HENG..MODIFIER LETTER SMALL U WITH LEFT HOOK
            0xAB60..=0xAB65, // L&   [6] LATIN SMALL LETTER SAKHA YAT..GREEK LETTER SMALL CAPITAL OMEGA
            0xAB70..=0xABBF, // L&  [80] CHEROKEE SMALL LETTER A..CHEROKEE SMALL LETTER YA
            0xABC0..=0xABE2, // Lo  [35] MEETEI MAYEK LETTER KOK..MEETEI MAYEK LETTER I LONSUM
            0xAC00..=0xD7A3, // Lo [11172] HANGUL SYLLABLE GA..HANGUL SYLLABLE HIH
            0xD7B0..=0xD7C6, // Lo  [23] HANGUL JUNGSEONG O-YEO..HANGUL JUNGSEONG ARAEA-E
            0xD7CB..=0xD7FB, // Lo  [49] HANGUL JONGSEONG NIEUN-RIEUL..HANGUL JONGSEONG PHIEUPH-THIEUTH
            0xF900..=0xFA6D, // Lo [366] CJK COMPATIBILITY IDEOGRAPH-F900..CJK COMPATIBILITY IDEOGRAPH-FA6D
            0xFA70..=0xFAD9, // Lo [106] CJK COMPATIBILITY IDEOGRAPH-FA70..CJK COMPATIBILITY IDEOGRAPH-FAD9
            0xFB00..=0xFB06, // L&   [7] LATIN SMALL LIGATURE FF..LATIN SMALL LIGATURE ST
            0xFB13..=0xFB17, // L&   [5] ARMENIAN SMALL LIGATURE MEN NOW..ARMENIAN SMALL LIGATURE MEN XEH
            0xFB1D..=0xFB1D, // Lo       HEBREW LETTER YOD WITH HIRIQ
            0xFB1F..=0xFB28, // Lo  [10] HEBREW LIGATURE YIDDISH YOD YOD PATAH..HEBREW LETTER WIDE TAV
            0xFB2A..=0xFB36, // Lo  [13] HEBREW LETTER SHIN WITH SHIN DOT..HEBREW LETTER ZAYIN WITH DAGESH
            0xFB38..=0xFB3C, // Lo   [5] HEBREW LETTER TET WITH DAGESH..HEBREW LETTER LAMED WITH DAGESH
            0xFB3E..=0xFB3E, // Lo       HEBREW LETTER MEM WITH DAGESH
            0xFB40..=0xFB41, // Lo   [2] HEBREW LETTER NUN WITH DAGESH..HEBREW LETTER SAMEKH WITH DAGESH
            0xFB43..=0xFB44, // Lo   [2] HEBREW LETTER FINAL PE WITH DAGESH..HEBREW LETTER PE WITH DAGESH
            0xFB46..=0xFBB1, // Lo [108] HEBREW LETTER TSADI WITH DAGESH..ARABIC LETTER YEH BARREE WITH HAMZA ABOVE FINAL FORM
            0xFBD3..=0xFD3D, // Lo [363] ARABIC LETTER NG ISOLATED FORM..ARABIC LIGATURE ALEF WITH FATHATAN ISOLATED FORM
            0xFD50..=0xFD8F, // Lo  [64] ARABIC LIGATURE TEH WITH JEEM WITH MEEM INITIAL FORM..ARABIC LIGATURE MEEM WITH KHAH WITH MEEM INITIAL FORM
            0xFD92..=0xFDC7, // Lo  [54] ARABIC LIGATURE MEEM WITH JEEM WITH KHAH INITIAL FORM..ARABIC LIGATURE NOON WITH JEEM WITH YEH FINAL FORM
            0xFDF0..=0xFDFB, // Lo  [12] ARABIC LIGATURE SALLA USED AS KORANIC STOP SIGN ISOLATED FORM..ARABIC LIGATURE JALLAJALALOUHOU
            0xFE70..=0xFE74, // Lo   [5] ARABIC FATHATAN ISOLATED FORM..ARABIC KASRATAN ISOLATED FORM
            0xFE76..=0xFEFC, // Lo [135] ARABIC FATHA ISOLATED FORM..ARABIC LIGATURE LAM WITH ALEF FINAL FORM
            0xFF21..=0xFF3A, // L&  [26] FULLWIDTH LATIN CAPITAL LETTER A..FULLWIDTH LATIN CAPITAL LETTER Z
            0xFF41..=0xFF5A, // L&  [26] FULLWIDTH LATIN SMALL LETTER A..FULLWIDTH LATIN SMALL LETTER Z
            0xFF66..=0xFF6F, // Lo  [10] HALFWIDTH KATAKANA LETTER WO..HALFWIDTH KATAKANA LETTER SMALL TU
            0xFF70..=0xFF70, // Lm       HALFWIDTH KATAKANA-HIRAGANA PROLONGED SOUND MARK
            0xFF71..=0xFF9D, // Lo  [45] HALFWIDTH KATAKANA LETTER A..HALFWIDTH KATAKANA LETTER N
            0xFF9E..=0xFF9F, // Lm   [2] HALFWIDTH KATAKANA VOICED SOUND MARK..HALFWIDTH KATAKANA SEMI-VOICED SOUND MARK
            0xFFA0..=0xFFBE, // Lo  [31] HALFWIDTH HANGUL FILLER..HALFWIDTH HANGUL LETTER HIEUH
            0xFFC2..=0xFFC7, // Lo   [6] HALFWIDTH HANGUL LETTER A..HALFWIDTH HANGUL LETTER E
            0xFFCA..=0xFFCF, // Lo   [6] HALFWIDTH HANGUL LETTER YEO..HALFWIDTH HANGUL LETTER OE
            0xFFD2..=0xFFD7, // Lo   [6] HALFWIDTH HANGUL LETTER YO..HALFWIDTH HANGUL LETTER YU
            0xFFDA..=0xFFDC, // Lo   [3] HALFWIDTH HANGUL LETTER EU..HALFWIDTH HANGUL LETTER I
        ][..]
    }

    fn r32() -> &'static [RangeInclusive<u32>] {
        &[
            0x10000..=0x1000B, // Lo  [12] LINEAR B SYLLABLE B008 A..LINEAR B SYLLABLE B046 JE
            0x1000D..=0x10026, // Lo  [26] LINEAR B SYLLABLE B036 JO..LINEAR B SYLLABLE B032 QO
            0x10028..=0x1003A, // Lo  [19] LINEAR B SYLLABLE B060 RA..LINEAR B SYLLABLE B042 WO
            0x1003C..=0x1003D, // Lo   [2] LINEAR B SYLLABLE B017 ZA..LINEAR B SYLLABLE B074 ZE
            0x1003F..=0x1004D, // Lo  [15] LINEAR B SYLLABLE B020 ZO..LINEAR B SYLLABLE B091 TWO
            0x10050..=0x1005D, // Lo  [14] LINEAR B SYMBOL B018..LINEAR B SYMBOL B089
            0x10080..=0x100FA, // Lo [123] LINEAR B IDEOGRAM B100 MAN..LINEAR B IDEOGRAM VESSEL B305
            0x10140..=0x10174, // Nl  [53] GREEK ACROPHONIC ATTIC ONE QUARTER..GREEK ACROPHONIC STRATIAN FIFTY MNAS
            0x101FD..=0x101FD, // Mn       PHAISTOS DISC SIGN COMBINING OBLIQUE STROKE
            0x10280..=0x1029C, // Lo  [29] LYCIAN LETTER A..LYCIAN LETTER X
            0x102A0..=0x102D0, // Lo  [49] CARIAN LETTER A..CARIAN LETTER UUU3
            0x102E0..=0x102E0, // Mn       COPTIC EPACT THOUSANDS MARK
            0x10300..=0x1031F, // Lo  [32] OLD ITALIC LETTER A..OLD ITALIC LETTER ESS
            0x10330..=0x10340, // Lo  [17] GOTHIC LETTER AHSA..GOTHIC LETTER PAIRTHRA
            0x10341..=0x10341, // Nl       GOTHIC LETTER NINETY
            0x10342..=0x10349, // Lo   [8] GOTHIC LETTER RAIDA..GOTHIC LETTER OTHAL
            0x1034A..=0x1034A, // Nl       GOTHIC LETTER NINE HUNDRED
            0x10350..=0x10375, // Lo  [38] OLD PERMIC LETTER AN..OLD PERMIC LETTER IA
            0x10376..=0x1037A, // Mn   [5] COMBINING OLD PERMIC LETTER AN..COMBINING OLD PERMIC LETTER SII
            0x10380..=0x1039D, // Lo  [30] UGARITIC LETTER ALPA..UGARITIC LETTER SSU
            0x103A0..=0x103C3, // Lo  [36] OLD PERSIAN SIGN A..OLD PERSIAN SIGN HA
            0x103C8..=0x103CF, // Lo   [8] OLD PERSIAN SIGN AURAMAZDAA..OLD PERSIAN SIGN BUUMISH
            0x103D1..=0x103D5, // Nl   [5] OLD PERSIAN NUMBER ONE..OLD PERSIAN NUMBER HUNDRED
            0x10400..=0x1044F, // L&  [80] DESERET CAPITAL LETTER LONG I..DESERET SMALL LETTER EW
            0x10450..=0x1049D, // Lo  [78] SHAVIAN LETTER PEEP..OSMANYA LETTER OO
            0x104A0..=0x104A9, // Nd  [10] OSMANYA DIGIT ZERO..OSMANYA DIGIT NINE
            0x104B0..=0x104D3, // L&  [36] OSAGE CAPITAL LETTER A..OSAGE CAPITAL LETTER ZHA
            0x104D8..=0x104FB, // L&  [36] OSAGE SMALL LETTER A..OSAGE SMALL LETTER ZHA
            0x10500..=0x10527, // Lo  [40] ELBASAN LETTER A..ELBASAN LETTER KHE
            0x10530..=0x10563, // Lo  [52] CAUCASIAN ALBANIAN LETTER ALT..CAUCASIAN ALBANIAN LETTER KIW
            0x10600..=0x10736, // Lo [311] LINEAR A SIGN AB001..LINEAR A SIGN A664
            0x10740..=0x10755, // Lo  [22] LINEAR A SIGN A701 A..LINEAR A SIGN A732 JE
            0x10760..=0x10767, // Lo   [8] LINEAR A SIGN A800..LINEAR A SIGN A807
            0x10800..=0x10805, // Lo   [6] CYPRIOT SYLLABLE A..CYPRIOT SYLLABLE JA
            0x10808..=0x10808, // Lo       CYPRIOT SYLLABLE JO
            0x1080A..=0x10835, // Lo  [44] CYPRIOT SYLLABLE KA..CYPRIOT SYLLABLE WO
            0x10837..=0x10838, // Lo   [2] CYPRIOT SYLLABLE XA..CYPRIOT SYLLABLE XE
            0x1083C..=0x1083C, // Lo       CYPRIOT SYLLABLE ZA
            0x1083F..=0x10855, // Lo  [23] CYPRIOT SYLLABLE ZO..IMPERIAL ARAMAIC LETTER TAW
            0x10860..=0x10876, // Lo  [23] PALMYRENE LETTER ALEPH..PALMYRENE LETTER TAW
            0x10880..=0x1089E, // Lo  [31] NABATAEAN LETTER FINAL ALEPH..NABATAEAN LETTER TAW
            0x108E0..=0x108F2, // Lo  [19] HATRAN LETTER ALEPH..HATRAN LETTER QOPH
            0x108F4..=0x108F5, // Lo   [2] HATRAN LETTER SHIN..HATRAN LETTER TAW
            0x10900..=0x10915, // Lo  [22] PHOENICIAN LETTER ALF..PHOENICIAN LETTER TAU
            0x10920..=0x10939, // Lo  [26] LYDIAN LETTER A..LYDIAN LETTER C
            0x10980..=0x109B7, // Lo  [56] MEROITIC HIEROGLYPHIC LETTER A..MEROITIC CURSIVE LETTER DA
            0x109BE..=0x109BF, // Lo   [2] MEROITIC CURSIVE LOGOGRAM RMT..MEROITIC CURSIVE LOGOGRAM IMN
            0x10A00..=0x10A00, // Lo       KHAROSHTHI LETTER A
            0x10A01..=0x10A03, // Mn   [3] KHAROSHTHI VOWEL SIGN I..KHAROSHTHI VOWEL SIGN VOCALIC R
            0x10A05..=0x10A06, // Mn   [2] KHAROSHTHI VOWEL SIGN E..KHAROSHTHI VOWEL SIGN O
            0x10A0C..=0x10A0F, // Mn   [4] KHAROSHTHI VOWEL LENGTH MARK..KHAROSHTHI SIGN VISARGA
            0x10A10..=0x10A13, // Lo   [4] KHAROSHTHI LETTER KA..KHAROSHTHI LETTER GHA
            0x10A15..=0x10A17, // Lo   [3] KHAROSHTHI LETTER CA..KHAROSHTHI LETTER JA
            0x10A19..=0x10A33, // Lo  [27] KHAROSHTHI LETTER NYA..KHAROSHTHI LETTER TTTHA
            0x10A38..=0x10A3A, // Mn   [3] KHAROSHTHI SIGN BAR ABOVE..KHAROSHTHI SIGN DOT BELOW
            0x10A3F..=0x10A3F, // Mn       KHAROSHTHI VIRAMA
            0x10A60..=0x10A7C, // Lo  [29] OLD SOUTH ARABIAN LETTER HE..OLD SOUTH ARABIAN LETTER THETH
            0x10A80..=0x10A9C, // Lo  [29] OLD NORTH ARABIAN LETTER HEH..OLD NORTH ARABIAN LETTER ZAH
            0x10AC0..=0x10AC7, // Lo   [8] MANICHAEAN LETTER ALEPH..MANICHAEAN LETTER WAW
            0x10AC9..=0x10AE4, // Lo  [28] MANICHAEAN LETTER ZAYIN..MANICHAEAN LETTER TAW
            0x10AE5..=0x10AE6, // Mn   [2] MANICHAEAN ABBREVIATION MARK ABOVE..MANICHAEAN ABBREVIATION MARK BELOW
            0x10B00..=0x10B35, // Lo  [54] AVESTAN LETTER A..AVESTAN LETTER HE
            0x10B40..=0x10B55, // Lo  [22] INSCRIPTIONAL PARTHIAN LETTER ALEPH..INSCRIPTIONAL PARTHIAN LETTER TAW
            0x10B60..=0x10B72, // Lo  [19] INSCRIPTIONAL PAHLAVI LETTER ALEPH..INSCRIPTIONAL PAHLAVI LETTER TAW
            0x10B80..=0x10B91, // Lo  [18] PSALTER PAHLAVI LETTER ALEPH..PSALTER PAHLAVI LETTER TAW
            0x10C00..=0x10C48, // Lo  [73] OLD TURKIC LETTER ORKHON A..OLD TURKIC LETTER ORKHON BASH
            0x10C80..=0x10CB2, // L&  [51] OLD HUNGARIAN CAPITAL LETTER A..OLD HUNGARIAN CAPITAL LETTER US
            0x10CC0..=0x10CF2, // L&  [51] OLD HUNGARIAN SMALL LETTER A..OLD HUNGARIAN SMALL LETTER US
            0x11000..=0x11000, // Mc       BRAHMI SIGN CANDRABINDU
            0x11001..=0x11001, // Mn       BRAHMI SIGN ANUSVARA
            0x11002..=0x11002, // Mc       BRAHMI SIGN VISARGA
            0x11003..=0x11037, // Lo  [53] BRAHMI SIGN JIHVAMULIYA..BRAHMI LETTER OLD TAMIL NNNA
            0x11038..=0x11046, // Mn  [15] BRAHMI VOWEL SIGN AA..BRAHMI VIRAMA
            0x11066..=0x1106F, // Nd  [10] BRAHMI DIGIT ZERO..BRAHMI DIGIT NINE
            0x1107F..=0x11081, // Mn   [3] BRAHMI NUMBER JOINER..KAITHI SIGN ANUSVARA
            0x11082..=0x11082, // Mc       KAITHI SIGN VISARGA
            0x11083..=0x110AF, // Lo  [45] KAITHI LETTER A..KAITHI LETTER HA
            0x110B0..=0x110B2, // Mc   [3] KAITHI VOWEL SIGN AA..KAITHI VOWEL SIGN II
            0x110B3..=0x110B6, // Mn   [4] KAITHI VOWEL SIGN U..KAITHI VOWEL SIGN AI
            0x110B7..=0x110B8, // Mc   [2] KAITHI VOWEL SIGN O..KAITHI VOWEL SIGN AU
            0x110B9..=0x110BA, // Mn   [2] KAITHI SIGN VIRAMA..KAITHI SIGN NUKTA
            0x110D0..=0x110E8, // Lo  [25] SORA SOMPENG LETTER SAH..SORA SOMPENG LETTER MAE
            0x110F0..=0x110F9, // Nd  [10] SORA SOMPENG DIGIT ZERO..SORA SOMPENG DIGIT NINE
            0x11100..=0x11102, // Mn   [3] CHAKMA SIGN CANDRABINDU..CHAKMA SIGN VISARGA
            0x11103..=0x11126, // Lo  [36] CHAKMA LETTER AA..CHAKMA LETTER HAA
            0x11127..=0x1112B, // Mn   [5] CHAKMA VOWEL SIGN A..CHAKMA VOWEL SIGN UU
            0x1112C..=0x1112C, // Mc       CHAKMA VOWEL SIGN E
            0x1112D..=0x11134, // Mn   [8] CHAKMA VOWEL SIGN AI..CHAKMA MAAYYAA
            0x11136..=0x1113F, // Nd  [10] CHAKMA DIGIT ZERO..CHAKMA DIGIT NINE
            0x11150..=0x11172, // Lo  [35] MAHAJANI LETTER A..MAHAJANI LETTER RRA
            0x11173..=0x11173, // Mn       MAHAJANI SIGN NUKTA
            0x11176..=0x11176, // Lo       MAHAJANI LIGATURE SHRI
            0x11180..=0x11181, // Mn   [2] SHARADA SIGN CANDRABINDU..SHARADA SIGN ANUSVARA
            0x11182..=0x11182, // Mc       SHARADA SIGN VISARGA
            0x11183..=0x111B2, // Lo  [48] SHARADA LETTER A..SHARADA LETTER HA
            0x111B3..=0x111B5, // Mc   [3] SHARADA VOWEL SIGN AA..SHARADA VOWEL SIGN II
            0x111B6..=0x111BE, // Mn   [9] SHARADA VOWEL SIGN U..SHARADA VOWEL SIGN O
            0x111BF..=0x111C0, // Mc   [2] SHARADA VOWEL SIGN AU..SHARADA SIGN VIRAMA
            0x111C1..=0x111C4, // Lo   [4] SHARADA SIGN AVAGRAHA..SHARADA OM
            0x111CA..=0x111CC, // Mn   [3] SHARADA SIGN NUKTA..SHARADA EXTRA SHORT VOWEL MARK
            0x111D0..=0x111D9, // Nd  [10] SHARADA DIGIT ZERO..SHARADA DIGIT NINE
            0x111DA..=0x111DA, // Lo       SHARADA EKAM
            0x111DC..=0x111DC, // Lo       SHARADA HEADSTROKE
            0x11200..=0x11211, // Lo  [18] KHOJKI LETTER A..KHOJKI LETTER JJA
            0x11213..=0x1122B, // Lo  [25] KHOJKI LETTER NYA..KHOJKI LETTER LLA
            0x1122C..=0x1122E, // Mc   [3] KHOJKI VOWEL SIGN AA..KHOJKI VOWEL SIGN II
            0x1122F..=0x11231, // Mn   [3] KHOJKI VOWEL SIGN U..KHOJKI VOWEL SIGN AI
            0x11232..=0x11233, // Mc   [2] KHOJKI VOWEL SIGN O..KHOJKI VOWEL SIGN AU
            0x11234..=0x11234, // Mn       KHOJKI SIGN ANUSVARA
            0x11235..=0x11235, // Mc       KHOJKI SIGN VIRAMA
            0x11236..=0x11237, // Mn   [2] KHOJKI SIGN NUKTA..KHOJKI SIGN SHADDA
            0x1123E..=0x1123E, // Mn       KHOJKI SIGN SUKUN
            0x11280..=0x11286, // Lo   [7] MULTANI LETTER A..MULTANI LETTER GA
            0x11288..=0x11288, // Lo       MULTANI LETTER GHA
            0x1128A..=0x1128D, // Lo   [4] MULTANI LETTER CA..MULTANI LETTER JJA
            0x1128F..=0x1129D, // Lo  [15] MULTANI LETTER NYA..MULTANI LETTER BA
            0x1129F..=0x112A8, // Lo  [10] MULTANI LETTER BHA..MULTANI LETTER RHA
            0x112B0..=0x112DE, // Lo  [47] KHUDAWADI LETTER A..KHUDAWADI LETTER HA
            0x112DF..=0x112DF, // Mn       KHUDAWADI SIGN ANUSVARA
            0x112E0..=0x112E2, // Mc   [3] KHUDAWADI VOWEL SIGN AA..KHUDAWADI VOWEL SIGN II
            0x112E3..=0x112EA, // Mn   [8] KHUDAWADI VOWEL SIGN U..KHUDAWADI SIGN VIRAMA
            0x112F0..=0x112F9, // Nd  [10] KHUDAWADI DIGIT ZERO..KHUDAWADI DIGIT NINE
            0x11300..=0x11301, // Mn   [2] GRANTHA SIGN COMBINING ANUSVARA ABOVE..GRANTHA SIGN CANDRABINDU
            0x11302..=0x11303, // Mc   [2] GRANTHA SIGN ANUSVARA..GRANTHA SIGN VISARGA
            0x11305..=0x1130C, // Lo   [8] GRANTHA LETTER A..GRANTHA LETTER VOCALIC L
            0x1130F..=0x11310, // Lo   [2] GRANTHA LETTER EE..GRANTHA LETTER AI
            0x11313..=0x11328, // Lo  [22] GRANTHA LETTER OO..GRANTHA LETTER NA
            0x1132A..=0x11330, // Lo   [7] GRANTHA LETTER PA..GRANTHA LETTER RA
            0x11332..=0x11333, // Lo   [2] GRANTHA LETTER LA..GRANTHA LETTER LLA
            0x11335..=0x11339, // Lo   [5] GRANTHA LETTER VA..GRANTHA LETTER HA
            0x1133C..=0x1133C, // Mn       GRANTHA SIGN NUKTA
            0x1133D..=0x1133D, // Lo       GRANTHA SIGN AVAGRAHA
            0x1133E..=0x1133F, // Mc   [2] GRANTHA VOWEL SIGN AA..GRANTHA VOWEL SIGN I
            0x11340..=0x11340, // Mn       GRANTHA VOWEL SIGN II
            0x11341..=0x11344, // Mc   [4] GRANTHA VOWEL SIGN U..GRANTHA VOWEL SIGN VOCALIC RR
            0x11347..=0x11348, // Mc   [2] GRANTHA VOWEL SIGN EE..GRANTHA VOWEL SIGN AI
            0x1134B..=0x1134D, // Mc   [3] GRANTHA VOWEL SIGN OO..GRANTHA SIGN VIRAMA
            0x11350..=0x11350, // Lo       GRANTHA OM
            0x11357..=0x11357, // Mc       GRANTHA AU LENGTH MARK
            0x1135D..=0x11361, // Lo   [5] GRANTHA SIGN PLUTA..GRANTHA LETTER VOCALIC LL
            0x11362..=0x11363, // Mc   [2] GRANTHA VOWEL SIGN VOCALIC L..GRANTHA VOWEL SIGN VOCALIC LL
            0x11366..=0x1136C, // Mn   [7] COMBINING GRANTHA DIGIT ZERO..COMBINING GRANTHA DIGIT SIX
            0x11370..=0x11374, // Mn   [5] COMBINING GRANTHA LETTER A..COMBINING GRANTHA LETTER PA
            0x11400..=0x11434, // Lo  [53] NEWA LETTER A..NEWA LETTER HA
            0x11435..=0x11437, // Mc   [3] NEWA VOWEL SIGN AA..NEWA VOWEL SIGN II
            0x11438..=0x1143F, // Mn   [8] NEWA VOWEL SIGN U..NEWA VOWEL SIGN AI
            0x11440..=0x11441, // Mc   [2] NEWA VOWEL SIGN O..NEWA VOWEL SIGN AU
            0x11442..=0x11444, // Mn   [3] NEWA SIGN VIRAMA..NEWA SIGN ANUSVARA
            0x11445..=0x11445, // Mc       NEWA SIGN VISARGA
            0x11446..=0x11446, // Mn       NEWA SIGN NUKTA
            0x11447..=0x1144A, // Lo   [4] NEWA SIGN AVAGRAHA..NEWA SIDDHI
            0x11450..=0x11459, // Nd  [10] NEWA DIGIT ZERO..NEWA DIGIT NINE
            0x11480..=0x114AF, // Lo  [48] TIRHUTA ANJI..TIRHUTA LETTER HA
            0x114B0..=0x114B2, // Mc   [3] TIRHUTA VOWEL SIGN AA..TIRHUTA VOWEL SIGN II
            0x114B3..=0x114B8, // Mn   [6] TIRHUTA VOWEL SIGN U..TIRHUTA VOWEL SIGN VOCALIC LL
            0x114B9..=0x114B9, // Mc       TIRHUTA VOWEL SIGN E
            0x114BA..=0x114BA, // Mn       TIRHUTA VOWEL SIGN SHORT E
            0x114BB..=0x114BE, // Mc   [4] TIRHUTA VOWEL SIGN AI..TIRHUTA VOWEL SIGN AU
            0x114BF..=0x114C0, // Mn   [2] TIRHUTA SIGN CANDRABINDU..TIRHUTA SIGN ANUSVARA
            0x114C1..=0x114C1, // Mc       TIRHUTA SIGN VISARGA
            0x114C2..=0x114C3, // Mn   [2] TIRHUTA SIGN VIRAMA..TIRHUTA SIGN NUKTA
            0x114C4..=0x114C5, // Lo   [2] TIRHUTA SIGN AVAGRAHA..TIRHUTA GVANG
            0x114C7..=0x114C7, // Lo       TIRHUTA OM
            0x114D0..=0x114D9, // Nd  [10] TIRHUTA DIGIT ZERO..TIRHUTA DIGIT NINE
            0x11580..=0x115AE, // Lo  [47] SIDDHAM LETTER A..SIDDHAM LETTER HA
            0x115AF..=0x115B1, // Mc   [3] SIDDHAM VOWEL SIGN AA..SIDDHAM VOWEL SIGN II
            0x115B2..=0x115B5, // Mn   [4] SIDDHAM VOWEL SIGN U..SIDDHAM VOWEL SIGN VOCALIC RR
            0x115B8..=0x115BB, // Mc   [4] SIDDHAM VOWEL SIGN E..SIDDHAM VOWEL SIGN AU
            0x115BC..=0x115BD, // Mn   [2] SIDDHAM SIGN CANDRABINDU..SIDDHAM SIGN ANUSVARA
            0x115BE..=0x115BE, // Mc       SIDDHAM SIGN VISARGA
            0x115BF..=0x115C0, // Mn   [2] SIDDHAM SIGN VIRAMA..SIDDHAM SIGN NUKTA
            0x115D8..=0x115DB, // Lo   [4] SIDDHAM LETTER THREE-CIRCLE ALTERNATE I..SIDDHAM LETTER ALTERNATE U
            0x115DC..=0x115DD, // Mn   [2] SIDDHAM VOWEL SIGN ALTERNATE U..SIDDHAM VOWEL SIGN ALTERNATE UU
            0x11600..=0x1162F, // Lo  [48] MODI LETTER A..MODI LETTER LLA
            0x11630..=0x11632, // Mc   [3] MODI VOWEL SIGN AA..MODI VOWEL SIGN II
            0x11633..=0x1163A, // Mn   [8] MODI VOWEL SIGN U..MODI VOWEL SIGN AI
            0x1163B..=0x1163C, // Mc   [2] MODI VOWEL SIGN O..MODI VOWEL SIGN AU
            0x1163D..=0x1163D, // Mn       MODI SIGN ANUSVARA
            0x1163E..=0x1163E, // Mc       MODI SIGN VISARGA
            0x1163F..=0x11640, // Mn   [2] MODI SIGN VIRAMA..MODI SIGN ARDHACANDRA
            0x11644..=0x11644, // Lo       MODI SIGN HUVA
            0x11650..=0x11659, // Nd  [10] MODI DIGIT ZERO..MODI DIGIT NINE
            0x11680..=0x116AA, // Lo  [43] TAKRI LETTER A..TAKRI LETTER RRA
            0x116AB..=0x116AB, // Mn       TAKRI SIGN ANUSVARA
            0x116AC..=0x116AC, // Mc       TAKRI SIGN VISARGA
            0x116AD..=0x116AD, // Mn       TAKRI VOWEL SIGN AA
            0x116AE..=0x116AF, // Mc   [2] TAKRI VOWEL SIGN I..TAKRI VOWEL SIGN II
            0x116B0..=0x116B5, // Mn   [6] TAKRI VOWEL SIGN U..TAKRI VOWEL SIGN AU
            0x116B6..=0x116B6, // Mc       TAKRI SIGN VIRAMA
            0x116B7..=0x116B7, // Mn       TAKRI SIGN NUKTA
            0x116C0..=0x116C9, // Nd  [10] TAKRI DIGIT ZERO..TAKRI DIGIT NINE
            0x11700..=0x11719, // Lo  [26] AHOM LETTER KA..AHOM LETTER JHA
            0x1171D..=0x1171F, // Mn   [3] AHOM CONSONANT SIGN MEDIAL LA..AHOM CONSONANT SIGN MEDIAL LIGATING RA
            0x11720..=0x11721, // Mc   [2] AHOM VOWEL SIGN A..AHOM VOWEL SIGN AA
            0x11722..=0x11725, // Mn   [4] AHOM VOWEL SIGN I..AHOM VOWEL SIGN UU
            0x11726..=0x11726, // Mc       AHOM VOWEL SIGN E
            0x11727..=0x1172B, // Mn   [5] AHOM VOWEL SIGN AW..AHOM SIGN KILLER
            0x11730..=0x11739, // Nd  [10] AHOM DIGIT ZERO..AHOM DIGIT NINE
            0x118A0..=0x118DF, // L&  [64] WARANG CITI CAPITAL LETTER NGAA..WARANG CITI SMALL LETTER VIYO
            0x118E0..=0x118E9, // Nd  [10] WARANG CITI DIGIT ZERO..WARANG CITI DIGIT NINE
            0x118FF..=0x118FF, // Lo       WARANG CITI OM
            0x11AC0..=0x11AF8, // Lo  [57] PAU CIN HAU LETTER PA..PAU CIN HAU GLOTTAL STOP FINAL
            0x11C00..=0x11C08, // Lo   [9] BHAIKSUKI LETTER A..BHAIKSUKI LETTER VOCALIC L
            0x11C0A..=0x11C2E, // Lo  [37] BHAIKSUKI LETTER E..BHAIKSUKI LETTER HA
            0x11C2F..=0x11C2F, // Mc       BHAIKSUKI VOWEL SIGN AA
            0x11C30..=0x11C36, // Mn   [7] BHAIKSUKI VOWEL SIGN I..BHAIKSUKI VOWEL SIGN VOCALIC L
            0x11C38..=0x11C3D, // Mn   [6] BHAIKSUKI VOWEL SIGN E..BHAIKSUKI SIGN ANUSVARA
            0x11C3E..=0x11C3E, // Mc       BHAIKSUKI SIGN VISARGA
            0x11C3F..=0x11C3F, // Mn       BHAIKSUKI SIGN VIRAMA
            0x11C40..=0x11C40, // Lo       BHAIKSUKI SIGN AVAGRAHA
            0x11C50..=0x11C59, // Nd  [10] BHAIKSUKI DIGIT ZERO..BHAIKSUKI DIGIT NINE
            0x11C72..=0x11C8F, // Lo  [30] MARCHEN LETTER KA..MARCHEN LETTER A
            0x11C92..=0x11CA7, // Mn  [22] MARCHEN SUBJOINED LETTER KA..MARCHEN SUBJOINED LETTER ZA
            0x11CA9..=0x11CA9, // Mc       MARCHEN SUBJOINED LETTER YA
            0x11CAA..=0x11CB0, // Mn   [7] MARCHEN SUBJOINED LETTER RA..MARCHEN VOWEL SIGN AA
            0x11CB1..=0x11CB1, // Mc       MARCHEN VOWEL SIGN I
            0x11CB2..=0x11CB3, // Mn   [2] MARCHEN VOWEL SIGN U..MARCHEN VOWEL SIGN E
            0x11CB4..=0x11CB4, // Mc       MARCHEN VOWEL SIGN O
            0x11CB5..=0x11CB6, // Mn   [2] MARCHEN SIGN ANUSVARA..MARCHEN SIGN CANDRABINDU
            0x12000..=0x12399, // Lo [922] CUNEIFORM SIGN A..CUNEIFORM SIGN U U
            0x12400..=0x1246E, // Nl [111] CUNEIFORM NUMERIC SIGN TWO ASH..CUNEIFORM NUMERIC SIGN NINE U VARIANT FORM
            0x12480..=0x12543, // Lo [196] CUNEIFORM SIGN AB TIMES NUN TENU..CUNEIFORM SIGN ZU5 TIMES THREE DISH TENU
            0x13000..=0x1342E, // Lo [1071] EGYPTIAN HIEROGLYPH A001..EGYPTIAN HIEROGLYPH AA032
            0x14400..=0x14646, // Lo [583] ANATOLIAN HIEROGLYPH A001..ANATOLIAN HIEROGLYPH A530
            0x16800..=0x16A38, // Lo [569] BAMUM LETTER PHASE-A NGKUE MFON..BAMUM LETTER PHASE-F VUEQ
            0x16A40..=0x16A5E, // Lo  [31] MRO LETTER TA..MRO LETTER TEK
            0x16A60..=0x16A69, // Nd  [10] MRO DIGIT ZERO..MRO DIGIT NINE
            0x16AD0..=0x16AED, // Lo  [30] BASSA VAH LETTER ENNI..BASSA VAH LETTER I
            0x16AF0..=0x16AF4, // Mn   [5] BASSA VAH COMBINING HIGH TONE..BASSA VAH COMBINING HIGH-LOW TONE
            0x16B00..=0x16B2F, // Lo  [48] PAHAWH HMONG VOWEL KEEB..PAHAWH HMONG CONSONANT CAU
            0x16B30..=0x16B36, // Mn   [7] PAHAWH HMONG MARK CIM TUB..PAHAWH HMONG MARK CIM TAUM
            0x16B40..=0x16B43, // Lm   [4] PAHAWH HMONG SIGN VOS SEEV..PAHAWH HMONG SIGN IB YAM
            0x16B50..=0x16B59, // Nd  [10] PAHAWH HMONG DIGIT ZERO..PAHAWH HMONG DIGIT NINE
            0x16B63..=0x16B77, // Lo  [21] PAHAWH HMONG SIGN VOS LUB..PAHAWH HMONG SIGN CIM NRES TOS
            0x16B7D..=0x16B8F, // Lo  [19] PAHAWH HMONG CLAN SIGN TSHEEJ..PAHAWH HMONG CLAN SIGN VWJ
            0x16F00..=0x16F44, // Lo  [69] MIAO LETTER PA..MIAO LETTER HHA
            0x16F50..=0x16F50, // Lo       MIAO LETTER NASALIZATION
            0x16F51..=0x16F7E, // Mc  [46] MIAO SIGN ASPIRATION..MIAO VOWEL SIGN NG
            0x16F8F..=0x16F92, // Mn   [4] MIAO TONE RIGHT..MIAO TONE BELOW
            0x16F93..=0x16F9F, // Lm  [13] MIAO LETTER TONE-2..MIAO LETTER REFORMED TONE-8
            0x16FE0..=0x16FE0, // Lm       TANGUT ITERATION MARK
            0x17000..=0x187EC, // Lo [6125] TANGUT IDEOGRAPH-17000..TANGUT IDEOGRAPH-187EC
            0x18800..=0x18AF2, // Lo [755] TANGUT COMPONENT-001..TANGUT COMPONENT-755
            0x1B000..=0x1B001, // Lo   [2] KATAKANA LETTER ARCHAIC E..HIRAGANA LETTER ARCHAIC YE
            0x1BC00..=0x1BC6A, // Lo [107] DUPLOYAN LETTER H..DUPLOYAN LETTER VOCALIC M
            0x1BC70..=0x1BC7C, // Lo  [13] DUPLOYAN AFFIX LEFT HORIZONTAL SECANT..DUPLOYAN AFFIX ATTACHED TANGENT HOOK
            0x1BC80..=0x1BC88, // Lo   [9] DUPLOYAN AFFIX HIGH ACUTE..DUPLOYAN AFFIX HIGH VERTICAL
            0x1BC90..=0x1BC99, // Lo  [10] DUPLOYAN AFFIX LOW ACUTE..DUPLOYAN AFFIX LOW ARROW
            0x1BC9D..=0x1BC9E, // Mn   [2] DUPLOYAN THICK LETTER SELECTOR..DUPLOYAN DOUBLE MARK
            0x1D165..=0x1D166, // Mc   [2] MUSICAL SYMBOL COMBINING STEM..MUSICAL SYMBOL COMBINING SPRECHGESANG STEM
            0x1D167..=0x1D169, // Mn   [3] MUSICAL SYMBOL COMBINING TREMOLO-1..MUSICAL SYMBOL COMBINING TREMOLO-3
            0x1D16D..=0x1D172, // Mc   [6] MUSICAL SYMBOL COMBINING AUGMENTATION DOT..MUSICAL SYMBOL COMBINING FLAG-5
            0x1D17B..=0x1D182, // Mn   [8] MUSICAL SYMBOL COMBINING ACCENT..MUSICAL SYMBOL COMBINING LOURE
            0x1D185..=0x1D18B, // Mn   [7] MUSICAL SYMBOL COMBINING DOIT..MUSICAL SYMBOL COMBINING TRIPLE TONGUE
            0x1D1AA..=0x1D1AD, // Mn   [4] MUSICAL SYMBOL COMBINING DOWN BOW..MUSICAL SYMBOL COMBINING SNAP PIZZICATO
            0x1D242..=0x1D244, // Mn   [3] COMBINING GREEK MUSICAL TRISEME..COMBINING GREEK MUSICAL PENTASEME
            0x1D400..=0x1D454, // L&  [85] MATHEMATICAL BOLD CAPITAL A..MATHEMATICAL ITALIC SMALL G
            0x1D456..=0x1D49C, // L&  [71] MATHEMATICAL ITALIC SMALL I..MATHEMATICAL SCRIPT CAPITAL A
            0x1D49E..=0x1D49F, // L&   [2] MATHEMATICAL SCRIPT CAPITAL C..MATHEMATICAL SCRIPT CAPITAL D
            0x1D4A2..=0x1D4A2, // L&       MATHEMATICAL SCRIPT CAPITAL G
            0x1D4A5..=0x1D4A6, // L&   [2] MATHEMATICAL SCRIPT CAPITAL J..MATHEMATICAL SCRIPT CAPITAL K
            0x1D4A9..=0x1D4AC, // L&   [4] MATHEMATICAL SCRIPT CAPITAL N..MATHEMATICAL SCRIPT CAPITAL Q
            0x1D4AE..=0x1D4B9, // L&  [12] MATHEMATICAL SCRIPT CAPITAL S..MATHEMATICAL SCRIPT SMALL D
            0x1D4BB..=0x1D4BB, // L&       MATHEMATICAL SCRIPT SMALL F
            0x1D4BD..=0x1D4C3, // L&   [7] MATHEMATICAL SCRIPT SMALL H..MATHEMATICAL SCRIPT SMALL N
            0x1D4C5..=0x1D505, // L&  [65] MATHEMATICAL SCRIPT SMALL P..MATHEMATICAL FRAKTUR CAPITAL B
            0x1D507..=0x1D50A, // L&   [4] MATHEMATICAL FRAKTUR CAPITAL D..MATHEMATICAL FRAKTUR CAPITAL G
            0x1D50D..=0x1D514, // L&   [8] MATHEMATICAL FRAKTUR CAPITAL J..MATHEMATICAL FRAKTUR CAPITAL Q
            0x1D516..=0x1D51C, // L&   [7] MATHEMATICAL FRAKTUR CAPITAL S..MATHEMATICAL FRAKTUR CAPITAL Y
            0x1D51E..=0x1D539, // L&  [28] MATHEMATICAL FRAKTUR SMALL A..MATHEMATICAL DOUBLE-STRUCK CAPITAL B
            0x1D53B..=0x1D53E, // L&   [4] MATHEMATICAL DOUBLE-STRUCK CAPITAL D..MATHEMATICAL DOUBLE-STRUCK CAPITAL G
            0x1D540..=0x1D544, // L&   [5] MATHEMATICAL DOUBLE-STRUCK CAPITAL I..MATHEMATICAL DOUBLE-STRUCK CAPITAL M
            0x1D546..=0x1D546, // L&       MATHEMATICAL DOUBLE-STRUCK CAPITAL O
            0x1D54A..=0x1D550, // L&   [7] MATHEMATICAL DOUBLE-STRUCK CAPITAL S..MATHEMATICAL DOUBLE-STRUCK CAPITAL Y
            0x1D552..=0x1D6A5, // L& [340] MATHEMATICAL DOUBLE-STRUCK SMALL A..MATHEMATICAL ITALIC SMALL DOTLESS J
            0x1D6A8..=0x1D6C0, // L&  [25] MATHEMATICAL BOLD CAPITAL ALPHA..MATHEMATICAL BOLD CAPITAL OMEGA
            0x1D6C2..=0x1D6DA, // L&  [25] MATHEMATICAL BOLD SMALL ALPHA..MATHEMATICAL BOLD SMALL OMEGA
            0x1D6DC..=0x1D6FA, // L&  [31] MATHEMATICAL BOLD EPSILON SYMBOL..MATHEMATICAL ITALIC CAPITAL OMEGA
            0x1D6FC..=0x1D714, // L&  [25] MATHEMATICAL ITALIC SMALL ALPHA..MATHEMATICAL ITALIC SMALL OMEGA
            0x1D716..=0x1D734, // L&  [31] MATHEMATICAL ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD ITALIC CAPITAL OMEGA
            0x1D736..=0x1D74E, // L&  [25] MATHEMATICAL BOLD ITALIC SMALL ALPHA..MATHEMATICAL BOLD ITALIC SMALL OMEGA
            0x1D750..=0x1D76E, // L&  [31] MATHEMATICAL BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD CAPITAL OMEGA
            0x1D770..=0x1D788, // L&  [25] MATHEMATICAL SANS-SERIF BOLD SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD SMALL OMEGA
            0x1D78A..=0x1D7A8, // L&  [31] MATHEMATICAL SANS-SERIF BOLD EPSILON SYMBOL..MATHEMATICAL SANS-SERIF BOLD ITALIC CAPITAL OMEGA
            0x1D7AA..=0x1D7C2, // L&  [25] MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL ALPHA..MATHEMATICAL SANS-SERIF BOLD ITALIC SMALL OMEGA
            0x1D7C4..=0x1D7CB, // L&   [8] MATHEMATICAL SANS-SERIF BOLD ITALIC EPSILON SYMBOL..MATHEMATICAL BOLD SMALL DIGAMMA
            0x1D7CE..=0x1D7FF, // Nd  [50] MATHEMATICAL BOLD DIGIT ZERO..MATHEMATICAL MONOSPACE DIGIT NINE
            0x1DA00..=0x1DA36, // Mn  [55] SIGNWRITING HEAD RIM..SIGNWRITING AIR SUCKING IN
            0x1DA3B..=0x1DA6C, // Mn  [50] SIGNWRITING MOUTH CLOSED NEUTRAL..SIGNWRITING EXCITEMENT
            0x1DA75..=0x1DA75, // Mn       SIGNWRITING UPPER BODY TILTING FROM HIP JOINTS
            0x1DA84..=0x1DA84, // Mn       SIGNWRITING LOCATION HEAD NECK
            0x1DA9B..=0x1DA9F, // Mn   [5] SIGNWRITING FILL MODIFIER-2..SIGNWRITING FILL MODIFIER-6
            0x1DAA1..=0x1DAAF, // Mn  [15] SIGNWRITING ROTATION MODIFIER-2..SIGNWRITING ROTATION MODIFIER-16
            0x1E000..=0x1E006, // Mn   [7] COMBINING GLAGOLITIC LETTER AZU..COMBINING GLAGOLITIC LETTER ZHIVETE
            0x1E008..=0x1E018, // Mn  [17] COMBINING GLAGOLITIC LETTER ZEMLJA..COMBINING GLAGOLITIC LETTER HERU
            0x1E01B..=0x1E021, // Mn   [7] COMBINING GLAGOLITIC LETTER SHTA..COMBINING GLAGOLITIC LETTER YATI
            0x1E023..=0x1E024, // Mn   [2] COMBINING GLAGOLITIC LETTER YU..COMBINING GLAGOLITIC LETTER SMALL YUS
            0x1E026..=0x1E02A, // Mn   [5] COMBINING GLAGOLITIC LETTER YO..COMBINING GLAGOLITIC LETTER FITA
            0x1E800..=0x1E8C4, // Lo [197] MENDE KIKAKUI SYLLABLE M001 KI..MENDE KIKAKUI SYLLABLE M060 NYON
            0x1E8D0..=0x1E8D6, // Mn   [7] MENDE KIKAKUI COMBINING NUMBER TEENS..MENDE KIKAKUI COMBINING NUMBER MILLIONS
            0x1E900..=0x1E943, // L&  [68] ADLAM CAPITAL LETTER ALIF..ADLAM SMALL LETTER SHA
            0x1E944..=0x1E94A, // Mn   [7] ADLAM ALIF LENGTHENER..ADLAM NUKTA
            0x1E950..=0x1E959, // Nd  [10] ADLAM DIGIT ZERO..ADLAM DIGIT NINE
            0x1EE00..=0x1EE03, // Lo   [4] ARABIC MATHEMATICAL ALEF..ARABIC MATHEMATICAL DAL
            0x1EE05..=0x1EE1F, // Lo  [27] ARABIC MATHEMATICAL WAW..ARABIC MATHEMATICAL DOTLESS QAF
            0x1EE21..=0x1EE22, // Lo   [2] ARABIC MATHEMATICAL INITIAL BEH..ARABIC MATHEMATICAL INITIAL JEEM
            0x1EE24..=0x1EE24, // Lo       ARABIC MATHEMATICAL INITIAL HEH
            0x1EE27..=0x1EE27, // Lo       ARABIC MATHEMATICAL INITIAL HAH
            0x1EE29..=0x1EE32, // Lo  [10] ARABIC MATHEMATICAL INITIAL YEH..ARABIC MATHEMATICAL INITIAL QAF
            0x1EE34..=0x1EE37, // Lo   [4] ARABIC MATHEMATICAL INITIAL SHEEN..ARABIC MATHEMATICAL INITIAL KHAH
            0x1EE39..=0x1EE39, // Lo       ARABIC MATHEMATICAL INITIAL DAD
            0x1EE3B..=0x1EE3B, // Lo       ARABIC MATHEMATICAL INITIAL GHAIN
            0x1EE42..=0x1EE42, // Lo       ARABIC MATHEMATICAL TAILED JEEM
            0x1EE47..=0x1EE47, // Lo       ARABIC MATHEMATICAL TAILED HAH
            0x1EE49..=0x1EE49, // Lo       ARABIC MATHEMATICAL TAILED YEH
            0x1EE4B..=0x1EE4B, // Lo       ARABIC MATHEMATICAL TAILED LAM
            0x1EE4D..=0x1EE4F, // Lo   [3] ARABIC MATHEMATICAL TAILED NOON..ARABIC MATHEMATICAL TAILED AIN
            0x1EE51..=0x1EE52, // Lo   [2] ARABIC MATHEMATICAL TAILED SAD..ARABIC MATHEMATICAL TAILED QAF
            0x1EE54..=0x1EE54, // Lo       ARABIC MATHEMATICAL TAILED SHEEN
            0x1EE57..=0x1EE57, // Lo       ARABIC MATHEMATICAL TAILED KHAH
            0x1EE59..=0x1EE59, // Lo       ARABIC MATHEMATICAL TAILED DAD
            0x1EE5B..=0x1EE5B, // Lo       ARABIC MATHEMATICAL TAILED GHAIN
            0x1EE5D..=0x1EE5D, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS NOON
            0x1EE5F..=0x1EE5F, // Lo       ARABIC MATHEMATICAL TAILED DOTLESS QAF
            0x1EE61..=0x1EE62, // Lo   [2] ARABIC MATHEMATICAL STRETCHED BEH..ARABIC MATHEMATICAL STRETCHED JEEM
            0x1EE64..=0x1EE64, // Lo       ARABIC MATHEMATICAL STRETCHED HEH
            0x1EE67..=0x1EE6A, // Lo   [4] ARABIC MATHEMATICAL STRETCHED HAH..ARABIC MATHEMATICAL STRETCHED KAF
            0x1EE6C..=0x1EE72, // Lo   [7] ARABIC MATHEMATICAL STRETCHED MEEM..ARABIC MATHEMATICAL STRETCHED QAF
            0x1EE74..=0x1EE77, // Lo   [4] ARABIC MATHEMATICAL STRETCHED SHEEN..ARABIC MATHEMATICAL STRETCHED KHAH
            0x1EE79..=0x1EE7C, // Lo   [4] ARABIC MATHEMATICAL STRETCHED DAD..ARABIC MATHEMATICAL STRETCHED DOTLESS BEH
            0x1EE7E..=0x1EE7E, // Lo       ARABIC MATHEMATICAL STRETCHED DOTLESS FEH
            0x1EE80..=0x1EE89, // Lo  [10] ARABIC MATHEMATICAL LOOPED ALEF..ARABIC MATHEMATICAL LOOPED YEH
            0x1EE8B..=0x1EE9B, // Lo  [17] ARABIC MATHEMATICAL LOOPED LAM..ARABIC MATHEMATICAL LOOPED GHAIN
            0x1EEA1..=0x1EEA3, // Lo   [3] ARABIC MATHEMATICAL DOUBLE-STRUCK BEH..ARABIC MATHEMATICAL DOUBLE-STRUCK DAL
            0x1EEA5..=0x1EEA9, // Lo   [5] ARABIC MATHEMATICAL DOUBLE-STRUCK WAW..ARABIC MATHEMATICAL DOUBLE-STRUCK YEH
            0x1EEAB..=0x1EEBB, // Lo  [17] ARABIC MATHEMATICAL DOUBLE-STRUCK LAM..ARABIC MATHEMATICAL DOUBLE-STRUCK GHAIN
            0x20000..=0x2A6D6, // Lo [42711] CJK UNIFIED IDEOGRAPH-20000..CJK UNIFIED IDEOGRAPH-2A6D6
            0x2A700..=0x2B734, // Lo [4149] CJK UNIFIED IDEOGRAPH-2A700..CJK UNIFIED IDEOGRAPH-2B734
            0x2B740..=0x2B81D, // Lo [222] CJK UNIFIED IDEOGRAPH-2B740..CJK UNIFIED IDEOGRAPH-2B81D
            0x2B820..=0x2CEA1, // Lo [5762] CJK UNIFIED IDEOGRAPH-2B820..CJK UNIFIED IDEOGRAPH-2CEA1
            0x2F800..=0x2FA1D, // Lo [542] CJK COMPATIBILITY IDEOGRAPH-2F800..CJK COMPATIBILITY IDEOGRAPH-2FA1D
            0xE0100..=0xE01EF, // Mn [240] VARIATION SELECTOR-17..VARIATION SELECTOR-256
        ][..]
    }
}
