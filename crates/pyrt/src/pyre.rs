//! Python `re` module semantics over the regex crate (with fancy-regex fallback).
use std::rc::Rc;

use crate::exc::PyExc;
use crate::value::*;

pub const IGNORECASE: i64 = 2;
pub const LOCALE: i64 = 4;
pub const MULTILINE: i64 = 8;
pub const DOTALL: i64 = 16;
pub const UNICODE: i64 = 32;
pub const VERBOSE: i64 = 64;
pub const ASCII: i64 = 256;
pub const TEMPLATE: i64 = 2; // re.T == re.I bit overlap in modern python? no: T=1
// Python: I=2, L=4, M=8, S=16, X=64, U=32, A=256, T=1
pub const T: i64 = 1;

enum Engine {
    Fast(regex::Regex),
    Fancy(fancy_regex::Regex),
}

pub struct PyPattern {
    pub source: String,
    pub flags: i64,
    pub engine: Engine,
    pub group_count: usize,
    pub group_names: Vec<String>,
}

pub struct PyMatch {
    pub text: String,
    /// Byte spans per group (None if group didn't participate); group 0 = whole match.
    pub spans: Vec<Option<(usize, usize)>>,
    pub names: Vec<String>,
}

fn flag_prefix(flags: i64) -> String {
    let mut s = String::from("(?");
    if flags & IGNORECASE != 0 { s.push('i'); }
    if flags & MULTILINE != 0 { s.push('m'); }
    if flags & DOTALL != 0 { s.push('s'); }
    if flags & VERBOSE != 0 { s.push('x'); }
    if flags & ASCII != 0 { s.push_str("-u"); }
    s.push(')');
    if s == "(?)" { String::new() } else { s }
}

fn count_groups(pattern: &str) -> (usize, Vec<String>) {
    // Count capturing groups, respecting escapes and character classes.
    let mut count = 0usize;
    let mut names = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            '\\' => { i += 2; continue; }
            '[' => {
                i += 1;
                if i < chars.len() && chars[i] == '^' { i += 1; }
                if i < chars.len() && chars[i] == ']' { i += 1; }
                while i < chars.len() && chars[i] != ']' {
                    if chars[i] == '\\' { i += 1; }
                    i += 1;
                }
                i += 1;
            }
            '(' => {
                if i + 1 < chars.len() && chars[i + 1] == '?' {
                    // (?P<name>...) capturing; others non-capturing
                    if i + 2 < chars.len() && chars[i + 2] == 'P' && i + 3 < chars.len() && chars[i + 3] == '<' {
                        if i + 4 < chars.len() && (chars[i + 4] == '=' || chars[i + 4] == '!') {
                            // lookbehind: non-capturing
                        } else {
                            count += 1;
                            let mut name = String::new();
                            let mut j = i + 4;
                            while j < chars.len() && chars[j] != '>' { name.push(chars[j]); j += 1; }
                            names.push(name);
                        }
                    }
                    // (?'name') not python; (?= (?! (?<= (?<! (?: (?# (?i: etc non-capturing
                } else {
                    count += 1;
                }
                i += 1;
            }
            _ => { i += 1; }
        }
    }
    (count, names)
}

/// Translate Python-only regex syntax the Rust engines reject.
/// fancy-regex already supports (?P<name>...) and lookarounds, so this is a pass-through.
fn translate_pattern(pat: &str) -> String {
    pat.replace(r"\Z", r"\z")
}

pub fn compile(pattern: &str, flags: i64) -> Result<Rc<PyPattern>, PyExc> {
    let (group_count, group_names) = count_groups(pattern);
    let full = format!("{}{}", flag_prefix(flags), pattern);
    if let Ok(re) = regex::Regex::new(&full) {
        return Ok(Rc::new(PyPattern { source: pattern.to_string(), flags, engine: Engine::Fast(re), group_count, group_names }));
    }
    let translated = translate_pattern(pattern);
    let full = format!("{}{}", flag_prefix(flags), translated);
    match fancy_regex::Regex::new(&full) {
        Ok(re) => Ok(Rc::new(PyPattern { source: pattern.to_string(), flags, engine: Engine::Fancy(re), group_count, group_names })),
        Err(e) => Err(crate::exc::py_err("error", &format!("regex error: {}", e))),
    }
}

