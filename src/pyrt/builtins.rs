//! Python builtins and method dispatch.
use indexmap::{IndexMap, IndexSet};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::pyrt::exc::{self, PyExc};
use crate::pyrt::pyre;
use crate::pyrt::pystr;
use crate::pyrt::pydatetime;
use crate::pyrt::pyrandom;
use crate::pyrt::value::*;

thread_local! {
    static CLASS_METHODS: RefCell<HashMap<String, HashMap<String, Py>>> = RefCell::new(HashMap::new());
}

pub fn register_class_method(class: &str, name: &str, f: Py) {
    CLASS_METHODS.with(|cm| {
        cm.borrow_mut().entry(class.to_string()).or_default().insert(name.to_string(), f);
    });
}

fn class_method(class: &str, name: &str) -> Option<Py> {
    CLASS_METHODS.with(|cm| cm.borrow().get(class).and_then(|m| m.get(name).cloned()))
}

// ---------------------------------------------------------------- call
pub fn py_call(f: &Py, args: &[Py]) -> Py {
    match f {
        Py::Func(func) => func.call(args),
        Py::BoundMethod(bm) => {
            match py_method_call(&bm.recv, &bm.name, args) {
                Ok(v) => v,
                Err(e) => exc::raise(e),
            }
        }
        _ => {
            // Builtin functions referenced by name reach here through py_func wrappers;
            // anything else is not callable.
            match py_method_call(f, "__call__", args) {
                Ok(v) => v,
                Err(_) => exc::raise(type_err(&format!("'{}' object is not callable", f.type_name()))),
            }
        }
    }
}

pub fn py_call_result(f: &Py, args: &[Py]) -> PyResult {
    match f {
        Py::Func(func) => Ok(func.call(args)),
        Py::BoundMethod(bm) => py_method_call(&bm.recv, &bm.name, args),
        _ => Err(type_err(&format!("'{}' object is not callable", f.type_name()))),
    }
}

// ---------------------------------------------------------------- getattr
pub fn py_getattr(v: &Py, name: &str) -> PyResult {
    match v {
        Py::Instance(inst) => {
            if let Some(val) = inst.attrs.borrow().get(name) {
                return Ok(val.clone());
            }
            if let Some(m) = class_method(&inst.class_name, name) {
                return Ok(Py::BoundMethod(Rc::new(PyBoundMethod { recv: Box::new(v.clone()), name: Rc::new(name.to_string()) })).with_func(m));
            }
            Err(attr_err(name))
        }
        Py::Module(m) => match (m.get)(name) {
            Some(val) => Ok(val),
            None => Err(attr_err(name)),
        },
        Py::Dict(d) => {
            // method access on dicts
            if dict_method_names().contains(&name) {
                return Ok(Py::BoundMethod(Rc::new(PyBoundMethod { recv: Box::new(v.clone()), name: Rc::new(name.to_string()) })));
            }
            let _ = d;
            Err(attr_err(name))
        }
        Py::Pattern(p) => match name {
            "pattern" => Ok(py_str(&p.source)),
            "flags" => Ok(Py::Int(p.flags)),
            "groups" => Ok(Py::Int(p.group_count as i64)),
            "groupindex" => Ok(py_dict(p.group_names.iter().enumerate().map(|(i, n)| (py_str(n), Py::Int(i as i64 + 1))).collect())),
            _ => Ok(Py::BoundMethod(Rc::new(PyBoundMethod { recv: Box::new(v.clone()), name: Rc::new(name.to_string()) }))),
        },
        Py::Match(m) => match name {
            "string" => Ok(py_string(m.text.clone())),
            "pos" => Ok(Py::Int(0)),
            "endpos" => Ok(Py::Int(m.text.chars().count() as i64)),
            "lastindex" => {
                let mut li = 0;
                for (i, s) in m.spans.iter().enumerate().skip(1) { if s.is_some() { li = i; } }
                Ok(if li == 0 { Py::None } else { Py::Int(li as i64) })
            }
            "lastgroup" => {
                let mut lg = Py::None;
                for (i, s) in m.spans.iter().enumerate().skip(1) {
                    if s.is_some() { if let Some(n) = m.names.get(i - 1) { lg = py_str(n); } }
                }
                Ok(lg)
            }
            _ => Ok(Py::BoundMethod(Rc::new(PyBoundMethod { recv: Box::new(v.clone()), name: Rc::new(name.to_string()) }))),
        },
        Py::Date(_) | Py::DateTime(_) | Py::TimeDelta(_) => {
            if let Some(val) = pydatetime::date_attr(v, name) { return Ok(val); }
            Ok(Py::BoundMethod(Rc::new(PyBoundMethod { recv: Box::new(v.clone()), name: Rc::new(name.to_string()) })))
        }
        _ => Ok(Py::BoundMethod(Rc::new(PyBoundMethod { recv: Box::new(v.clone()), name: Rc::new(name.to_string()) }))),
    }
}

