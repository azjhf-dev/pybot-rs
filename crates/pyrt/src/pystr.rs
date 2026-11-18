//! Python str methods and formatting.
use std::rc::Rc;

use crate::exc::PyExc;
use crate::value::*;

pub const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
pub const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ASCII_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const DIGITS: &str = "0123456789";
pub const HEXDIGITS: &str = "0123456789abcdefABCDEF";
pub const WHITESPACE: &str = " \t\n\r\x0b\x0c";
pub const PRINTABLE: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r\x0b\x0c";

fn arg_str(args: &[Py], i: usize) -> Option<String> {
    args.get(i).and_then(|p| p.as_str().map(|s| s.to_string()))
}

fn py_whitespace(c: char) -> bool { c.is_whitespace() }

/// str.split with Python semantics: sep=None splits on runs of whitespace.
pub fn split(s: &str, sep: Option<&str>, maxsplit: i64) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    match sep {
        None => {
            let mut cur = String::new();
            let mut splits = 0i64;
            let mut in_word = false;
            for c in s.chars() {
                if py_whitespace(c) {
                    if in_word { out.push(std::mem::take(&mut cur)); splits += 1; in_word = false; }
                    if maxsplit >= 0 && splits >= maxsplit {
                        // remainder: skip whitespace then take rest raw
                        let rest: String = s.chars().skip(s.chars().count().min(0)).collect();
                        let _ = rest;
                        break;
                    }
                } else {
                    in_word = true;
                    cur.push(c);
                }
            }
            if in_word { out.push(cur); }
            if maxsplit >= 0 && out.len() as i64 > maxsplit + 1 {
                // redo with maxsplit honored precisely
                return split_ws_max(s, maxsplit);
            }
            out
        }
        Some(sep) if sep.is_empty() => out, // caller errors
        Some(sep) => {
            if maxsplit < 0 {
                s.split(sep).map(|x| x.to_string()).collect()
            } else {
                s.splitn((maxsplit + 1) as usize, sep).map(|x| x.to_string()).collect()
            }
        }
    }
}

fn split_ws_max(s: &str, maxsplit: i64) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = s.trim_start_matches(py_whitespace);
    let mut splits = 0;
    while !rest.is_empty() {
        if splits >= maxsplit {
            out.push(rest.to_string());
            return out;
        }
        match rest.find(py_whitespace) {
            None => { out.push(rest.to_string()); return out; }
            Some(i) => {
                out.push(rest[..i].to_string());
                rest = rest[i..].trim_start_matches(py_whitespace);
                splits += 1;
            }
        }
    }
    out
}

pub fn rsplit(s: &str, sep: Option<&str>, maxsplit: i64) -> Vec<String> {
    match sep {
        None => {
            if maxsplit < 0 { return split(s, None, -1); }
            let all = split(s, None, -1);
            if all.len() as i64 <= maxsplit + 1 { return all; }
            // find the split point for maxsplit from the right
            let mut out: Vec<String> = Vec::new();
            let mut rest = s.trim_end_matches(py_whitespace);
            let mut splits = 0;
            while !rest.is_empty() && splits < maxsplit {
                match rest.rfind(py_whitespace) {
                    None => break,
                    Some(i) => {
                        out.push(rest[i..].trim_start_matches(py_whitespace).to_string());
                        rest = rest[..i].trim_end_matches(py_whitespace);
                        splits += 1;
                    }
                }
            }
            out.push(rest.to_string());
            out.reverse();
            out
        }
        Some(sep) => {
            if maxsplit < 0 { return s.split(sep).map(|x| x.to_string()).collect(); }
            let parts: Vec<&str> = s.split(sep).collect();
            if parts.len() as i64 <= maxsplit + 1 { return parts.iter().map(|x| x.to_string()).collect(); }
            let keep = maxsplit as usize;
            let head = parts[..parts.len() - keep].join(sep);
            let mut out = vec![head];
            out.extend(parts[parts.len() - keep..].iter().map(|x| x.to_string()));
            out
        }
    }
}

fn strip_set(s: &str, chars: Option<&str>, left: bool, right: bool) -> String {
    let pred = |c: char| match chars {
        None => py_whitespace(c),
        Some(set) => set.contains(c),
    };
    let mut out = s;
    if left { out = out.trim_start_matches(pred); }
    if right { out = out.trim_end_matches(pred); }
    out.to_string()
}

