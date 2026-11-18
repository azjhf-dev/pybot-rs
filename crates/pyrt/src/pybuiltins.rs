//! Python builtin functions.
use std::rc::Rc;

use crate::exc::{self, PyExc};
use crate::pystr;
use crate::builtins::*;
use crate::value::*;

fn kw<'a>(args: &'a [Py], name: &str) -> Option<&'a Py> {
    for a in args {
        if let Py::Kwargs(kws) = a {
            for (n, v) in kws.iter() {
                if n == name { return Some(v); }
            }
        }
    }
    None
}
fn pos_args(args: &[Py]) -> Vec<&Py> {
    args.iter().filter(|a| !matches!(a, Py::Kwargs(_))).collect()
}

pub fn call_builtin(name: &str, args: &[Py]) -> PyResult {
    match name {
        "len" => Ok(Py::Int(py_len(args.first().unwrap_or(&Py::None))?)),
        "str" => Ok(py_string(args.first().map_or(String::new(), |v| py_str_of(v)))),
        "repr" => Ok(py_string(py_repr(args.first().unwrap_or(&Py::None)))),
        "int" => py_int(args),
        "float" => py_float(args),
        "bool" => Ok(Py::Bool(args.first().map_or(false, |v| v.is_truthy()))),
        "list" => Ok(py_list(match args.first() { None => vec![], Some(v) => py_iter(v)? })),
        "tuple" => Ok(py_tuple(match args.first() { None => vec![], Some(v) => py_iter(v)? })),
        "set" => {
            let items = match args.first() { None => vec![], Some(v) => py_iter(v)? };
            Ok(py_set(items))
        }
        "frozenset" => {
            let items = match args.first() { None => vec![], Some(v) => py_iter(v)? };
            Ok(py_frozenset(items))
        }
        "dict" => {
            let mut out = Vec::new();
            if let Some(first) = args.first() {
                match first {
                    Py::Kwargs(_) => {}
                    Py::Dict(d) => { for (k, v) in d.borrow().iter() { out.push((k.0.clone(), v.clone())); } }
                    other => {
                        for pair in py_iter(other)? {
                            let kv = py_iter(&pair)?;
                            if kv.len() == 2 { out.push((kv[0].clone(), kv[1].clone())); }
                        }
                    }
                }
            }
            if let Some(Py::Kwargs(kws)) = args.iter().find(|a| matches!(a, Py::Kwargs(_))) {
                for (n, v) in kws.iter() { out.push((py_str(n), v.clone())); }
            }
            Ok(py_dict(out))
        }
        "isinstance" => {
            let v = args.first().unwrap_or(&Py::None);
            let t = args.get(1).unwrap_or(&Py::None);
            py_isinstance(v, t)
        }
        "type" => Ok(match args.first() { Some(Py::Instance(i)) => py_str(&i.class_name), other => py_str(other.unwrap_or(&Py::None).type_name()) }),
        "any" => {
            for x in py_iter(args.first().unwrap_or(&Py::None))? {
                if x.is_truthy() { return Ok(Py::Bool(true)); }
            }
            Ok(Py::Bool(false))
        }
        "all" => {
            for x in py_iter(args.first().unwrap_or(&Py::None))? {
                if !x.is_truthy() { return Ok(Py::Bool(false)); }
            }
            Ok(Py::Bool(true))
        }
        "enumerate" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let start = args.get(1).and_then(|p| p.as_i64()).or(kw(args, "start").and_then(|p| p.as_i64())).unwrap_or(0);
            Ok(py_list(items.into_iter().enumerate().map(|(i, x)| py_tuple(vec![Py::Int(i as i64 + start), x])).collect()))
        }
        "range" => {
            let p = pos_args(args);
            let g = |i: usize| p.get(i).and_then(|v| v.as_i64());
            let (start, stop, step) = match p.len() {
                1 => (0, g(0).unwrap_or(0), 1),
                2 => (g(0).unwrap_or(0), g(1).unwrap_or(0), 1),
                _ => (g(0).unwrap_or(0), g(1).unwrap_or(0), g(2).unwrap_or(1)),
            };
            if step == 0 { return Err(val_err("range() arg 3 must not be zero")); }
            Ok(Py::Range(Rc::new(PyRange { start, stop, step })))
        }
        "reversed" => {
            let mut items = py_iter(args.first().unwrap_or(&Py::None))?;
            items.reverse();
            Ok(py_list(items))
        }
        "sorted" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let key = kw(args, "key").filter(|v| !v.is_none()).cloned();
            let reverse = kw(args, "reverse").map_or(false, |v| v.is_truthy());
            let mut keyed: Vec<(Py, Py)> = items.into_iter().map(|x| {
                let k = match &key { Some(f) => py_call(f, std::slice::from_ref(&x)), None => x.clone() };
                (k, x)
            }).collect();
            keyed.sort_by(|a, b| if reverse { py_cmp_total(&b.0, &a.0) } else { py_cmp_total(&a.0, &b.0) });
            Ok(py_list(keyed.into_iter().map(|(_, x)| x).collect()))
        }
        "min" | "max" => {
            let p = pos_args(args);
            let items = if p.len() == 1 { py_iter(p[0])? } else { p.iter().map(|v| (*v).clone()).collect() };
            if items.is_empty() {
                if let Some(d) = kw(args, "default") { return Ok(d.clone()); }
                return Err(val_err("min()/max() arg is an empty sequence"));
            }
            let key = kw(args, "key").filter(|v| !v.is_none()).cloned();
            let mut best = items[0].clone();
            let mut best_k = match &key { Some(f) => py_call(f, std::slice::from_ref(&best)), None => best.clone() };
            for x in items.into_iter().skip(1) {
                let kx = match &key { Some(f) => py_call(f, std::slice::from_ref(&x)), None => x.clone() };
                let better = if name == "min" { py_cmp_total(&kx, &best_k) == std::cmp::Ordering::Less } else { py_cmp_total(&kx, &best_k) == std::cmp::Ordering::Greater };
                if better { best = x; best_k = kx; }
            }
            Ok(best)
        }
        "sum" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            let mut acc = args.get(1).cloned().unwrap_or(Py::Int(0));
            for x in items { acc = py_add(&acc, &x)?; }
            Ok(acc)
        }
        "abs" => {
            let v = args.first().unwrap_or(&Py::None);
            match v {
                Py::Int(i) => Ok(Py::Int(i.abs())),
                Py::Bool(b) => Ok(Py::Int(*b as i64)),
                Py::Float(f) => Ok(Py::Float(f.abs())),
                _ => Err(type_err("bad operand type for abs()")),
            }
        }
        "round" => {
            let v = args.first().unwrap_or(&Py::None);
            let ndigits = args.get(1).and_then(|p| p.as_i64());
            let f = v.as_f64().ok_or_else(|| type_err("round needs a number"))?;
            match ndigits {
                None => Ok(Py::Int(round_half_even(f, 0) as i64)),
                Some(n) => {
                    let r = round_half_even(f, n);
                    if matches!(v, Py::Int(_)) && n <= 0 { Ok(Py::Int(r as i64)) } else { Ok(Py::Float(r)) }
                }
            }
        }
        "zip" => {
            let iters: Vec<Vec<Py>> = args.iter().filter(|a| !matches!(a, Py::Kwargs(_))).map(py_iter).collect::<Result<_, _>>()?;
            let n = iters.iter().map(|v| v.len()).min().unwrap_or(0);
            let mut out = Vec::with_capacity(n);
            for i in 0..n {
                out.push(py_tuple(iters.iter().map(|v| v[i].clone()).collect()));
            }
            Ok(py_list(out))
        }
        "map" => {
            let f = args.first().unwrap_or(&Py::None);
            let items = py_iter(args.get(1).unwrap_or(&Py::None))?;
            Ok(py_list(items.iter().map(|x| py_call(f, std::slice::from_ref(x))).collect()))
        }
        "filter" => {
            let f = args.first().unwrap_or(&Py::None);
            let items = py_iter(args.get(1).unwrap_or(&Py::None))?;
            Ok(py_list(items.into_iter().filter(|x| {
                if f.is_none() { x.is_truthy() } else { py_call(f, std::slice::from_ref(x)).is_truthy() }
            }).collect()))
        }
        "ord" => {
            let s = args.first().and_then(|p| p.as_str()).unwrap_or("");
            Ok(Py::Int(s.chars().next().map(|c| c as i64).unwrap_or(0)))
        }
        "chr" => {
            let i = args.first().and_then(|p| p.as_i64()).unwrap_or(0);
            Ok(py_string(char::from_u32(i as u32).map(|c| c.to_string()).unwrap_or_default()))
        }
        "hex" => Ok(py_str(&format!("{:#x}", args.first().and_then(|p| p.as_i64()).unwrap_or(0)))),
        "oct" => Ok(py_str(&format!("{:#o}", args.first().and_then(|p| p.as_i64()).unwrap_or(0)))),
        "bin" => Ok(py_str(&format!("{:#b}", args.first().and_then(|p| p.as_i64()).unwrap_or(0)))),
        "pow" => py_pow(args.first().unwrap_or(&Py::None), args.get(1).unwrap_or(&Py::None)),
        "divmod" => {
            let a = args.first().unwrap_or(&Py::None);
            let b = args.get(1).unwrap_or(&Py::None);
            Ok(py_tuple(vec![py_floordiv(a, b)?, py_mod(a, b)?]))
        }
        "hash" => Ok(Py::Int(py_hash(args.first().unwrap_or(&Py::None)))),
        "id" => Ok(Py::Int(args.first().map(py_identity).unwrap_or(0))),
        "getattr" => {
            let v = args.first().unwrap_or(&Py::None);
            let name = args.get(1).and_then(|p| p.as_str()).unwrap_or("");
            match py_getattr(v, name) {
                Ok(x) => Ok(x),
                Err(e) => match args.get(2) {
                    Some(d) => Ok(d.clone()),
                    None => Err(e),
                },
            }
        }
        "hasattr" => {
            let v = args.first().unwrap_or(&Py::None);
            let name = args.get(1).and_then(|p| p.as_str()).unwrap_or("");
            Ok(Py::Bool(py_hasattr(v, name)))
        }
        "setattr" => {
            py_setattr(args.first().unwrap_or(&Py::None), args.get(1).and_then(|p| p.as_str()).unwrap_or(""), args.get(2).cloned().unwrap_or(Py::None))?;
            Ok(Py::None)
        }
        "callable" => Ok(Py::Bool(matches!(args.first(), Some(Py::Func(_)) | Some(Py::BoundMethod(_))))),
        "format" => {
            let v = args.first().unwrap_or(&Py::None);
            let spec = args.get(1).and_then(|p| p.as_str()).unwrap_or("");
            Ok(py_string(pystr::format_spec(v, spec)?))
        }
        "print" => {
            let parts: Vec<String> = args.iter().filter(|a| !matches!(a, Py::Kwargs(_))).map(py_str_of).collect();
            let sep = kw(args, "sep").and_then(|p| p.as_str()).unwrap_or(" ").to_string();
            let end = kw(args, "end").and_then(|p| p.as_str()).unwrap_or("\n").to_string();
            print!("{}{}", parts.join(&sep), end);
            Ok(Py::None)
        }
        "next" => {
            let it = args.first().unwrap_or(&Py::None);
            let items = py_iter(it)?;
            match items.into_iter().next() {
                Some(v) => Ok(v),
                None => match args.get(1) { Some(d) => Ok(d.clone()), None => Err(exc::py_err("StopIteration", "")) },
            }
        }
        "iter" => Ok(args.first().cloned().unwrap_or(Py::None)),
        "object" => Ok(Py::Instance(Rc::new(PyInstance { class_name:"object".into(), attrs:Rc::new(std::cell::RefCell::new(indexmap::IndexMap::new())) }))),
        "Exception" | "ValueError" | "TypeError" | "KeyError" | "IndexError" | "AttributeError"
        | "RuntimeError" | "StopIteration" | "ZeroDivisionError" | "ImportError" | "SyntaxError"
        | "NameError" | "LookupError" | "ArithmeticError" | "OverflowError" | "OSError"
        | "FileNotFoundError" | "UnicodeDecodeError" | "NotImplementedError" | "RecursionError" => {
            let msg = args.first().map_or(String::new(), |v| py_str_of(v));
            Ok(Py::Exc(Rc::new(PyExc::with_value(name, &msg, args.first().map_or(String::new(), |v| py_repr(v))))))
        }
        "vars" => {
            if let Some(Py::Instance(i)) = args.first() {
                Ok(Py::Dict(Rc::new(std::cell::RefCell::new(i.attrs.borrow().iter().map(|(k, v)| (PyKey(py_str(k)), v.clone())).collect()))))
            } else { Ok(py_dict(vec![])) }
        }
        "globals" | "locals" => Ok(py_dict(vec![])),
        "open" => Err(exc::py_err("OSError", "pybot-rs: file access is not available")),
        "input" => Ok(py_str("")),
        "super" | "self" => Ok(Py::None),
        "exit" | "quit" => std::process::exit(0),
        _ => Err(name_err(name)),
    }
}

