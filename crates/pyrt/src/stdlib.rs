//! Stdlib module emulation: re, random, datetime, string, copy, math, sys, itertools, ...
use std::rc::Rc;

use unicode_normalization::UnicodeNormalization;

use crate::exc::PyExc;
use crate::pydatetime;
use crate::pyre;
use crate::pyrandom;
use crate::pystr;
use crate::builtins::*;
use crate::value::*;

/// Attribute *load* on a stdlib module (constants mostly).
pub fn stdlib_attr(module: &str, name: &str) -> PyResult {
    if module == "ast" && matches!(name,"Add"|"Sub"|"Mult"|"Div"|"Pow"|"Mod"|"UAdd"|"USub"|"Expression"|"Constant"|"BinOp"|"UnaryOp") { return Ok(py_str(&format!("ast.{}",name))); }
    match (module, name) {
        ("re", "I") | ("re", "IGNORECASE") => Ok(Py::Int(pyre::IGNORECASE)),
        ("re", "M") | ("re", "MULTILINE") => Ok(Py::Int(pyre::MULTILINE)),
        ("re", "S") | ("re", "DOTALL") => Ok(Py::Int(pyre::DOTALL)),
        ("re", "X") | ("re", "VERBOSE") => Ok(Py::Int(pyre::VERBOSE)),
        ("re", "U") | ("re", "UNICODE") => Ok(Py::Int(pyre::UNICODE)),
        ("re", "A") | ("re", "ASCII") => Ok(Py::Int(pyre::ASCII)),
        ("re", "L") | ("re", "LOCALE") => Ok(Py::Int(pyre::LOCALE)),
        ("re", "T") | ("re", "TEMPLATE") => Ok(Py::Int(pyre::T)),
        ("re", "Pattern") => Ok(py_str("re.Pattern")),
        ("re", "Match") => Ok(py_str("re.Match")),
        ("re", "error") => Ok(py_str("re.error")),
        ("string", "punctuation") => Ok(py_str(pystr::PUNCTUATION)),
        ("string", "ascii_lowercase") => Ok(py_str(pystr::ASCII_LOWERCASE)),
        ("string", "ascii_uppercase") => Ok(py_str(pystr::ASCII_UPPERCASE)),
        ("string", "ascii_letters") => Ok(py_str(pystr::ASCII_LETTERS)),
        ("string", "digits") => Ok(py_str(pystr::DIGITS)),
        ("string", "hexdigits") => Ok(py_str(pystr::HEXDIGITS)),
        ("string", "whitespace") => Ok(py_str(pystr::WHITESPACE)),
        ("string", "printable") => Ok(py_str(pystr::PRINTABLE)),
        ("math", "pi") => Ok(Py::Float(std::f64::consts::PI)),
        ("math", "e") => Ok(Py::Float(std::f64::consts::E)),
        ("math", "inf") => Ok(Py::Float(f64::INFINITY)),
        ("math", "nan") => Ok(Py::Float(f64::NAN)),
        ("sys", "argv") => Ok(py_list(vec![])),
        ("sys", "platform") => Ok(py_str("linux")),
        ("sys", "version") => Ok(py_str("3.12.0 (pybot-rs)")),
        ("sys", "maxsize") => Ok(Py::Int(i64::MAX)),
        ("sys", "stdin") => Ok(Py::None),
        ("sys", "stdout") => Ok(Py::None),
        ("sys", "stderr") => Ok(Py::None),
        ("sys", "path") => Ok(py_list(vec![])),
        ("sys", "modules") => Ok(py_dict(vec![])),
        ("sys", "recursionlimit") => Ok(Py::Int(1000)),
        ("os", "name") => Ok(py_str("posix")),
        ("os", "sep") => Ok(py_str("/")),
        ("os", "environ") => Ok(py_dict(vec![])),
        ("os", "path") => Ok(Py::Module(Rc::new(PyModule {
            name: "os.path".into(),
            get: Rc::new(|_| None),
            set: None,
        }))),
        ("calendar", "day_name") => Ok(py_list(DAY_NAMES.iter().map(|d| py_str(d)).collect())),
        ("calendar", "month_name") => {
            let mut v = vec![py_str("")];
            v.extend(MONTH_NAMES.iter().map(|d| py_str(d)));
            Ok(py_list(v))
        }
        ("calendar", "day_abbr") => Ok(py_list(DAY_ABBR.iter().map(|d| py_str(d)).collect())),
        ("calendar", "month_abbr") => {
            let mut v = vec![py_str("")];
            v.extend(MONTH_ABBR.iter().map(|d| py_str(d)));
            Ok(py_list(v))
        }
        // Callable attributes return native function wrappers
        _ => {
            if stdlib_has_call(module, name) {
                let m = module.to_string();
                let n = name.to_string();
                return Ok(py_func(&format!("{}.{}", module, name), move |args| {
                    match stdlib_call(&m, &n, args) {
                        Ok(v) => v,
                        Err(e) => crate::exc::raise(e),
                    }
                }));
            }
            Err(attr_err(name))
        }
    }
}