fn starts_ends_with(s: &str, arg: &Py, prefix: bool) -> bool {
    let check = |p: &str| if prefix { s.starts_with(p) } else { s.ends_with(p) };
    match arg {
        Py::Str(p) => check(p),
        Py::Tuple(t) => t.iter().any(|x| x.as_str().map_or(false, &check)),
        _ => false,
    }
}

pub fn py_lower(s: &str) -> String {
    // Python lower() with Unicode simple case folding; Rust to_lowercase is close.
    s.chars().flat_map(|c| c.to_lowercase()).collect()
}
pub fn py_upper(s: &str) -> String {
    s.chars().flat_map(|c| c.to_uppercase()).collect()
}
pub fn py_title(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_alpha = false;
    for c in s.chars() {
        if c.is_alphabetic() {
            if prev_alpha { out.extend(c.to_lowercase()); } else { out.extend(c.to_uppercase()); }
            prev_alpha = true;
        } else {
            out.push(c);
            prev_alpha = false;
        }
    }
    out
}

pub fn str_method(s: &str, name: &str, args: &[Py]) -> PyResult {
    match name {
        "lower" | "casefold" => Ok(py_string(py_lower(s))),
        "upper" => Ok(py_string(py_upper(s))),
        "strip" => Ok(py_string(strip_set(s, arg_str(args, 0).as_deref(), true, true))),
        "lstrip" => Ok(py_string(strip_set(s, arg_str(args, 0).as_deref(), true, false))),
        "rstrip" => Ok(py_string(strip_set(s, arg_str(args, 0).as_deref(), false, true))),
        "split" => {
            let sep = match args.first() { None | Some(Py::None) => None, Some(Py::Str(x)) => Some(x.to_string()), Some(other) => return Err(crate::value::type_err(&format!("must be str or None, not {}", other.type_name()))) };
            if let Some(ref x) = sep { if x.is_empty() { return Err(crate::value::val_err("empty separator")); } }
            let maxsplit = args.get(1).and_then(|p| p.as_i64()).unwrap_or(-1);
            Ok(py_list(split(s, sep.as_deref(), maxsplit).into_iter().map(py_string).collect()))
        }
        "rsplit" => {
            let sep = match args.first() { None | Some(Py::None) => None, Some(Py::Str(x)) => Some(x.to_string()), Some(other) => return Err(crate::value::type_err(&format!("must be str or None, not {}", other.type_name()))) };
            if let Some(ref x) = sep { if x.is_empty() { return Err(crate::value::val_err("empty separator")); } }
            let maxsplit = args.get(1).and_then(|p| p.as_i64()).unwrap_or(-1);
            Ok(py_list(rsplit(s, sep.as_deref(), maxsplit).into_iter().map(py_string).collect()))
        }
        "splitlines" => {
            let keepends = args.first().map_or(false, |p| p.is_truthy());
            let mut out: Vec<Py> = Vec::new();
            let mut cur = String::new();
            let mut chars = s.chars().peekable();
            while let Some(c) = chars.next() {
                let is_break = c == '\n' || c == '\r' || c == '\u{85}' || c == '\u{2028}' || c == '\u{2029}'
                    || c == '\u{b}' || c == '\u{c}' || c == '\u{1c}' || c == '\u{1d}' || c == '\u{1e}';
                if is_break {
                    if keepends { cur.push(c); }
                    if c == '\r' && chars.peek() == Some(&'\n') {
                        if keepends { cur.push('\n'); }
                        chars.next();
                    }
                    out.push(py_string(std::mem::take(&mut cur)));
                } else {
                    cur.push(c);
                }
            }
            if !cur.is_empty() { out.push(py_string(cur)); }
            Ok(py_list(out))
        }
        "join" => {
            let parts = py_iter(args.first().ok_or_else(|| type_err("join needs an argument"))?)?;
            let mut strs = Vec::with_capacity(parts.len());
            for p in &parts {
                match p.as_str() { Some(x) => strs.push(x.to_string()), None => return Err(type_err("sequence item: expected str instance")) }
            }
            Ok(py_string(strs.join(s)))
        }
        "replace" => {
            let old = arg_str(args, 0).unwrap_or_default();
            let new = arg_str(args, 1).unwrap_or_default();
            let count = args.get(2).and_then(|p| p.as_i64()).unwrap_or(-1);
            if old.is_empty() {
                // Python inserts between every char
                let mut out = String::new();
                let chars: Vec<char> = s.chars().collect();
                let n = if count < 0 { chars.len() + 1 } else { (count as usize).min(chars.len() + 1) };
                let mut done = 0;
                for (i, c) in chars.iter().enumerate() {
                    if done < n { out.push_str(&new); done += 1; }
                    out.push(*c);
                    let _ = i;
                }
                if done < n { out.push_str(&new); }
                return Ok(py_string(out));
            }
            if count < 0 { Ok(py_string(s.replace(&old, &new))) } else { Ok(py_string(s.replacen(&old, &new, count as usize))) }
        }
        "startswith" => Ok(Py::Bool(starts_ends_with(s, args.first().unwrap_or(&Py::None), true))),
        "endswith" => Ok(Py::Bool(starts_ends_with(s, args.first().unwrap_or(&Py::None), false))),
        "find" => Ok(Py::Int(find_sub(s, args, false))),
        "rfind" => Ok(Py::Int(rfind_sub(s, args))),
        "index" => {
            let i = find_sub(s, args, false);
            if i < 0 { Err(val_err("substring not found")) } else { Ok(Py::Int(i)) }
        }
        "rindex" => {
            let i = rfind_sub(s, args);
            if i < 0 { Err(val_err("substring not found")) } else { Ok(Py::Int(i)) }
        }
        "count" => {
            let sub = arg_str(args, 0).unwrap_or_default();
            if sub.is_empty() { return Ok(Py::Int(s.chars().count() as i64 + 1)); }
            Ok(Py::Int(s.matches(&sub).count() as i64))
        }
        "isdigit" => Ok(Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_ascii_digit() || c.is_numeric()))),
        "isnumeric" => Ok(Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_numeric()))),
        "isdecimal" => Ok(Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))),
        "isalpha" => Ok(Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_alphabetic()))),
        "isalnum" => Ok(Py::Bool(!s.is_empty() && s.chars().all(|c| c.is_alphanumeric()))),
        "isspace" => Ok(Py::Bool(!s.is_empty() && s.chars().all(py_whitespace))),
        "isupper" => Ok(Py::Bool(s.chars().any(|c| c.is_uppercase()) && !s.chars().any(|c| c.is_lowercase()))),
        "islower" => Ok(Py::Bool(s.chars().any(|c| c.is_lowercase()) && !s.chars().any(|c| c.is_uppercase()))),
        "istitle" => {
            let mut prev_cased = false;
            let mut any = false;
            for c in s.chars() {
                if c.is_uppercase() {
                    if prev_cased { return Ok(Py::Bool(false)); }
                    prev_cased = true; any = true;
                } else if c.is_lowercase() {
                    if !prev_cased { return Ok(Py::Bool(false)); }
                    prev_cased = true; any = true;
                } else { prev_cased = false; }
            }
            Ok(Py::Bool(any))
        }
        "title" => Ok(py_string(py_title(s))),
        "capitalize" => {
            let mut chars = s.chars();
            match chars.next() {
                None => Ok(py_string(String::new())),
                Some(first) => Ok(py_string(format!("{}{}", first.to_uppercase().collect::<String>(), py_lower(chars.as_str())))),
            }
        }
        "swapcase" => Ok(py_string(s.chars().map(|c| if c.is_uppercase() { c.to_lowercase().collect::<String>() } else if c.is_lowercase() { c.to_uppercase().collect::<String>() } else { c.to_string() }).collect())),
        "partition" => {
            let sep = arg_str(args, 0).unwrap_or_default();
            match s.find(&sep) {
                Some(i) => Ok(py_tuple(vec![py_string(s[..i].to_string()), py_string(sep.clone()), py_string(s[i + sep.len()..].to_string())])),
                None => Ok(py_tuple(vec![py_string(s.to_string()), py_str(""), py_str("")])),
            }
        }
        "rpartition" => {
            let sep = arg_str(args, 0).unwrap_or_default();
            match s.rfind(&sep) {
                Some(i) => Ok(py_tuple(vec![py_string(s[..i].to_string()), py_string(sep.clone()), py_string(s[i + sep.len()..].to_string())])),
                None => Ok(py_tuple(vec![py_str(""), py_str(""), py_string(s.to_string())])),
            }
        }
        "zfill" => {
            let width = args.first().and_then(|p| p.as_i64()).unwrap_or(0).max(0) as usize;
            let len = s.chars().count();
            if len >= width { return Ok(py_string(s.to_string())); }
            let pad = width - len;
            let (sign, rest) = if s.starts_with('-') { ("-", &s[1..]) } else if s.starts_with('+') { ("+", &s[1..]) } else { ("", s) };
            Ok(py_string(format!("{}{}{}", sign, "0".repeat(pad), rest)))
        }
        "center" => {
            let width = args.first().and_then(|p| p.as_i64()).unwrap_or(0).max(0) as usize;
            let fill = arg_str(args, 1).and_then(|f| f.chars().next()).unwrap_or(' ');
            let len = s.chars().count();
            if len >= width { return Ok(py_string(s.to_string())); }
            let total = width - len;
            let left = total / 2 + (total & 1 & (width & 1 ^ 1));
            // CPython: left = (width - len) // 2 + ((width - len) & width & 1)? Simplify: put extra on right.
            let left = total / 2 + if (total % 2 == 1) && (width % 2 == 0) { 1 } else { 0 };
            let _ = left;
            let l = total / 2;
            let r = total - l;
            Ok(py_string(format!("{}{}{}", fill.to_string().repeat(l), s, fill.to_string().repeat(r))))
        }
        "ljust" => {
            let width = args.first().and_then(|p| p.as_i64()).unwrap_or(0).max(0) as usize;
            let fill = arg_str(args, 1).and_then(|f| f.chars().next()).unwrap_or(' ');
            let len = s.chars().count();
            if len >= width { Ok(py_string(s.to_string())) } else { Ok(py_string(format!("{}{}", s, fill.to_string().repeat(width - len)))) }
        }
        "rjust" => {
            let width = args.first().and_then(|p| p.as_i64()).unwrap_or(0).max(0) as usize;
            let fill = arg_str(args, 1).and_then(|f| f.chars().next()).unwrap_or(' ');
            let len = s.chars().count();
            if len >= width { Ok(py_string(s.to_string())) } else { Ok(py_string(format!("{}{}", fill.to_string().repeat(width - len), s))) }
        }
        "removeprefix" => {
            let p = arg_str(args, 0).unwrap_or_default();
            Ok(py_string(if s.starts_with(&p) { s[p.len()..].to_string() } else { s.to_string() }))
        }
        "removesuffix" => {
            let p = arg_str(args, 0).unwrap_or_default();
            Ok(py_string(if !p.is_empty() && s.ends_with(&p) { s[..s.len() - p.len()].to_string() } else { s.to_string() }))
        }
        "expandtabs" => {
            let tabsize = args.first().and_then(|p| p.as_i64()).unwrap_or(8).max(0) as usize;
            let mut out = String::new();
            let mut col = 0usize;
            for c in s.chars() {
                match c {
                    '\t' => { let n = tabsize - (col % tabsize.max(1)); if tabsize > 0 { out.push_str(&" ".repeat(n)); col += n; } }
                    '\n' | '\r' => { out.push(c); col = 0; }
                    c => { out.push(c); col += 1; }
                }
            }
            Ok(py_string(out))
        }
        "format" => format(s, args),
        "format_map" => format(s, args),
        "encode" => Ok(Py::Bytes(Rc::new(s.as_bytes().to_vec()))),
        "translate" => {
            let table = args.first().cloned().unwrap_or(Py::None);
            translate(s, &table)
        }
        "isidentifier" => {
            let mut chars = s.chars();
            match chars.next() {
                None => Ok(Py::Bool(false)),
                Some(c) if c.is_alphabetic() || c == '_' => Ok(Py::Bool(chars.all(|c| c.is_alphanumeric() || c == '_'))),
                _ => Ok(Py::Bool(false)),
            }
        }
        "isprintable" => Ok(Py::Bool(s.chars().all(|c| !c.is_control() || c == ' '))),
        "isascii" => Ok(Py::Bool(s.is_ascii())),
        "index_of" => Err(val_err("unknown")),
        _ => Err(attr_err(name)),
    }
}