/// Byte-span capture result.
fn captures(pat: &PyPattern, text: &str, start_at: usize) -> Option<Vec<Option<(usize, usize)>>> {
    let hay = &text[start_at..];
    match &pat.engine {
        Engine::Fast(re) => {
            let caps = re.captures(hay)?;
            let mut spans = Vec::with_capacity(caps.len());
            for i in 0..caps.len() {
                spans.push(caps.get(i).map(|m| (m.start() + start_at, m.end() + start_at)));
            }
            Some(spans)
        }
        Engine::Fancy(re) => {
            let caps = re.captures(hay).ok()??;
            let mut spans = Vec::with_capacity(caps.len());
            for i in 0..caps.len() {
                spans.push(caps.get(i).map(|m| (m.start() + start_at, m.end() + start_at)));
            }
            Some(spans)
        }
    }
}

fn make_match(text: &str, spans: Vec<Option<(usize, usize)>>, pat: &PyPattern) -> Py {
    Py::Match(Rc::new(PyMatch { text: text.to_string(), spans, names: pat.group_names.clone() }))
}

impl PyMatch {
    pub fn group(&self, i: i64) -> Py {
        let idx = i as usize;
        match self.spans.get(idx).copied().flatten() {
            Some((s, e)) => py_string(self.text[s..e].to_string()),
            None => Py::None,
        }
    }
    fn char_pos(&self, byte: usize) -> i64 {
        self.text[..byte].chars().count() as i64
    }
    pub fn start(&self, i: i64) -> Py {
        match self.spans.get(i as usize).copied().flatten() {
            Some((s, _)) => Py::Int(self.char_pos(s)),
            None => Py::Int(-1),
        }
    }
    pub fn end(&self, i: i64) -> Py {
        match self.spans.get(i as usize).copied().flatten() {
            Some((_, e)) => Py::Int(self.char_pos(e)),
            None => Py::Int(-1),
        }
    }
    pub fn span(&self, i: i64) -> Py {
        match self.spans.get(i as usize).copied().flatten() {
            Some((s, e)) => py_tuple(vec![Py::Int(self.char_pos(s)), Py::Int(self.char_pos(e))]),
            None => py_tuple(vec![Py::Int(-1), Py::Int(-1)]),
        }
    }
    pub fn group_by_name(&self, name: &str) -> Py {
        if let Some(pos) = self.names.iter().position(|n| n == name) {
            self.group(pos as i64 + 1)
        } else {
            Py::None
        }
    }
    pub fn groups(&self, default: &Py) -> Py {
        let mut out = Vec::new();
        for i in 1..self.spans.len() {
            match self.spans[i] {
                Some((s, e)) => out.push(py_string(self.text[s..e].to_string())),
                None => out.push(default.clone()),
            }
        }
        py_tuple(out)
    }
    pub fn groupdict(&self, default: &Py) -> Py {
        let mut out = Vec::new();
        for (i, name) in self.names.iter().enumerate() {
            let v = match self.spans.get(i + 1).copied().flatten() {
                Some((s, e)) => py_string(self.text[s..e].to_string()),
                None => default.clone(),
            };
            out.push((py_str(name), v));
        }
        py_dict(out)
    }
    pub fn expand(&self, template: &str) -> Py {
        py_string(expand_template(template, &self.text, &self.spans, &self.names))
    }
}