impl Py {
    fn with_func(self, _f: Py) -> Py { self }
}

fn dict_method_names() -> &'static [&'static str] {
    &["get", "items", "keys", "values", "pop", "popitem", "setdefault", "update", "clear", "copy", "fromkeys", "set"]
}

pub fn py_setattr(v: &Py, name: &str, val: Py) -> Result<(), PyExc> {
    match v {
        Py::Instance(inst) => {
            inst.attrs.borrow_mut().insert(name.to_string(), val);
            Ok(())
        }
        Py::Module(m) => {
            if let Some(set) = &m.set { set(name, val); Ok(()) } else { Err(attr_err(name)) }
        }
        _ => Err(attr_err(name)),
    }
}

pub fn py_hasattr(v: &Py, name: &str) -> bool { py_getattr(v, name).is_ok() }

// ---------------------------------------------------------------- method calls
pub fn py_method_call(recv: &Py, name: &str, args: &[Py]) -> PyResult {
    match recv {
        Py::Str(s) => pystr::str_method(s, name, args),
        Py::List(l) => list_method(l, name, args),
        Py::Tuple(t) => tuple_method(t, name, args),
        Py::Dict(d) => dict_method(d, name, args),
        Py::Set(s) => set_method(&Py::Set(s.clone()), name, args),
        Py::FrozenSet(s) => set_method(&Py::FrozenSet(s.clone()), name, args),
        Py::Pattern(p) => pattern_method(p, name, args),
        Py::Match(m) => match_method(m, name, args),
        Py::Instance(inst) => {
            if let Some(m) = class_method(&inst.class_name, name) {
                let mut full = vec![recv.clone()];
                full.extend_from_slice(args);
                return py_call_result(&m, &full);
            }
            Err(attr_err(name))
        }
        Py::Module(m) => {
            if let Some(f) = (m.get)(name) {
                return py_call_result(&f, args);
            }
            Err(attr_err(name))
        }
        Py::Date(_) | Py::DateTime(_) | Py::TimeDelta(_) => {
            match pydatetime::date_method(recv, name, args) {
                Some(v) => Ok(v),
                None => Err(attr_err(name)),
            }
        }
        Py::Int(i) => int_method(*i, name, args),
        Py::Float(f) => float_method(*f, name, args),
        Py::Bool(b) => int_method(*b as i64, name, args),
        Py::Bytes(b) => bytes_method(b, name, args),
        _ => Err(type_err(&format!("'{}' object has no attribute '{}'", recv.type_name(), name))),
    }
}

fn int_method(i: i64, name: &str, _args: &[Py]) -> PyResult {
    match name {
        "bit_length" => Ok(Py::Int(if i == 0 { 0 } else { 64 - i.unsigned_abs().leading_zeros() as i64 })),
        "to_bytes" => {
            let len = _args.first().and_then(|p| p.as_i64()).unwrap_or(1).max(0) as usize;
            let mut b = i.to_be_bytes().to_vec();
            while b.len() < len { b.insert(0, 0); }
            let off = b.len().saturating_sub(len);
            Ok(Py::Bytes(Rc::new(b.split_off(off))))
        }
        "as_integer_ratio" => Ok(py_tuple(vec![Py::Int(i), Py::Int(1)])),
        "real" | "numerator" => Ok(Py::Int(i)),
        "imag" | "denominator" => Ok(Py::Int(if name == "denominator" { 1 } else { 0 })),
        _ => Err(attr_err(name)),
    }
}