const DAY_NAMES: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
const DAY_ABBR: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
const MONTH_NAMES: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
const MONTH_ABBR: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

fn stdlib_has_call(module: &str, name: &str) -> bool {
    match module {
        "re" => matches!(name, "compile" | "search" | "match" | "fullmatch" | "findall" | "finditer" | "sub" | "subn" | "split" | "escape" | "purge"),
        "random" => matches!(name, "Random" | "seed" | "random" | "randint" | "randrange" | "choice" | "shuffle" | "uniform" | "sample" | "getrandbits" | "choices" | "triangular"),
        "datetime" => matches!(name, "date" | "datetime" | "timedelta" | "date.today" | "datetime.now" | "date.fromisoformat" | "datetime.fromisoformat" | "datetime.combine" | "time"),
        "string" => matches!(name, "capwords" | "maketrans"),
        "copy" => matches!(name, "deepcopy" | "copy"),
        "math" => matches!(name, "floor" | "ceil" | "sqrt" | "log" | "log10" | "log2" | "exp" | "pow" | "fabs" | "isnan" | "isinf" | "gcd" | "prod" | "factorial" | "trunc" | "fmod" | "sin" | "cos" | "tan" | "degrees" | "radians" | "hypot" | "isfinite"),
        "itertools" => matches!(name, "product" | "chain" | "combinations" | "permutations" | "zip_longest" | "count" | "cycle" | "repeat" | "takewhile" | "dropwhile" | "groupby" | "islice" | "accumulate" | "combinations_with_replacement" | "pairwise" | "starmap" | "filterfalse"),
        "functools" => matches!(name, "lru_cache" | "cache" | "reduce" | "partial" | "wraps"),
        "unicodedata" => matches!(name, "normalize" | "combining" | "category" | "name" | "east_asian_width"),
        "hashlib" => matches!(name, "md5" | "sha1" | "sha256" | "sha512"),
        "difflib" => matches!(name, "get_close_matches" | "SequenceMatcher"),
        "sys" => matches!(name, "exit" | "getrecursionlimit" | "setrecursionlimit" | "getsizeof"),
        "calendar" => matches!(name, "monthrange" | "isleap" | "month" | "weekday"),
        "importlib" => matches!(name, "import_module"),
        "pkgutil" => name == "iter_modules",
        "operator" => matches!(name, "add" | "sub" | "mul" | "truediv" | "pow" | "mod" | "floordiv" | "eq" | "ne" | "lt" | "le" | "gt" | "ge" | "itemgetter" | "attrgetter" | "contains" | "pos" | "neg" | "not_" | "and_" | "or_"),
        "ast" => matches!(name, "parse" | "literal_eval" | "walk"),
        "json" => matches!(name, "loads" | "dumps"),
        "textwrap" => matches!(name, "wrap" | "fill" | "shorten" | "dedent" | "indent"),
        "html" => matches!(name, "escape" | "unescape"),
        "urllib" => matches!(name, "parse.quote"),
        "time" => matches!(name, "time" | "sleep" | "monotonic" | "perf_counter" | "strftime" | "strptime" | "localtime" | "mktime"),
        _ => false,
    }
}