fn round_half_even(x: f64, ndigits: i64) -> f64 {
    if !x.is_finite() { return x; }
    let mult = 10f64.powi(ndigits as i32);
    let v = x * mult;
    let floor = v.floor();
    let diff = v - floor;
    let rounded = if diff > 0.5 { floor + 1.0 } else if diff < 0.5 { floor } else { if (floor as i64) % 2 == 0 { floor } else { floor + 1.0 } };
    rounded / mult
}

fn py_int(args: &[Py]) -> PyResult {
    let v = args.first().unwrap_or(&Py::None);
    match v {
        Py::Int(i) => Ok(Py::Int(*i)),
        Py::Bool(b) => Ok(Py::Int(*b as i64)),
        Py::Float(f) => Ok(Py::Int(f.trunc() as i64)),
        Py::Str(s) => {
            let t = s.trim().replace('_', "");
            match args.get(1).and_then(|p| p.as_i64()) {
                Some(16) => i64::from_str_radix(t.trim_start_matches("0x"), 16).map(Py::Int).map_err(|_| val_err("invalid literal for int()")),
                Some(8) => i64::from_str_radix(t.trim_start_matches("0o"), 8).map(Py::Int).map_err(|_| val_err("invalid literal for int()")),
                Some(2) => i64::from_str_radix(t.trim_start_matches("0b"), 2).map(Py::Int).map_err(|_| val_err("invalid literal for int()")),
                _ => {
                    let t = t.trim_start_matches('+');
                    if let Some(hex) = t.strip_prefix("0x").or(t.strip_prefix("0X")) {
                        return i64::from_str_radix(hex, 16).map(Py::Int).map_err(|_| val_err("invalid literal for int()"));
                    }
                    t.parse::<i64>().map(Py::Int).map_err(|_| val_err(&format!("invalid literal for int() with base 10: {}", py_repr(v))))
                }
            }
        }
        Py::Bytes(b) => {
            let s = String::from_utf8_lossy(b).to_string();
            s.trim().parse::<i64>().map(Py::Int).map_err(|_| val_err("invalid literal for int()"))
        }
        _ => Err(type_err("int() argument must be a string or a number")),
    }
}