fn find_sub(s: &str, args: &[Py], _rev: bool) -> i64 {
    let sub = match arg_str(args, 0) { Some(x) => x, None => return -1 };
    let chars: Vec<char> = s.chars().collect();
    let subc: Vec<char> = sub.chars().collect();
    let start = args.get(1).and_then(|p| p.as_i64()).unwrap_or(0);
    let end = args.get(2).and_then(|p| p.as_i64()).unwrap_or(chars.len() as i64);
    let n = chars.len() as i64;
    let lo = if start < 0 { (n + start).max(0) } else { start.min(n) } as usize;
    let hi = if end < 0 { (n + end).max(0) } else { end.min(n) } as usize;
    if subc.is_empty() { return lo.min(hi) as i64; }
    if lo > hi || subc.len() > hi - lo { return -1; }
    for i in lo..=(hi - subc.len()) {
        if chars[i..i + subc.len()] == subc[..] { return i as i64; }
    }
    -1
}

fn rfind_sub(s: &str, args: &[Py]) -> i64 {
    let sub = match arg_str(args, 0) { Some(x) => x, None => return -1 };
    let chars: Vec<char> = s.chars().collect();
    let subc: Vec<char> = sub.chars().collect();
    if subc.is_empty() { return chars.len() as i64; }
    if subc.len() > chars.len() { return -1; }
    for i in (0..=(chars.len() - subc.len())).rev() {
        if chars[i..i + subc.len()] == subc[..] { return i as i64; }
    }
    -1
}