/// Python replacement-template expansion: \1 \g<1> \g<name> \\ \n \t
pub fn expand_template(template: &str, text: &str, spans: &[Option<(usize, usize)>], names: &[String]) -> String {
    let chars: Vec<char> = template.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let get_group = |idx: usize| -> String {
        match spans.get(idx).copied().flatten() {
            Some((s, e)) => text[s..e].to_string(),
            None => String::new(),
        }
    };
    while i < chars.len() {
        let c = chars[i];
        if c == '\\' && i + 1 < chars.len() {
            let n = chars[i + 1];
            match n {
                'g' if i + 2 < chars.len() && chars[i + 2] == '<' => {
                    let mut j = i + 3;
                    let mut name = String::new();
                    while j < chars.len() && chars[j] != '>' { name.push(chars[j]); j += 1; }
                    i = j + 1;
                    if let Ok(idx) = name.parse::<usize>() {
                        out.push_str(&get_group(idx));
                    } else if let Some(pos) = names.iter().position(|n| n == &name) {
                        out.push_str(&get_group(pos + 1));
                    }
                    continue;
                }
                '0'..='9' => {
                    // up to two digits; \1..\99; \0 -> whole match? Python: \0 is null byte... octal. Keep: single digit unless two-digit valid group
                    let d1 = n.to_digit(10).unwrap() as usize;
                    let mut idx = d1;
                    let mut consumed = 2;
                    if i + 2 < chars.len() {
                        if let Some(d2) = chars[i + 2].to_digit(10) {
                            let two = d1 * 10 + d2 as usize;
                            if two < spans.len() && two > 0 { idx = two; consumed = 3; }
                        }
                    }
                    out.push_str(&get_group(idx));
                    i += consumed;
                    continue;
                }
                'n' => { out.push('\n'); i += 2; continue; }
                't' => { out.push('\t'); i += 2; continue; }
                'r' => { out.push('\r'); i += 2; continue; }
                'f' => { out.push('\x0c'); i += 2; continue; }
                'v' => { out.push('\x0b'); i += 2; continue; }
                'a' => { out.push('\x07'); i += 2; continue; }
                '\\' => { out.push('\\'); i += 2; continue; }
                other => { out.push('\\'); out.push(other); i += 2; continue; }
            }
        }
        out.push(c);
        i += 1;
    }
    out
}

// ---------------------------------------------------------------- module-level ops
pub fn search(pat: &Rc<PyPattern>, text: &str, pos: usize) -> Py {
    // pos is a char index in Python; convert to byte offset
    let byte_pos = char_to_byte(text, pos);
    match captures(pat, text, byte_pos) {
        Some(spans) => make_match(text, spans, pat),
        None => Py::None,
    }
}

pub fn match_at(pat: &Rc<PyPattern>, text: &str, pos: usize, require_start: bool, require_end: bool) -> Py {
    let byte_pos = char_to_byte(text, pos);
    match captures(pat, text, byte_pos) {
        Some(spans) => {
            if require_start {
                if let Some(Some((s, _))) = spans.first() {
                    if *s != byte_pos { return Py::None; }
                }
            }
            if require_end {
                if let Some(Some((_, e))) = spans.first() {
                    if *e != text.len() { return Py::None; }
                }
            }
            make_match(text, spans, pat)
        }
        None => Py::None,
    }
}

fn char_to_byte(text: &str, char_pos: usize) -> usize {
    text.char_indices().nth(char_pos).map(|(b, _)| b).unwrap_or(text.len())
}

pub fn findall(pat: &Rc<PyPattern>, text: &str) -> Py {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos <= text.len() {
        match captures(pat, text, pos) {
            Some(spans) => {
                let (s, e) = spans[0].unwrap();
                if pat.group_count == 0 {
                    out.push(py_string(text[s..e].to_string()));
                } else if pat.group_count == 1 {
                    out.push(match spans.get(1).copied().flatten() {
                        Some((gs, ge)) => py_string(text[gs..ge].to_string()),
                        None => py_str(""),
                    });
                } else {
                    let mut t = Vec::new();
                    for gi in 1..=pat.group_count {
                        t.push(match spans.get(gi).copied().flatten() {
                            Some((gs, ge)) => py_string(text[gs..ge].to_string()),
                            None => py_str(""),
                        });
                    }
                    out.push(py_tuple(t));
                }
                pos = if e > s { e } else { e + 1 };
                if pos > text.len() { break; }
            }
            None => break,
        }
    }
    py_list(out)
}