fn float_method(f: f64, name: &str, _args: &[Py]) -> PyResult {
    match name {
        "is_integer" => Ok(Py::Bool(f.fract() == 0.0)),
        "hex" => Ok(py_str(&format!("{}", f))),
        "as_integer_ratio" => {
            if f.fract() == 0.0 && f.is_finite() { Ok(py_tuple(vec![Py::Int(f as i64), Py::Int(1)])) } else { Ok(py_tuple(vec![Py::Int((f * 1000000.0) as i64), Py::Int(1000000)])) }
        }
        "real" => Ok(Py::Float(f)),
        _ => Err(attr_err(name)),
    }
}

fn bytes_method(b: &Rc<Vec<u8>>, name: &str, args: &[Py]) -> PyResult {
    match name {
        "decode" => Ok(py_string(String::from_utf8_lossy(b).to_string())),
        "hex" => Ok(py_string(b.iter().map(|x| format!("{:02x}", x)).collect())),
        "find" => {
            let sub = match args.first() { Some(Py::Bytes(s)) => (**s).clone(), _ => vec![] };
            if sub.is_empty() { return Ok(Py::Int(0)); }
            for i in 0..=b.len().saturating_sub(sub.len()) {
                if b[i..i + sub.len()] == sub[..] { return Ok(Py::Int(i as i64)); }
            }
            Ok(Py::Int(-1))
        }
        "startswith" => {
            if let Some(Py::Bytes(s)) = args.first() { return Ok(Py::Bool(b.starts_with(s))); }
            Ok(Py::Bool(false))
        }
        "split" => {
            let sep = match args.first() { Some(Py::Bytes(s)) => (**s).clone(), _ => vec![b' '] };
            let mut out = Vec::new();
            let mut cur = Vec::new();
            let mut i = 0;
            while i < b.len() {
                if i + sep.len() <= b.len() && b[i..i + sep.len()] == sep[..] {
                    out.push(Py::Bytes(Rc::new(std::mem::take(&mut cur))));
                    i += sep.len();
                } else { cur.push(b[i]); i += 1; }
            }
            out.push(Py::Bytes(Rc::new(cur)));
            Ok(py_list(out))
        }
        _ => Err(attr_err(name)),
    }
}

fn list_method(l: &Rc<RefCell<Vec<Py>>>, name: &str, args: &[Py]) -> PyResult {
    match name {
        "append" => { l.borrow_mut().push(args.first().cloned().unwrap_or(Py::None)); Ok(Py::None) }
        "extend" => {
            let items = py_iter(args.first().unwrap_or(&Py::None))?;
            l.borrow_mut().extend(items);
            Ok(Py::None)
        }
        "insert" => {
            let i = args.first().and_then(|p| p.as_i64()).unwrap_or(0);
            let val = args.get(1).cloned().unwrap_or(Py::None);
            let mut b = l.borrow_mut();
            let len = b.len() as i64;
            let idx = if i < 0 { (len + i).max(0) } else { i.min(len) } as usize;
            b.insert(idx, val);
            Ok(Py::None)
        }
        "pop" => {
            let mut b = l.borrow_mut();
            let len = b.len() as i64;
            if len == 0 { return Err(index_err_msg("pop from empty list")); }
            let i = args.first().and_then(|p| p.as_i64()).unwrap_or(-1);
            let idx = if i < 0 { len + i } else { i };
            if idx < 0 || idx >= len { return Err(index_err_msg("pop index out of range")); }
            Ok(b.remove(idx as usize))
        }
        "remove" => {
            let target = args.first().unwrap_or(&Py::None);
            let mut b = l.borrow_mut();
            if let Some(pos) = b.iter().position(|x| py_eq(x, target)) {
                b.remove(pos);
                Ok(Py::None)
            } else { Err(val_err("list.remove(x): x not in list")) }
        }
        "index" => {
            let target = args.first().unwrap_or(&Py::None);
            let b = l.borrow();
            match b.iter().position(|x| py_eq(x, target)) {
                Some(pos) => Ok(Py::Int(pos as i64)),
                None => Err(val_err("is not in list")),
            }
        }
        "count" => {
            let target = args.first().unwrap_or(&Py::None);
            Ok(Py::Int(l.borrow().iter().filter(|x| py_eq(x, target)).count() as i64))
        }
        "sort" => {
            let (key, reverse) = sort_args(args);
            let mut items: Vec<(Py, Py)> = l.borrow().iter().map(|x| {
                let k = match &key { Some(f) => py_call(f, std::slice::from_ref(x)), None => x.clone() };
                (k, x.clone())
            }).collect();
            items.sort_by(|a, b| py_cmp_total(&a.0, &b.0));
            if reverse { items.reverse(); }
            *l.borrow_mut() = items.into_iter().map(|(_, x)| x).collect();
            Ok(Py::None)
        }
        "reverse" => { l.borrow_mut().reverse(); Ok(Py::None) }
        "clear" => { l.borrow_mut().clear(); Ok(Py::None) }
        "copy" => Ok(py_list(l.borrow().clone())),
        _ => Err(attr_err(name)),
    }
}