pub fn stdlib_call(module: &str, name: &str, args: &[Py]) -> PyResult {
    match module {
        "pkgutil" if name == "iter_modules" => {
            let paths = args.first().map(py_iter).transpose()?.unwrap_or_default();
            let package = paths.first().and_then(|p| p.as_str()).unwrap_or("");
            Ok(py_list(crate::xrefs::children(package).into_iter().map(|n| {
                let saved = n.clone();
                Py::Module(Rc::new(PyModule { name: n, set: None, get: Rc::new(move |a| match a { "name" => Some(py_str(&saved)), "ispkg" => Some(Py::Bool(false)), _ => None }) }))
            }).collect()))
        }
        "re" => re_call(name, args),
        "random" => pyrandom::call(name, args),
        "datetime" => pydatetime::datetime_call(&name.split('.').collect::<Vec<_>>(), args),
        "string" => match name {
            "capwords" => {
                let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
                let sep = args.get(1).and_then(|p| p.as_str());
                let words = pystr::split(s, sep, -1);
                Ok(py_string(words.iter().map(|w| {
                    let mut c = w.chars();
                    match c.next() {
                        None => String::new(),
                        Some(f) => format!("{}{}", f.to_uppercase().collect::<String>(), pystr::py_lower(c.as_str())),
                    }
                }).collect::<Vec<_>>().join(" ")))
            }
            "maketrans" => Ok(pystr::maketrans(args)),
            _ => Err(attr_err(name)),
        },
        "copy" => match name {
            "deepcopy" => Ok(py_deepcopy(args.first().unwrap_or(&Py::None))),
            "copy" => {
                match args.first().unwrap_or(&Py::None) {
                    Py::List(l) => Ok(py_list(l.borrow().clone())),
                    Py::Dict(d) => Ok(Py::Dict(Rc::new(std::cell::RefCell::new(d.borrow().clone())))),
                    Py::Set(s) => Ok(Py::Set(Rc::new(std::cell::RefCell::new(s.borrow().clone())))),
                    other => Ok(other.clone()),
                }
            }
            _ => Err(attr_err(name)),
        },
        "math" => math_call(name, args),
        "itertools" => itertools_call(name, args),
        "functools" => functools_call(name, args),
        "unicodedata" => unicodedata_call(name, args),
        "hashlib" => hashlib_call(name, args),
        "difflib" => difflib_call(name, args),
        "sys" => match name {
            "exit" => std::process::exit(args.first().and_then(|p| p.as_i64()).unwrap_or(0) as i32),
            "getrecursionlimit" => Ok(Py::Int(1000)),
            "setrecursionlimit" => Ok(Py::None),
            "getsizeof" => Ok(Py::Int(64)),
            _ => Err(attr_err(name)),
        },
        "calendar" => match name {
            "monthrange" => {
                let y = args.first().and_then(|p| p.as_i64()).unwrap_or(2000);
                let m = args.get(1).and_then(|p| p.as_i64()).unwrap_or(1);
                let first = pydatetime::PyDate { year: y, month: m, day: 1 };
                Ok(py_tuple(vec![Py::Int(first.weekday()), Py::Int(pydatetime::days_in_month(y, m))]))
            }
            "isleap" => Ok(Py::Bool(pydatetime::is_leap(args.first().and_then(|p| p.as_i64()).unwrap_or(2000)))),
            "weekday" => {
                let g = |i: usize| args.get(i).and_then(|p| p.as_i64()).unwrap_or(1);
                let d = pydatetime::PyDate { year: g(0), month: g(1), day: g(2) };
                Ok(Py::Int(d.weekday()))
            }
            _ => Err(attr_err(name)),
        },
        "importlib" => match name {
            "import_module" => crate::xrefs::import_module(args.first().and_then(|p| p.as_str()).unwrap_or(""), args.get(1).and_then(|p| p.as_str())),
            _ => Err(attr_err(name)),
        },
        "operator" => operator_call(name, args),
        "ast" => match name {
            "parse" => crate::pyast::parse(args.first().and_then(|v|v.as_str()).unwrap_or("")),
            "literal_eval" => literal_eval(args.first().and_then(|p| p.as_str()).unwrap_or("")),
            _ => Err(attr_err(name)),
        },
        "json" => match name {
            "dumps" => Ok(py_string(json_dumps(args.first().unwrap_or(&Py::None)))),
            "loads" => Err(val_err("json.loads is not implemented in pybot-rs")),
            _ => Err(attr_err(name)),
        },
        "textwrap" => match name {
            "dedent" => {
                let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
                let lines: Vec<&str> = s.lines().collect();
                let indent = lines.iter().filter(|l| !l.trim().is_empty()).map(|l| l.len() - l.trim_start().len()).min().unwrap_or(0);
                Ok(py_string(lines.iter().map(|l| if l.len() >= indent { &l[indent..] } else { "" }).collect::<Vec<_>>().join("\n")))
            }
            "shorten" => {
                let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
                let w = args.get(1).and_then(|p| p.as_i64()).unwrap_or(70) as usize;
                let one = pystr::split(s, None, -1).join(" ");
                if one.len() <= w { Ok(py_string(one)) } else { Ok(py_string(format!("{} [...]", &one[..w.saturating_sub(6).max(0)]))) }
            }
            "fill" | "wrap" => {
                let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
                let w = args.get(1).and_then(|p| p.as_i64()).unwrap_or(70).max(1) as usize;
                let words = pystr::split(s, None, -1);
                let mut lines: Vec<String> = Vec::new();
                let mut cur = String::new();
                for word in words {
                    if !cur.is_empty() && cur.len() + 1 + word.len() > w { lines.push(std::mem::take(&mut cur)); }
                    if !cur.is_empty() { cur.push(' '); }
                    cur.push_str(&word);
                }
                if !cur.is_empty() { lines.push(cur); }
                if name == "fill" { Ok(py_string(lines.join("\n"))) } else { Ok(py_list(lines.into_iter().map(py_string).collect())) }
            }
            "indent" => {
                let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
                let prefix = args.get(1).and_then(|p| p.as_str()).unwrap_or("");
                Ok(py_string(s.lines().map(|l| format!("{}{}", prefix, l)).collect::<Vec<_>>().join("\n")))
            }
            _ => Err(attr_err(name)),
        },
        "html" => match name {
            "escape" => {
                let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
                Ok(py_string(s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#x27;")))
            }
            "unescape" => {
                let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
                Ok(py_string(s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&#x27;", "'").replace("&#39;", "'").replace("&amp;", "&")))
            }
            _ => Err(attr_err(name)),
        },
        "time" => match name {
            "time" => Ok(Py::Float(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0))),
            "monotonic" | "perf_counter" => Ok(Py::Float(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0))),
            "sleep" => { std::thread::sleep(std::time::Duration::from_secs_f64(args.first().and_then(|p| p.as_f64()).unwrap_or(0.0).max(0.0))); Ok(Py::None) }
            _ => Err(attr_err(name)),
        },
        _ => Err(attr_err(module)),
    }
}

fn re_call(name: &str, args: &[Py]) -> PyResult {
    if name=="escape" {return Ok(py_string(pyre::escape(args.first().and_then(|v|v.as_str()).unwrap_or(""))));}
    if name=="purge" {return Ok(Py::None);}
    let pat_arg = args.first().cloned().unwrap_or(Py::None);
    let flag_idx = match name {
        "compile" => 1,
        "search" | "match" | "fullmatch" | "findall" | "finditer" => 2,
        "split" => 3,
        "sub" | "subn" => 4,
        _ => 99,
    };
    let kw_flags = args.iter().find_map(|a| {
        if let Py::Kwargs(kws) = a { kws.iter().find(|(n, _)| n == "flags").and_then(|(_, v)| v.as_i64()) } else { None }
    });
    let flags = kw_flags.or_else(|| args.get(flag_idx).and_then(|a| a.as_i64())).unwrap_or(0);
    let pat = match &pat_arg {
        Py::Pattern(p) => p.clone(),
        Py::Str(s) => pyre::compile(s, flags)?,
        _ => return Err(type_err("first argument must be a string or compiled pattern")),
    };
    let text = args.get(1).and_then(|p| p.as_str()).unwrap_or("").to_string();
    match name {
        "compile" => Ok(Py::Pattern(pat)),
        "search" => Ok(pyre::search(&pat, &text, 0)),
        "match" => Ok(pyre::match_at(&pat, &text, 0, true, false)),
        "fullmatch" => Ok(pyre::fullmatch(&pat, &text, 0)),
        "findall" => Ok(pyre::findall(&pat, &text)),
        "finditer" => Ok(pyre::finditer(&pat, &text)),
        "split" => {
            let maxsplit = args.get(2).and_then(|p| p.as_i64()).unwrap_or(0);
            Ok(pyre::split(&pat, &text, maxsplit))
        }
        "sub" => {
            // re.sub(pattern, repl, string, count=0, flags=0)
            let repl = args.get(1).cloned().unwrap_or(Py::None);
            let text = args.get(2).and_then(|p| p.as_str()).unwrap_or("").to_string();
            let count = args.get(3).and_then(|p| p.as_i64()).unwrap_or(0);
            Ok(pyre::sub(&pat, &repl, &text, count))
        }
        "subn" => {
            let repl = args.get(1).cloned().unwrap_or(Py::None);
            let text = args.get(2).and_then(|p| p.as_str()).unwrap_or("").to_string();
            let count = args.get(3).and_then(|p| p.as_i64()).unwrap_or(0);
            Ok(pyre::subn(&pat, &repl, &text, count))
        }
        "escape" => Ok(py_string(pyre::escape(args.first().and_then(|p| p.as_str()).unwrap_or("")))),
        "purge" => Ok(Py::None),
        _ => Err(attr_err(name)),
    }
}

fn math_call(name: &str, args: &[Py]) -> PyResult {
    let f = |i: usize| args.get(i).and_then(|p| p.as_f64()).unwrap_or(0.0);
    let r = match name {
        "floor" => return Ok(Py::Int(f(0).floor() as i64)),
        "ceil" => return Ok(Py::Int(f(0).ceil() as i64)),
        "trunc" => return Ok(Py::Int(f(0).trunc() as i64)),
        "sqrt" => f(0).sqrt(),
        "log" => if args.len() > 1 { f(0).log(f(1)) } else { f(0).ln() },
        "log10" => f(0).log10(),
        "log2" => f(0).log2(),
        "exp" => f(0).exp(),
        "pow" => f(0).powf(f(1)),
        "fabs" => f(0).abs(),
        "isnan" => return Ok(Py::Bool(f(0).is_nan())),
        "isinf" => return Ok(Py::Bool(f(0).is_infinite())),
        "isfinite" => return Ok(Py::Bool(f(0).is_finite())),
        "gcd" => {
            let mut g = args.first().and_then(|p| p.as_i64()).unwrap_or(0).abs();
            for a in &args[1..] {
                let mut b = a.as_i64().unwrap_or(0).abs();
                while b != 0 { let t = g % b; g = b; b = t; }
            }
            return Ok(Py::Int(g));
        }
        "factorial" => {
            let n = args.first().and_then(|p| p.as_i64()).unwrap_or(0);
            let mut out = 1i64;
            for i in 2..=n { out *= i; }
            return Ok(Py::Int(out));
        }
        "prod" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let mut acc = Py::Int(1);
            for x in items { acc = py_mul(&acc, &x)?; }
            return Ok(acc);
        }
        "fmod" => f(0) % f(1),
        "sin" => f(0).sin(),
        "cos" => f(0).cos(),
        "tan" => f(0).tan(),
        "degrees" => f(0).to_degrees(),
        "radians" => f(0).to_radians(),
        "hypot" => f(0).hypot(f(1)),
        _ => return Err(attr_err(name)),
    };
    Ok(Py::Float(r))
}

fn itertools_call(name: &str, args: &[Py]) -> PyResult {
    match name {
        "product" => {
            let pools: Vec<Vec<Py>> = args.iter().map(py_iter).collect::<Result<_, _>>()?;
            let mut out: Vec<Vec<Py>> = vec![vec![]];
            for pool in &pools {
                let mut next = Vec::new();
                for prefix in &out {
                    for item in pool {
                        let mut p = prefix.clone();
                        p.push(item.clone());
                        next.push(p);
                    }
                }
                out = next;
            }
            Ok(py_list(out.into_iter().map(py_tuple).collect()))
        }
        "chain" => {
            let mut out = Vec::new();
            for a in args { out.extend(py_iter(a)?); }
            Ok(py_list(out))
        }
        "combinations" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let r = args.get(1).and_then(|p| p.as_i64()).unwrap_or(2).max(0) as usize;
            let mut out = Vec::new();
            let n = items.len();
            let mut idx: Vec<usize> = (0..r).collect();
            if r > n { return Ok(py_list(vec![])); }
            loop {
                out.push(py_tuple(idx.iter().map(|&i| items[i].clone()).collect()));
                let mut i = r;
                loop {
                    if i == 0 { return Ok(py_list(out)); }
                    i -= 1;
                    if idx[i] != i + n - r { break; }
                }
                idx[i] += 1;
                for j in i + 1..r { idx[j] = idx[j - 1] + 1; }
            }
        }
        "permutations" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let r = args.get(1).and_then(|p| p.as_i64()).map(|x| x as usize).unwrap_or(items.len());
            let mut out = Vec::new();
            permute(&items, r, &mut vec![], &mut vec![false; items.len()], &mut out);
            Ok(py_list(out))
        }
        "zip_longest" => {
            let fill = args.iter().find_map(|a| if let Py::Kwargs(kws) = a { kws.iter().find(|(n, _)| n == "fillvalue").map(|(_, v)| v.clone()) } else { None }).unwrap_or(Py::None);
            let iters: Vec<Vec<Py>> = args.iter().filter(|a| !matches!(a, Py::Kwargs(_))).map(py_iter).collect::<Result<_, _>>()?;
            let n = iters.iter().map(|v| v.len()).max().unwrap_or(0);
            let mut out = Vec::with_capacity(n);
            for i in 0..n {
                out.push(py_tuple(iters.iter().map(|v| v.get(i).cloned().unwrap_or_else(|| fill.clone())).collect()));
            }
            Ok(py_list(out))
        }
        "repeat" => {
            let v = args.first().cloned().unwrap_or(Py::None);
            let n = args.get(1).and_then(|p| p.as_i64()).unwrap_or(1);
            Ok(py_list((0..n).map(|_| v.clone()).collect()))
        }
        "islice" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let g = |i: usize| args.get(i).and_then(|p| p.as_i64());
            let (start, stop, step) = match args.len() {
                2 => (0, g(1).unwrap_or(0), 1),
                _ => (g(1).unwrap_or(0), g(2).unwrap_or(items.len() as i64), g(3).unwrap_or(1)),
            };
            let mut out = Vec::new();
            let mut i = start;
            while i < stop && (i as usize) < items.len() {
                out.push(items[i as usize].clone());
                i += step;
            }
            Ok(py_list(out))
        }
        "takewhile" => {
            let f = args.first().unwrap_or(&Py::None);
            let items = py_iter(args.get(1).unwrap_or(&Py::None))?;
            let mut out = Vec::new();
            for x in items {
                if py_call(f, std::slice::from_ref(&x)).is_truthy() { out.push(x); } else { break; }
            }
            Ok(py_list(out))
        }
        "dropwhile" => {
            let f = args.first().unwrap_or(&Py::None);
            let items = py_iter(args.get(1).unwrap_or(&Py::None))?;
            let mut out = Vec::new();
            let mut dropping = true;
            for x in items {
                if dropping && py_call(f, std::slice::from_ref(&x)).is_truthy() { continue; }
                dropping = false;
                out.push(x);
            }
            Ok(py_list(out))
        }
        "pairwise" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let mut out = Vec::new();
            for w in items.windows(2) { out.push(py_tuple(w.to_vec())); }
            Ok(py_list(out))
        }
        "accumulate" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let mut out = Vec::new();
            let mut acc: Option<Py> = None;
            for x in items {
                acc = Some(match acc { None => x, Some(a) => py_add(&a, &x)? });
                out.push(acc.clone().unwrap());
            }
            Ok(py_list(out))
        }
        _ => Err(attr_err(name)),
    }
}