pub fn finditer(pat: &Rc<PyPattern>, text: &str) -> Py {
    let mut out = Vec::new();
    let mut pos = 0usize;
    while pos <= text.len() {
        match captures(pat, text, pos) {
            Some(spans) => {
                let (s, e) = spans[0].unwrap();
                out.push(make_match(text, spans.clone(), pat));
                pos = if e > s { e } else { e + 1 };
                if pos > text.len() { break; }
            }
            None => break,
        }
    }
    py_list(out)
}

pub fn split(pat: &Rc<PyPattern>, text: &str, maxsplit: i64) -> Py {
    let mut out = Vec::new();
    let mut pos = 0usize;
    let mut last = 0usize;
    let mut splits = 0i64;
    while pos <= text.len() && (maxsplit <= 0 || splits < maxsplit) {
        match captures(pat, text, pos) {
            Some(spans) => {
                let (s, e) = spans[0].unwrap();
                out.push(py_string(text[last..s].to_string()));
                // captured groups are included
                for gi in 1..=pat.group_count {
                    match spans.get(gi).copied().flatten() {
                        Some((gs, ge)) => out.push(py_string(text[gs..ge].to_string())),
                        None => out.push(Py::None),
                    }
                }
                last = e;
                splits += 1;
                pos = if e > s { e } else { e + 1 };
                if pos > text.len() { break; }
            }
            None => break,
        }
    }
    out.push(py_string(text[last..].to_string()));
    py_list(out)
}

pub fn sub(pat: &Rc<PyPattern>, repl: &Py, text: &str, count: i64) -> Py {
    let (result, _) = sub_impl(pat, repl, text, count);
    result
}

pub fn subn(pat: &Rc<PyPattern>, repl: &Py, text: &str, count: i64) -> Py {
    let (result, n) = sub_impl(pat, repl, text, count);
    py_tuple(vec![result, Py::Int(n as i64)])
}

fn sub_impl(pat: &Rc<PyPattern>, repl: &Py, text: &str, count: i64) -> (Py, usize) {
    let mut out = String::new();
    let mut pos = 0usize;
    let mut last = 0usize;
    let mut n = 0usize;
    while pos <= text.len() && (count <= 0 || n < count as usize) {
        match captures(pat, text, pos) {
            Some(spans) => {
                let (s, e) = spans[0].unwrap();
                out.push_str(&text[last..s]);
                let replacement = match repl {
                    Py::Str(t) => expand_template(t, text, &spans, &pat.group_names),
                    Py::Func(_) | Py::BoundMethod(_) => {
                        let m = make_match(text, spans.clone(), pat);
                        let r = crate::builtins::py_call(repl, &[m]);
                        py_str_of(&r)
                    }
                    other => py_str_of(other),
                };
                out.push_str(&replacement);
                last = e;
                n += 1;
                pos = if e > s { e } else { e + 1 };
                if pos > text.len() { break; }
            }
            None => break,
        }
    }
    out.push_str(&text[last..]);
    (py_string(out), n)
}

pub fn escape(s: &str) -> String {
    // Python re.escape: escape all non-alphanumeric (unicode-aware: only ascii alnum kept)
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || c == '_' {
            out.push(c);
        } else if (c as u32) < 128 {
            out.push('\\');
            out.push(c);
        } else {
            out.push(c);
        }
    }
    out
}

/// re.fullmatch with fancy: anchor both ends
pub fn fullmatch(pat: &Rc<PyPattern>, text: &str, pos: usize) -> Py {
    match_at(pat, text, pos, true, true)
}