fn translate(s: &str, table: &Py) -> PyResult {
    let mut out = String::with_capacity(s.len());
    'outer: for c in s.chars() {
        if let Py::Dict(d) = table {
            let key = Py::Int(c as i64);
            if let Some(v) = d.borrow().get(&PyKey(key)) {
                match v {
                    Py::None => continue 'outer,
                    Py::Int(i) => out.push(char::from_u32(*i as u32).unwrap_or(c)),
                    Py::Str(t) => out.push_str(t),
                    _ => out.push(c),
                }
            } else {
                out.push(c);
            }
        } else {
            out.push(c);
        }
    }
    Ok(py_string(out))
}

/// str.maketrans: dict from str->str/None, or (from, to) strings.
pub fn maketrans(args: &[Py]) -> Py {
    let mut m = Vec::new();
    if args.len() == 1 {
        if let Py::Dict(d) = &args[0] {
            for (k, v) in d.borrow().iter() {
                let code = match &k.0 {
                    Py::Str(s) => s.chars().next().map(|c| c as i64).unwrap_or(0),
                    Py::Int(i) => *i,
                    _ => 0,
                };
                m.push((Py::Int(code), v.clone()));
            }
        }
    } else if args.len() >= 2 {
        if let (Some(from), Some(to)) = (args[0].as_str(), args[1].as_str()) {
            for (a, b) in from.chars().zip(to.chars()) {
                m.push((Py::Int(a as i64), py_string(b.to_string())));
            }
        }
        if let Some(third) = args.get(2).and_then(|p| p.as_str()) {
            for c in third.chars() {
                m.push((Py::Int(c as i64), Py::None));
            }
        }
    }
    py_dict(m)
}