fn sort_args(args: &[Py]) -> (Option<Py>, bool) {
    let mut key = None;
    let mut reverse = false;
    for a in args {
        if let Py::Kwargs(kws) = a {
            for (n, v) in kws.iter() {
                match n.as_str() {
                    "key" => key = if v.is_none() { None } else { Some(v.clone()) },
                    "reverse" => reverse = v.is_truthy(),
                    _ => {}
                }
            }
        }
    }
    (key, reverse)
}

fn index_err_msg(m: &str) -> PyExc { exc::py_err("IndexError", m) }

fn tuple_method(t: &Rc<Vec<Py>>, name: &str, args: &[Py]) -> PyResult {
    match name {
        "index" => {
            let target = args.first().unwrap_or(&Py::None);
            match t.iter().position(|x| py_eq(x, target)) {
                Some(pos) => Ok(Py::Int(pos as i64)),
                None => Err(val_err("tuple.index(x): x not in tuple")),
            }
        }
        "count" => Ok(Py::Int(t.iter().filter(|x| py_eq(x, args.first().unwrap_or(&Py::None))).count() as i64)),
        _ => Err(attr_err(name)),
    }
}

fn dict_method(d: &Rc<RefCell<IndexMap<PyKey, Py>>>, name: &str, args: &[Py]) -> PyResult {
    match name {
        "get" => {
            let key = args.first().unwrap_or(&Py::None);
            let default = args.get(1).cloned().unwrap_or(Py::None);
            Ok(d.borrow().get(&PyKey(key.clone())).cloned().unwrap_or(default))
        }
        "items" => Ok(py_list(d.borrow().iter().map(|(k, v)| py_tuple(vec![k.0.clone(), v.clone()])).collect())),
        "keys" => Ok(py_list(d.borrow().keys().map(|k| k.0.clone()).collect())),
        "values" => Ok(py_list(d.borrow().values().cloned().collect())),
        "pop" => {
            let key = args.first().unwrap_or(&Py::None);
            let mut b = d.borrow_mut();
            match b.shift_remove(&PyKey(key.clone())) {
                Some(v) => Ok(v),
                None => match args.get(1) {
                    Some(def) => Ok(def.clone()),
                    None => Err(key_err(key)),
                },
            }
        }
        "popitem" => {
            let mut b = d.borrow_mut();
            match b.pop() {
                Some((k, v)) => Ok(py_tuple(vec![k.0, v])),
                None => Err(exc::py_err("KeyError", "popitem(): dictionary is empty")),
            }
        }
        "setdefault" => {
            let key = args.first().unwrap_or(&Py::None);
            let default = args.get(1).cloned().unwrap_or(Py::None);
            let mut b = d.borrow_mut();
            Ok(b.entry(PyKey(key.clone())).or_insert(default).clone())
        }
        "update" => {
            let mut b = d.borrow_mut();
            if let Some(src) = args.first() {
                match src {
                    Py::Dict(other) => { for (k, v) in other.borrow().iter() { b.insert(k.clone(), v.clone()); } }
                    Py::Kwargs(kws) => { for (n, v) in kws.iter() { b.insert(PyKey(py_str(n)), v.clone()); } }
                    other => {
                        for pair in py_iter(other)? {
                            let kv = py_iter(&pair)?;
                            if kv.len() == 2 { b.insert(PyKey(kv[0].clone()), kv[1].clone()); }
                        }
                    }
                }
            }
            for a in args.iter().skip(1) {
                if let Py::Kwargs(kws) = a {
                    for (n, v) in kws.iter() { b.insert(PyKey(py_str(n)), v.clone()); }
                }
            }
            Ok(Py::None)
        }
        "clear" => { d.borrow_mut().clear(); Ok(Py::None) }
        "copy" => Ok(Py::Dict(Rc::new(RefCell::new(d.borrow().clone())))),
        "fromkeys" => Ok(Py::None), // fromkeys is a classmethod; handled via dict() call
        _ => Err(attr_err(name)),
    }
}