fn py_float(args: &[Py]) -> PyResult {
    let v = args.first().unwrap_or(&Py::None);
    match v {
        Py::Float(f) => Ok(Py::Float(*f)),
        Py::Int(i) => Ok(Py::Float(*i as f64)),
        Py::Bool(b) => Ok(Py::Float(*b as i64 as f64)),
        Py::Str(s) => {
            let t = s.trim().to_lowercase();
            match t.as_str() {
                "inf" | "+inf" | "infinity" | "+infinity" => Ok(Py::Float(f64::INFINITY)),
                "-inf" | "-infinity" => Ok(Py::Float(f64::NEG_INFINITY)),
                "nan" => Ok(Py::Float(f64::NAN)),
                _ => t.parse::<f64>().map(Py::Float).map_err(|_| val_err("could not convert string to float")),
            }
        }
        _ => Err(type_err("float() argument must be a string or a number")),
    }
}

/// isinstance with Python type spec: a name string, tuple of specs, or class value.
pub fn py_isinstance(v: &Py, spec: &Py) -> PyResult {
    match spec {
        Py::Tuple(t) => {
            for item in t.iter() {
                if py_isinstance(v, item)?.is_truthy() { return Ok(Py::Bool(true)); }
            }
            Ok(Py::Bool(false))
        }
        Py::Str(name) => Ok(Py::Bool(isinstance_name(v, name))),
        Py::Instance(_) => Ok(Py::Bool(false)),
        // classes passed as Func values carry their name
        Py::Func(f) => Ok(Py::Bool(isinstance_name(v, &f.name))),
        _ => Ok(Py::Bool(false)),
    }
}