fn permute(items: &[Py], r: usize, cur: &mut Vec<Py>, used: &mut Vec<bool>, out: &mut Vec<Py>) {
    if cur.len() == r { out.push(py_tuple(cur.clone())); return; }
    for (i, item) in items.iter().enumerate() {
        if used[i] { continue; }
        used[i] = true;
        cur.push(item.clone());
        permute(items, r, cur, used, out);
        cur.pop();
        used[i] = false;
    }
}

fn functools_call(name: &str, args: &[Py]) -> PyResult {
    match name {
        "lru_cache" | "cache" => {
            // Used as decorator (possibly with kwargs): return identity decorator
            Ok(py_func("lru_cache", |args| args.first().cloned().unwrap_or(Py::None)))
        }
        "reduce" => {
            let f = args.first().unwrap_or(&Py::None);
            let items = py_iter(args.get(1).unwrap_or(&Py::None))?;
            let mut iter = items.into_iter();
            let mut acc = match args.get(2) {
                Some(v) => v.clone(),
                None => match iter.next() { Some(v) => v, None => return Err(type_err("reduce() of empty iterable with no initial value")) },
            };
            for x in iter { acc = py_call(f, &[acc, x]); }
            Ok(acc)
        }
        "partial" => {
            let f = args.first().cloned().unwrap_or(Py::None);
            let bound: Vec<Py> = args[1..].to_vec();
            Ok(py_func("partial", move |args| {
                let mut full = bound.clone();
                full.extend_from_slice(args);
                py_call(&f, &full)
            }))
        }
        "wraps" => Ok(py_func("wraps", |args| args.first().cloned().unwrap_or(Py::None))),
        _ => Err(attr_err(name)),
    }
}