fn set_method(s: &Py, name: &str, args: &[Py]) -> PyResult {
    let get_set = |v: &Py| -> IndexSet<PyKey> {
        match v {
            Py::Set(x) => x.borrow().clone(),
            Py::FrozenSet(x) => (**x).clone(),
            other => py_iter(other).unwrap_or_default().into_iter().map(PyKey).collect(),
        }
    };
    let mutable = if let Py::Set(x) = s { Some(x.clone()) } else { None };
    let cur = get_set(s);
    match name {
        "add" => {
            if let Some(m) = mutable { m.borrow_mut().insert(PyKey(args.first().cloned().unwrap_or(Py::None))); }
            Ok(Py::None)
        }
        "discard" => {
            if let Some(m) = mutable { m.borrow_mut().shift_remove(&PyKey(args.first().cloned().unwrap_or(Py::None))); }
            Ok(Py::None)
        }
        "remove" => {
            if let Some(m) = mutable {
                if !m.borrow_mut().shift_remove(&PyKey(args.first().cloned().unwrap_or(Py::None))) {
                    return Err(key_err(args.first().unwrap_or(&Py::None)));
                }
            }
            Ok(Py::None)
        }
        "pop" => {
            if let Some(m) = mutable {
                let v = m.borrow_mut().pop().map(|k| k.0);
                return v.ok_or_else(|| key_err(&py_str("pop from an empty set")));
            }
            Err(type_err("frozenset has no pop"))
        }
        "clear" => { if let Some(m) = mutable { m.borrow_mut().clear(); } Ok(Py::None) }
        "update" => {
            if let Some(m) = mutable {
                let mut b = m.borrow_mut();
                for a in args { for k in get_set(a) { b.insert(k); } }
            }
            Ok(Py::None)
        }
        "union" => {
            let mut out = cur.clone();
            for a in args { for k in get_set(a) { out.insert(k); } }
            Ok(Py::Set(Rc::new(RefCell::new(out))))
        }
        "intersection" => {
            let mut out = cur.clone();
            for a in args {
                let other = get_set(a);
                out.retain(|k| other.contains(k));
            }
            Ok(Py::Set(Rc::new(RefCell::new(out))))
        }
        "difference" => {
            let mut out = cur.clone();
            for a in args {
                let other = get_set(a);
                out.retain(|k| !other.contains(k));
            }
            Ok(Py::Set(Rc::new(RefCell::new(out))))
        }
        "symmetric_difference" => {
            let other = get_set(args.first().unwrap_or(&Py::None));
            let mut out = IndexSet::new();
            for k in cur.iter() { if !other.contains(k) { out.insert(k.clone()); } }
            for k in other.iter() { if !cur.contains(k) { out.insert(k.clone()); } }
            Ok(Py::Set(Rc::new(RefCell::new(out))))
        }
        "issubset" => {
            let other = get_set(args.first().unwrap_or(&Py::None));
            Ok(Py::Bool(cur.iter().all(|k| other.contains(k))))
        }
        "issuperset" => {
            let other = get_set(args.first().unwrap_or(&Py::None));
            Ok(Py::Bool(other.iter().all(|k| cur.contains(k))))
        }
        "isdisjoint" => {
            let other = get_set(args.first().unwrap_or(&Py::None));
            Ok(Py::Bool(!cur.iter().any(|k| other.contains(k))))
        }
        "copy" => Ok(Py::Set(Rc::new(RefCell::new(cur)))),
        _ => Err(attr_err(name)),
    }
}