// ---------------------------------------------------------------- formatting
/// %-formatting: "%s %d" % (a, b)
pub fn percent_format(fmt: &str, value: &Py) -> PyResult {
    let values: Vec<Py> = match value {
        Py::Tuple(t) => (**t).clone(),
        Py::Dict(_) => vec![], // %(name)s lookup handled inline
        other => vec![other.clone()],
    };
    let dict = if let Py::Dict(_) = value { Some(value.clone()) } else { None };
    let mut out = String::new();
    let chars: Vec<char> = fmt.chars().collect();
    let mut i = 0;
    let mut argi = 0;
    while i < chars.len() {
        let c = chars[i];
        if c != '%' { out.push(c); i += 1; continue; }
        i += 1;
        if i >= chars.len() { return Err(val_err("incomplete format")); }
        let mut key: Option<String> = None;
        if chars[i] == '(' {
            let mut depth = 1;
            let mut k = String::new();
            i += 1;
            while i < chars.len() && depth > 0 {
                if chars[i] == '(' { depth += 1; } else if chars[i] == ')' { depth -= 1; if depth == 0 { i += 1; break; } }
                if depth > 0 { k.push(chars[i]); }
                i += 1;
            }
            key = Some(k);
        }
        // flags
        while i < chars.len() && matches!(chars[i], '#' | '0' | '-' | ' ' | '+') { i += 1; }
        // width
        while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '*') { i += 1; }
        // precision
        if i < chars.len() && chars[i] == '.' { i += 1; while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '*') { i += 1; } }
        // length modifiers
        while i < chars.len() && matches!(chars[i], 'h' | 'l' | 'L') { i += 1; }
        if i >= chars.len() { return Err(val_err("incomplete format")); }
        let conv = chars[i];
        i += 1;
        if conv == '%' { out.push('%'); continue; }
        let v = match &key {
            Some(k) => match &dict {
                Some(d) => py_getitem(d, &py_str(k))?,
                None => return Err(type_err("format requires a mapping")),
            },
            None => {
                if argi >= values.len() { return Err(type_err("not enough arguments for format string")); }
                let v = values[argi].clone();
                argi += 1;
                v
            }
        };
        let piece = match conv {
            's' => py_str_of(&v),
            'r' => py_repr(&v),
            'a' => py_repr(&v).chars().map(|c| if c.is_ascii() { c.to_string() } else { format!("\\u{:04x}", c as u32) }).collect(),
            'd' | 'i' | 'u' => v.as_i64().map(|x| x.to_string()).ok_or_else(|| type_err("%d format: a number is required"))?,
            'f' | 'F' => format!("{:.6}", v.as_f64().ok_or_else(|| type_err("float argument required"))?),
            'e' => format!("{:e}", v.as_f64().unwrap_or(0.0)),
            'g' => format_g(v.as_f64().unwrap_or(0.0), 6),
            'x' => format!("{:x}", v.as_i64().unwrap_or(0)),
            'X' => format!("{:X}", v.as_i64().unwrap_or(0)),
            'o' => format!("{:o}", v.as_i64().unwrap_or(0)),
            'c' => match &v {
                Py::Str(s) => s.to_string(),
                other => other.as_i64().and_then(|x| char::from_u32(x as u32)).map(|c| c.to_string()).unwrap_or_default(),
            },
            _ => return Err(val_err("unsupported format character")),
        };
        out.push_str(&piece);
    }
    Ok(py_string(out))
}