fn unicodedata_call(name: &str, args: &[Py]) -> PyResult {
    match name {
        "normalize" => {
            let form = args.first().and_then(|p| p.as_str()).unwrap_or("NFC");
            let s = args.get(1).and_then(|p| p.as_str()).unwrap_or("");
            let out: String = match form {
                "NFC" => s.nfc().collect(),
                "NFD" => s.nfd().collect(),
                "NFKC" => s.nfkc().collect(),
                "NFKD" => s.nfkd().collect(),
                _ => s.to_string(),
            };
            Ok(py_string(out))
        }
        "combining" => {
            let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
            let c = s.chars().next().unwrap_or('\0');
            Ok(Py::Int(unicode_normalization::char::canonical_combining_class(c) as i64))
        }
        "category" => Ok(py_str("Lo")),
        "name" => Ok(py_str("")),
        "east_asian_width" => Ok(py_str("N")),
        _ => Err(attr_err(name)),
    }
}

fn hashlib_call(name: &str, args: &[Py]) -> PyResult {
    // FNV-based stand-in: only used for cache keys in this codebase, never for security.
    let data = match args.first() {
        Some(Py::Bytes(b)) => (**b).clone(),
        Some(Py::Str(s)) => s.as_bytes().to_vec(),
        _ => vec![],
    };
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in &data { h = (h ^ b as u64).wrapping_mul(0x100000001b3); }
    let digest: Vec<u8> = (0..16).map(|i| ((h >> ((i % 8) * 8)) as u8) ^ (i as u8 * 37)).collect();
    let hex: String = digest.iter().map(|b| format!("{:02x}", b)).collect();
    let _ = name;
    Ok(Py::Instance(Rc::new(PyInstance {
        class_name: "hashlib.HASH".into(),
        attrs: Rc::new(std::cell::RefCell::new(indexmap::IndexMap::from([
            ("hexdigest".to_string(), py_func("hexdigest", move |_| py_str(&hex))),
            ("digest".to_string(), py_func("digest", move |_| Py::Bytes(Rc::new(digest.clone())))),
        ]))),
    })))
}