fn pattern_method(p: &Rc<pyre::PyPattern>, name: &str, args: &[Py]) -> PyResult {
    let text = args.first().and_then(|v| v.as_str()).unwrap_or("").to_string();
    let pos = args.get(1).and_then(|v| v.as_i64()).unwrap_or(0).max(0) as usize;
    match name {
        "search" => Ok(pyre::search(p, &text, pos)),
        "match" => Ok(pyre::match_at(p, &text, pos, true, false)),
        "fullmatch" => Ok(pyre::fullmatch(p, &text, pos)),
        "findall" => Ok(pyre::findall(p, &text)),
        "finditer" => Ok(pyre::finditer(p, &text)),
        "split" => {
            let maxsplit = args.get(1).and_then(|v| v.as_i64()).unwrap_or(0);
            Ok(pyre::split(p, &text, maxsplit))
        }
        "sub" => {
            let repl = args.get(1).cloned().unwrap_or(Py::None);
            let count = args.get(2).and_then(|v| v.as_i64()).unwrap_or(0);
            Ok(pyre::sub(p, &repl, &text, count))
        }
        "subn" => {
            let repl = args.get(1).cloned().unwrap_or(Py::None);
            let count = args.get(2).and_then(|v| v.as_i64()).unwrap_or(0);
            Ok(pyre::subn(p, &repl, &text, count))
        }
        _ => Err(attr_err(name)),
    }
}

fn match_method(m: &Rc<pyre::PyMatch>, name: &str, args: &[Py]) -> PyResult {
    match name {
        "group" => {
            if args.is_empty() { return Ok(m.group(0)); }
            if args.len() == 1 {
                return match &args[0] {
                    Py::Str(s) => Ok(m.group_by_name(s)),
                    other => Ok(m.group(other.as_i64().unwrap_or(0))),
                };
            }
            let mut out = Vec::new();
            for a in args {
                out.push(match a {
                    Py::Str(s) => m.group_by_name(s),
                    other => m.group(other.as_i64().unwrap_or(0)),
                });
            }
            Ok(py_tuple(out))
        }
        "groups" => Ok(m.groups(args.first().unwrap_or(&Py::None))),
        "groupdict" => Ok(m.groupdict(args.first().unwrap_or(&Py::None))),
        "start" => Ok(m.start(args.first().and_then(|p| p.as_i64()).unwrap_or(0))),
        "end" => Ok(m.end(args.first().and_then(|p| p.as_i64()).unwrap_or(0))),
        "span" => Ok(m.span(args.first().and_then(|p| p.as_i64()).unwrap_or(0))),
        "expand" => Ok(m.expand(args.first().and_then(|p| p.as_str()).unwrap_or(""))),
        "__getitem__" => Ok(m.group(args.first().and_then(|p| p.as_i64()).unwrap_or(0))),
        _ => Err(attr_err(name)),
    }
}

// ---------------------------------------------------------------- deepcopy
pub fn py_deepcopy(v: &Py) -> Py {
    match v {
        Py::List(l) => py_list(l.borrow().iter().map(py_deepcopy).collect()),
        Py::Tuple(t) => py_tuple(t.iter().map(py_deepcopy).collect()),
        Py::Dict(d) => Py::Dict(Rc::new(RefCell::new(
            d.borrow().iter().map(|(k, v)| (PyKey(py_deepcopy(&k.0)), py_deepcopy(v))).collect()
        ))),
        Py::Set(s) => Py::Set(Rc::new(RefCell::new(s.borrow().iter().map(|k| PyKey(py_deepcopy(&k.0))).collect()))),
        Py::FrozenSet(s) => Py::FrozenSet(Rc::new(s.iter().map(|k| PyKey(py_deepcopy(&k.0))).collect())),
        Py::Instance(i) => Py::Instance(Rc::new(PyInstance {
            class_name: i.class_name.clone(),
            attrs: Rc::new(RefCell::new(i.attrs.borrow().iter().map(|(k, v)| (k.clone(), py_deepcopy(v))).collect())),
        })),
        other => other.clone(),
    }
}
