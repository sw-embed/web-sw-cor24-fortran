//! FTI-0 (FORTRAN) syntax highlighter for the editor overlay.
//!
//! Fixed-form layout: column 1 holds 'C', 'c', or '*' for full-line
//! comments. The rest of the line is keyword/string/number-coloured.

pub struct Span {
    pub text: String,
    pub color: &'static str,
}

const KEYWORD: &str = "#cba6f7";
const TYPE_KW: &str = "#89b4fa";
const NUMBER: &str = "#fab387";
const STRING: &str = "#a6e3a1";
const COMMENT: &str = "#a6adc8";
const PLAIN: &str = "#cdd6f4";

const KEYWORDS: &[&str] = &[
    "PROGRAM", "END", "STOP", "RETURN", "SUBROUTINE", "FUNCTION", "CALL", "GOTO", "GO", "TO",
    "IF", "THEN", "ELSE", "ENDIF", "DO", "CONTINUE", "PRINT", "READ", "WRITE", "FORMAT",
    "DATA", "DIMENSION", "COMMON", "EQUIVALENCE", "EXTERNAL", "INTRINSIC", "PARAMETER",
];

const TYPE_KEYWORDS: &[&str] = &[
    "INTEGER", "REAL", "DOUBLE", "PRECISION", "COMPLEX", "LOGICAL", "CHARACTER",
];

fn is_keyword(word: &str) -> bool {
    KEYWORDS.iter().any(|k| k.eq_ignore_ascii_case(word))
}
fn is_type_keyword(word: &str) -> bool {
    TYPE_KEYWORDS.iter().any(|k| k.eq_ignore_ascii_case(word))
}

pub fn highlight(source: &str) -> Vec<Span> {
    let mut spans = Vec::new();

    for (i, raw_line) in source.split('\n').enumerate() {
        if i > 0 {
            spans.push(Span { text: "\n".into(), color: PLAIN });
        }

        let first = raw_line.as_bytes().first().copied();
        if matches!(first, Some(b'C') | Some(b'c') | Some(b'*')) {
            spans.push(Span { text: raw_line.into(), color: COMMENT });
            continue;
        }

        highlight_statement_line(raw_line, &mut spans);
    }
    spans
}

fn highlight_statement_line(line: &str, spans: &mut Vec<Span>) {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        let ch = bytes[i];

        if ch == b'\'' {
            let start = i;
            i += 1;
            while i < len {
                if bytes[i] == b'\'' {
                    if i + 1 < len && bytes[i + 1] == b'\'' { i += 2; continue; }
                    i += 1;
                    break;
                }
                i += 1;
            }
            spans.push(Span { text: line[start..i].into(), color: STRING });
            continue;
        }

        if ch.is_ascii_digit() {
            let start = i;
            while i < len && bytes[i].is_ascii_digit() { i += 1; }
            if i < len && bytes[i] == b'.' {
                i += 1;
                while i < len && bytes[i].is_ascii_digit() { i += 1; }
            }
            spans.push(Span { text: line[start..i].into(), color: NUMBER });
            continue;
        }

        if ch.is_ascii_alphabetic() || ch == b'_' {
            let start = i;
            while i < len && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                i += 1;
            }
            let word = &line[start..i];
            let color = if is_keyword(word) {
                KEYWORD
            } else if is_type_keyword(word) {
                TYPE_KW
            } else {
                PLAIN
            };
            spans.push(Span { text: word.into(), color });
            continue;
        }

        let start = i;
        i += 1;
        while i < len
            && !bytes[i].is_ascii_alphanumeric()
            && bytes[i] != b'_'
            && bytes[i] != b'\''
        {
            i += 1;
        }
        spans.push(Span { text: line[start..i].into(), color: PLAIN });
    }
}