fn difflib_call(name: &str, args: &[Py]) -> PyResult {
    match name {
        "get_close_matches" => {
            let word = args.first().and_then(|p| p.as_str()).unwrap_or("");
            let candidates = py_iter(args.get(1).unwrap_or(&Py::None))?;
            let n = args.get(2).and_then(|p| p.as_i64()).unwrap_or(3) as usize;
            let cutoff = args.get(3).and_then(|p| p.as_f64()).unwrap_or(0.6);
            let mut scored: Vec<(f64, Py)> = candidates.iter().map(|c| {
                let s = c.as_str().unwrap_or("");
                (sequence_ratio(word, s), c.clone())
            }).filter(|(r, _)| *r >= cutoff).collect();
            scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
            scored.truncate(n);
            Ok(py_list(scored.into_iter().map(|(_, c)| c).collect()))
        }
        _ => Err(attr_err(name)),
    }
}

/// difflib.SequenceMatcher.ratio() on chars.
pub fn sequence_ratio(a: &str, b: &str) -> f64 {
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    if ac.is_empty() && bc.is_empty() { return 1.0; }
    let m = matching_blocks(&ac, &bc);
    2.0 * m as f64 / (ac.len() + bc.len()) as f64
}

fn matching_blocks(a: &[char], b: &[char]) -> usize {
    // longest contiguous matching block, recursively (Ratcliff/Obershelp)
    let mut best = (0usize, 0usize, 0usize); // (i, j, len)
    for (i, ca) in a.iter().enumerate() {
        let mut run = 0;
        for (j, cb) in b.iter().enumerate() {
            if ca == cb {
                run += 1;
                if run > best.2 { best = (i + 1 - run, j + 1 - run, run); }
            } else {
                run = 0;
            }
        }
    }
    if best.2 == 0 { return 0; }
    let (i, j, l) = best;
    matching_blocks(&a[..i], &b[..j]) + l + matching_blocks(&a[i + l..], &b[j + l..])
}