pub fn format_g(x: f64, prec: usize) -> String {
    if x == 0.0 { return "0".into(); }
    let exp = x.abs().log10().floor() as i32;
    if exp < -4 || exp >= prec as i32 {
        let s = format!("{:.*e}", prec.saturating_sub(1), x);
        s
    } else {
        let decimals = (prec as i32 - exp - 1).max(0) as usize;
        let s = format!("{:.*}", decimals, x);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Python format mini-language for one value. spec like ".1f", ",.2f", ">10", "02d", "%A" (dates), "g"
pub fn format_spec(value: &Py, spec: &str) -> Result<String, PyExc> {
    // Date/datetime strftime-style specs
    if let Py::Date(d) = value { return Ok(crate::pydatetime::strftime_date(d, spec)); }
    if let Py::DateTime(d) = value { return Ok(crate::pydatetime::strftime_datetime(d, spec)); }
    if let Py::TimeDelta(_) = value { return Ok(py_str_of(value)); }
    if spec.is_empty() { return Ok(py_str_of(value)); }
    let mut rest = spec;
    let mut fill = ' ';
    let mut align = '<';
    let mut sign = '-';
    let mut comma = false;
    let mut width: Option<usize> = None;
    let mut prec: Option<usize> = None;
    let mut ty = "";
    let rc: Vec<char> = rest.chars().collect();
    let mut i = 0;
    if rc.len() >= 2 && matches!(rc[1], '<' | '>' | '^' | '=') { fill = rc[0]; align = rc[1]; i = 2; }
    else if !rc.is_empty() && matches!(rc[0], '<' | '>' | '^' | '=') { align = rc[0]; i = 1; }
    if i < rc.len() && matches!(rc[i], '+' | '-' | ' ') { sign = rc[i]; i += 1; }
    if i < rc.len() && rc[i] == '#' { i += 1; }
    if i < rc.len() && rc[i] == '0' { if width.is_none() { fill = '0'; align = '='; } i += 1; }
    let mut w = String::new();
    while i < rc.len() && rc[i].is_ascii_digit() { w.push(rc[i]); i += 1; }
    if !w.is_empty() { width = w.parse().ok(); }
    if i < rc.len() && rc[i] == ',' { comma = true; i += 1; }
    if i < rc.len() && rc[i] == '.' { i += 1; let mut p = String::new(); while i < rc.len() && rc[i].is_ascii_digit() { p.push(rc[i]); i += 1; } prec = p.parse().ok().or(Some(0)); }
    if i < rc.len() { ty = &rest[rest.char_indices().nth(i).map(|(b, _)| b).unwrap_or(rest.len())..]; }
    let _ = ty;
    let ty_char = ty.chars().next().unwrap_or('\0');
    let mut body = match ty_char {
        'f' | 'F' => format!("{:.*}", prec.unwrap_or(6), value.as_f64().ok_or_else(|| type_err("float required"))?),
        'e' | 'E' => format!("{:.*e}", prec.unwrap_or(6), value.as_f64().unwrap_or(0.0)),
        'g' | 'G' => format_g(value.as_f64().unwrap_or(0.0), prec.unwrap_or(6)),
        'd' => value.as_i64().ok_or_else(|| type_err("int required"))?.to_string(),
        'x' => format!("{:x}", value.as_i64().unwrap_or(0)),
        'X' => format!("{:X}", value.as_i64().unwrap_or(0)),
        'o' => format!("{:o}", value.as_i64().unwrap_or(0)),
        'b' => format!("{:b}", value.as_i64().unwrap_or(0)),
        '%' => format!("{:.*}%", prec.unwrap_or(6), value.as_f64().unwrap_or(0.0) * 100.0),
        's' => {
            let mut s = py_str_of(value);
            if let Some(p) = prec { s = s.chars().take(p).collect(); }
            s
        }
        '\0' => {
            match value {
                Py::Float(f) => {
                    // default float formatting: str() unless precision given
                    match prec { Some(p) => format!("{:.*}", p, f), None => py_float_repr(*f) }
                }
                _ => py_str_of(value),
            }
        }
        _ => py_str_of(value),
    };
    if comma {
        if let Some(dot) = body.find('.') {
            let (int_part, frac) = body.split_at(dot);
            body = format!("{}{}", group_thousands(int_part), frac);
        } else {
            body = group_thousands(&body);
        }
    }
    if sign == '+' && !body.starts_with('-') { body = format!("+{}", body); }
    else if sign == ' ' && !body.starts_with('-') { body = format!(" {}", body); }
    if let Some(w) = width {
        let len = body.chars().count();
        if len < w {
            let pad = w - len;
            body = match align {
                '>' => format!("{}{}", fill.to_string().repeat(pad), body),
                '^' => { let l = pad / 2; let r = pad - l; format!("{}{}{}", fill.to_string().repeat(l), body, fill.to_string().repeat(r)) }
                '=' => {
                    if body.starts_with('-') || body.starts_with('+') {
                        let (s, rest) = body.split_at(1);
                        format!("{}{}{}", s, fill.to_string().repeat(pad), rest)
                    } else { format!("{}{}", fill.to_string().repeat(pad), body) }
                }
                _ => format!("{}{}", body, fill.to_string().repeat(pad)),
            };
        }
    }
    Ok(body)
}

fn group_thousands(int_part: &str) -> String {
    let (sign, digits) = if let Some(stripped) = int_part.strip_prefix('-') { ("-", stripped) } else { ("", int_part) };
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 { out.push(','); }
        out.push(c);
    }
    format!("{}{}", sign, out)
}

/// str.format with {} {0} {name} {:spec} {!r}
pub fn format(fmt: &str, args: &[Py]) -> PyResult {
    let chars: Vec<char> = fmt.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut auto = 0usize;
    while i < chars.len() {
        match chars[i] {
            '{' => {
                if i + 1 < chars.len() && chars[i + 1] == '{' { out.push('{'); i += 2; continue; }
                let mut depth = 0;
                let mut j = i + 1;
                let mut inner = String::new();
                while j < chars.len() {
                    if chars[j] == '{' { depth += 1; }
                    if chars[j] == '}' { if depth == 0 { break; } depth -= 1; }
                    inner.push(chars[j]);
                    j += 1;
                }
                if j >= chars.len() { return Err(val_err("single '{' encountered in format string")); }
                i = j + 1;
                // parse inner: field[!conv][:spec]
                let mut field = String::new();
                let mut conv: Option<char> = None;
                let mut spec = String::new();
                let mut k = 0;
                let ic: Vec<char> = inner.chars().collect();
                while k < ic.len() && !matches!(ic[k], '!' | ':') { field.push(ic[k]); k += 1; }
                if k < ic.len() && ic[k] == '!' { conv = ic.get(k + 1).copied(); k += 2; }
                if k < ic.len() && ic[k] == ':' { spec = ic[k + 1..].iter().collect(); }
                let value = if field.is_empty() {
                    let v = args.get(auto).cloned().unwrap_or(Py::None);
                    auto += 1;
                    v
                } else if let Ok(idx) = field.parse::<usize>() {
                    args.get(idx).cloned().unwrap_or(Py::None)
                } else {
                    // named field: look in Kwargs arg or attribute/index path
                    let mut base = Py::None;
                    let mut found = false;
                    for a in args {
                        if let Py::Kwargs(kws) = a {
                            for (n, v) in kws.iter() {
                                if n == &field { base = v.clone(); found = true; break; }
                            }
                        }
                        if found { break; }
                    }
                    if !found {
                        // field like "0.attr" or "0[key]"
                        base = resolve_field(&field, args)?;
                    }
                    base
                };
                let s = match conv {
                    Some('r') => py_repr(&value),
                    Some('s') => py_str_of(&value),
                    Some('a') => py_repr(&value),
                    _ => format_spec(&value, &spec)?,
                };
                out.push_str(&s);
            }
            '}' => {
                if i + 1 < chars.len() && chars[i + 1] == '}' { out.push('}'); i += 2; } else { return Err(val_err("single '}' encountered in format string")); }
            }
            c => { out.push(c); i += 1; }
        }
    }
    Ok(py_string(out))
}

fn resolve_field(field: &str, args: &[Py]) -> PyResult {
    // "0.x", "0['k']"
    let mut parts: Vec<(bool, String)> = Vec::new(); // (is_index, name)
    let mut cur = String::new();
    let fc: Vec<char> = field.chars().collect();
    let mut i = 0;
    while i < fc.len() {
        match fc[i] {
            '.' => { parts.push((false, std::mem::take(&mut cur))); i += 1; }
            '[' => {
                parts.push((true, std::mem::take(&mut cur)));
                i += 1;
                let mut key = String::new();
                while i < fc.len() && fc[i] != ']' { key.push(fc[i]); i += 1; }
                i += 1;
                parts.push((true, key.trim_matches(|c| c == '\'' || c == '"').to_string()));
            }
            c => { cur.push(c); i += 1; }
        }
    }
    if !cur.is_empty() { parts.push((false, cur)); }
    let mut value = Py::None;
    for (idx, (is_idx, name)) in parts.iter().enumerate() {
        if idx == 0 {
            value = if let Ok(n) = name.parse::<usize>() { args.get(n).cloned().unwrap_or(Py::None) } else { Py::None };
        } else if *is_idx {
            value = py_getitem(&value, &py_str(name)).unwrap_or(Py::None);
        } else {
            value = crate::builtins::py_getattr(&value, name).unwrap_or(Py::None);
        }
    }
    Ok(value)
}

/// f-string formatting: value + optional conversion char + spec.
pub fn fstring_value(value: &Py, conv: i32, spec: &str) -> Result<String, PyExc> {
    match conv {
        115 => Ok(py_str_of(value)),  // 's'
        114 => Ok(py_repr(&value)),   // 'r'
        97 => Ok(py_repr(&value)),    // 'a'
        _ => format_spec(value, spec),
    }
}