pub fn isinstance_name(v: &Py, name: &str) -> bool {
    if let Py::Instance(i) = v { if i.class_name == name { return true; } }
    match name {
        "str" => matches!(v, Py::Str(_)),
        "int" => matches!(v, Py::Int(_) | Py::Bool(_)),
        "bool" => matches!(v, Py::Bool(_)),
        "float" => matches!(v, Py::Float(_)),
        "list" => matches!(v, Py::List(_)),
        "tuple" => matches!(v, Py::Tuple(_)),
        "dict" => matches!(v, Py::Dict(_)),
        "set" => matches!(v, Py::Set(_)),
        "frozenset" => matches!(v, Py::FrozenSet(_) | Py::Set(_)),
        "NoneType" => v.is_none(),
        "Pattern" | "re.Pattern" => matches!(v, Py::Pattern(_)),
        "Match" | "re.Match" => matches!(v, Py::Match(_)),
        "date" => matches!(v, Py::Date(_)),
        "datetime" => matches!(v, Py::DateTime(_)),
        "timedelta" => matches!(v, Py::TimeDelta(_)),
        "range" => matches!(v, Py::Range(_)),
        "bytes" => matches!(v, Py::Bytes(_)),
        "function" | "Callable" => matches!(v, Py::Func(_) | Py::BoundMethod(_)),
        "object" => true,
        "Exception" | "BaseException" => matches!(v, Py::Exc(_)),
        "ValueError" => matches!(v, Py::Exc(e) if e.matches("ValueError")),
        "TypeError" => matches!(v, Py::Exc(e) if e.matches("TypeError")),
        "KeyError" => matches!(v, Py::Exc(e) if e.matches("KeyError")),
        "IndexError" => matches!(v, Py::Exc(e) if e.matches("IndexError")),
        "AttributeError" => matches!(v, Py::Exc(e) if e.matches("AttributeError")),
        "ZeroDivisionError" => matches!(v, Py::Exc(e) if e.matches("ZeroDivisionError")),
        other => matches!(v, Py::Instance(i) if i.class_name == other),
    }
}