fn operator_call(name: &str, args: &[Py]) -> PyResult {
    let a = args.first().unwrap_or(&Py::None);
    let b = args.get(1).unwrap_or(&Py::None);
    match name {
        "add" => py_add(a, b),
        "sub" => py_sub(a, b),
        "mul" => py_mul(a, b),
        "truediv" => py_truediv(a, b),
        "floordiv" => py_floordiv(a, b),
        "mod" => py_mod(a, b),
        "pow" => py_pow(a, b),
        "eq" => Ok(Py::Bool(py_eq(a, b))),
        "ne" => Ok(Py::Bool(!py_eq(a, b))),
        "lt" => Ok(Py::Bool(py_cmp(a, b, "<")? == std::cmp::Ordering::Less)),
        "le" => Ok(Py::Bool(py_cmp(a, b, "<=")? != std::cmp::Ordering::Greater)),
        "gt" => Ok(Py::Bool(py_cmp(a, b, ">")? == std::cmp::Ordering::Greater)),
        "ge" => Ok(Py::Bool(py_cmp(a, b, ">=")? != std::cmp::Ordering::Less)),
        "contains" => Ok(Py::Bool(py_contains(a, b))),
        "pos" => Ok(a.clone()),
        "neg" => py_neg(a),
        "not_" => Ok(Py::Bool(!a.is_truthy())),
        "and_" => py_bitand(a, b),
        "or_" => py_bitor(a, b),
        _ => Err(attr_err(name)),
    }
}

fn literal_eval(s: &str) -> PyResult {
    // minimal: strings, numbers, containers
    let t = s.trim();
    if t.starts_with('\'') || t.starts_with('"') {
        let inner = &t[1..t.len() - 1];
        return Ok(py_string(inner.replace("\\'", "'").replace("\\n", "\n").replace("\\\\", "\\")));
    }
    if let Ok(i) = t.parse::<i64>() { return Ok(Py::Int(i)); }
    if let Ok(f) = t.parse::<f64>() { return Ok(Py::Float(f)); }
    match t { "None" => return Ok(Py::None), "True" => return Ok(Py::Bool(true)), "False" => return Ok(Py::Bool(false)), _ => {} }
    Err(val_err("malformed literal"))
}

fn json_dumps(v: &Py) -> String {
    match v {
        Py::None => "null".into(),
        Py::Bool(b) => b.to_string(),
        Py::Int(i) => i.to_string(),
        Py::Float(f) => py_float_repr(*f),
        Py::Str(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")),
        Py::List(l) => format!("[{}]", l.borrow().iter().map(json_dumps).collect::<Vec<_>>().join(", ")),
        Py::Tuple(t) => format!("[{}]", t.iter().map(json_dumps).collect::<Vec<_>>().join(", ")),
        Py::Dict(d) => format!("{{{}}}", d.borrow().iter().map(|(k, v)| format!("\"{}\": {}", py_str_of(&k.0).replace('"', "\\\""), json_dumps(v))).collect::<Vec<_>>().join(", ")),
        other => format!("\"{}\"", py_str_of(other)),
    }
}

/// A first-class stdlib module used by function-local imports.
pub fn stdlib_module(name: &str) -> Py {
    let module = name.to_string();
    Py::Module(Rc::new(PyModule { name: name.to_string(), set: None,
        get: Rc::new(move |attr| {
            if let Ok(value) = stdlib_attr(&module, attr) { return Some(value); }
            let m = module.clone(); let a = attr.to_string();
            Some(py_func(attr, move |args| stdlib_call(&m, &a, args).unwrap_or_else(|e| crate::exc::raise(e))))
        })
    }))
}
