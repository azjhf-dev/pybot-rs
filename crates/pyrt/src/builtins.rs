//! Python builtins and method dispatch.
use indexmap::{IndexMap, IndexSet};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::exc::{self, PyExc};
use crate::pyre;
use crate::pystr;
use crate::pydatetime;
use crate::pyrandom;
use crate::value::*;

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
                if m.borrow_mut().shift_remove(&PyKey(args.first().cloned().unwrap_or(Py::None))).is_none() {
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

// ---------------------------------------------------------------- transpiler helpers
pub fn gattr(v: &Py, name: &str) -> Py {
    py_getattr(v, name).unwrap_or_else(|e| exc::raise(e))
}
pub fn mcall(recv: &Py, name: &str, args: &[Py]) -> Py {
    py_method_call(recv, name, args).unwrap_or_else(|e| exc::raise(e))
}
pub fn kw_pairs(v: &Py) -> Vec<(String, Py)> {
    match v {
        Py::Dict(d) => d.borrow().iter().map(|(k, val)| (py_str_of(k), val.clone())).collect(),
        _ => Vec::new(),
    }
}
pub fn __kwp(args: &[Py], i: usize, name: &str) -> Option<Py> {
    let mut pos: Vec<&Py> = Vec::new();
    for a in args {
        match a {
            Py::Kwargs(kws) => {
                for (n, v) in kws.iter() {
                    if n == name { return Some(v.clone()); }
                }
            }
            other => pos.push(other),
        }
    }
    if pos.len() > i { Some(pos[i].clone()) } else { None }
}
pub fn py_iter_or_raise(v: &Py) -> Vec<Py> {
    py_iter(v).unwrap_or_else(|e| exc::raise(e))
}
pub fn py_unpack(v: &Py, n: usize, star: Option<usize>) -> Vec<Py> {
    let items = py_iter_or_raise(v);
    match star {
        None => {
            if items.len() != n {
                exc::raise(value_err(&format!("too many or too few values to unpack (expected {})", n)));
            }
            items
        }
        Some(s) => {
            if items.len() < n - 1 {
                exc::raise(value_err("not enough values to unpack"));
            }
            let mut out: Vec<Py> = Vec::new();
            for i in 0..s { out.push(items[i].clone()); }
            let tail = n - 1 - s;
            out.push(py_list(items[s..items.len() - tail].to_vec()));
            for i in (items.len() - tail)..items.len() { out.push(items[i].clone()); }
            out
        }
    }
}
pub fn dict_spread(vals: &[Py]) -> Py {
    let mut out: Vec<(Py, Py)> = Vec::new();
    for v in vals {
        match v {
            Py::Dict(d) => out.extend(d.borrow().iter().map(|(k, val)| (k.clone(), val.clone()))),
            _ => { exc::raise(type_err("argument is not a mapping")); }
        }
    }
    py_dict(out)
}
pub fn py_is(a: &Py, b: &Py) -> bool {
    match (a, b) {
        (Py::None, Py::None) => true,
        (Py::Bool(x), Py::Bool(y)) => x == y,
        (Py::Str(x), Py::Str(y)) => Rc::ptr_eq(x, y),
        (Py::List(x), Py::List(y)) => Rc::ptr_eq(x, y),
        (Py::Dict(x), Py::Dict(y)) => Rc::ptr_eq(x, y),
        (Py::Tuple(x), Py::Tuple(y)) => Rc::ptr_eq(x, y),
        (Py::Set(x), Py::Set(y)) => Rc::ptr_eq(x, y),
        (Py::Func(x), Py::Func(y)) => Rc::ptr_eq(x, y),
        (Py::Instance(x), Py::Instance(y)) => Rc::ptr_eq(x, y),
        (Py::Module(x), Py::Module(y)) => Rc::ptr_eq(x, y),
        (Py::Int(x), Py::Int(y)) => x == y && (*x >= -5 && *x <= 256),
        _ => false,
    }
}
pub fn geti(v: &Py, key: &Py) -> Py {
    py_getitem(v, key).unwrap_or_else(|e| exc::raise(e))
}
pub fn slice_or_raise(v: &Py, start: &Py, stop: &Py, step: &Py) -> Py {
    py_slice(v, start, stop, step).unwrap_or_else(|e| exc::raise(e))
}
pub fn fstr(v: &Py, conv: i32, spec: &str) -> String {
    let s = match conv {
        114 => py_repr(v),               // 'r'
        115 => py_str_of(v),             // 's'
        97 => py_repr(v),                // 'a'
        _ => py_str_of(v),
    };
    if spec.is_empty() { return s; }
    format_with_spec(v, &s, spec)
}
fn format_with_spec(v: &Py, s: &str, spec: &str) -> String {
    // minimal format spec: [[fill]align][sign][#][0][width][,][.prec][type]
    let b = spec.as_bytes();
    let mut i = 0;
    let mut fill = ' ';
    let mut align = '\0';
    if b.len() >= 2 && (b[1] == b'<' || b[1] == b'>' || b[1] == b'^' || b[1] == b'=') {
        fill = b[0] as char; align = b[1] as char; i = 2;
    } else if b.len() >= 1 && (b[0] == b'<' || b[0] == b'>' || b[0] == b'^' || b[0] == b'=') {
        align = b[0] as char; i = 1;
    }
    let mut sign = '-';
    if i < b.len() && (b[i] == b'+' || b[i] == b'-' || b[i] == b' ') { sign = b[i] as char; i += 1; }
    if i < b.len() && b[i] == b'#' { i += 1; }
    let mut zero = false;
    if i < b.len() && b[i] == b'0' { zero = true; i += 1; }
    let mut width = String::new();
    while i < b.len() && b[i].is_ascii_digit() { width.push(b[i] as char); i += 1; }
    if i < b.len() && b[i] == b',' { i += 1; }
    let mut prec = String::new();
    let mut has_prec = false;
    if i < b.len() && b[i] == b'.' { has_prec = true; i += 1; while i < b.len() && b[i].is_ascii_digit() { prec.push(b[i] as char); i += 1; } }
    let ty = if i < b.len() { b[i] as char } else { '\0' };
    let width_n: usize = width.parse().unwrap_or(0);
    let mut body = s.to_string();
    match ty {
        'f' | 'F' => {
            let p: usize = if has_prec { prec.parse().unwrap_or(6) } else { 6 };
            if let Some(f) = num_f64(v) { body = format!("{:.*}", p, f); }
        }
        'd' => { if let Some(n) = num_i64(v) { body = n.to_string(); } }
        'x' => { if let Some(n) = num_i64(v) { body = format!("{:x}", n); } }
        'X' => { if let Some(n) = num_i64(v) { body = format!("{:X}", n); } }
        'o' => { if let Some(n) = num_i64(v) { body = format!("{:o}", n); } }
        'b' => { if let Some(n) = num_i64(v) { body = format!("{:b}", n); } }
        'e' | 'E' => {
            let p: usize = if has_prec { prec.parse().unwrap_or(6) } else { 6 };
            if let Some(f) = num_f64(v) { body = format!("{:.*e}", p, f); }
        }
        '%' => {
            let p: usize = if has_prec { prec.parse().unwrap_or(6) } else { 6 };
            if let Some(f) = num_f64(v) { body = format!("{:.*}%", p, f * 100.0); }
        }
        's' => {}
        '\0' => {
            if has_prec {
                if let Some(f) = num_f64(v) {
                    let p: usize = prec.parse().unwrap_or(6);
                    body = format_g(f, p);
                } else {
                    body = s.chars().take(prec.parse().unwrap_or(usize::MAX)).collect();
                }
            }
        }
        _ => {}
    }
    if sign == '+' {
        if matches!(v, Py::Int(_) | Py::Float(_)) && !body.starts_with('-') && !body.starts_with('+') { body = format!("+{}", body); }
    } else if sign == ' ' {
        if matches!(v, Py::Int(_) | Py::Float(_)) && !body.starts_with('-') { body = format!(" {}", body); }
    }
    if body.chars().count() >= width_n { return body; }
    let pad = width_n - body.chars().count();
    if zero && align == '\0' && matches!(v, Py::Int(_) | Py::Float(_)) {
        let (signpart, rest) = if body.starts_with('-') || body.starts_with('+') || body.starts_with(' ') {
            body.split_at(1)
        } else { ("", body.as_str()) };
        return format!("{}{}{}", signpart, "0".repeat(pad), rest);
    }
    match align {
        '<' => format!("{}{}", body, fill.to_string().repeat(pad)),
        '^' => { let l = pad / 2; let r = pad - l; format!("{}{}{}", fill.to_string().repeat(l), body, fill.to_string().repeat(r)) }
        _ => format!("{}{}", fill.to_string().repeat(pad), body),
    }
}
fn num_f64(v: &Py) -> Option<f64> {
    match v { Py::Int(i) => Some(*i as f64), Py::Float(f) => Some(*f), Py::Bool(b) => Some(*b as u8 as f64), _ => None }
}
fn num_i64(v: &Py) -> Option<i64> {
    match v { Py::Int(i) => Some(*i), Py::Bool(b) => Some(*b as i64), _ => None }
}
fn format_g(f: f64, p: usize) -> String {
    let p = if p == 0 { 1 } else { p };
    let exp = f.abs().log10().floor() as i32;
    if exp < -4 || exp >= p as i32 {
        format!("{:.*e}", p - 1, f)
    } else {
        let dec = (p as i32 - 1 - exp).max(0) as usize;
        let s = format!("{:.*}", dec, f);
        let s = s.trim_end_matches('0').trim_end_matches('.');
        s.to_string()
    }
}
// raising op wrappers
pub fn op_add(a: &Py, b: &Py) -> Py { py_add(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_sub(a: &Py, b: &Py) -> Py { py_sub(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_mul(a: &Py, b: &Py) -> Py { py_mul(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_truediv(a: &Py, b: &Py) -> Py { py_truediv(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_floordiv(a: &Py, b: &Py) -> Py { py_floordiv(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_modulo(a: &Py, b: &Py) -> Py { py_mod(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_pow(a: &Py, b: &Py) -> Py { py_pow(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_neg(a: &Py) -> Py { py_neg(a).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_invert(a: &Py) -> Py { py_invert(a).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_bitand(a: &Py, b: &Py) -> Py { py_bitand(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_bitor(a: &Py, b: &Py) -> Py { py_bitor(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_bitxor(a: &Py, b: &Py) -> Py { py_bitxor(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_lshift(a: &Py, b: &Py) -> Py { py_lshift(a, b).unwrap_or_else(|e| exc::raise(e)) }
pub fn op_rshift(a: &Py, b: &Py) -> Py { py_rshift(a, b).unwrap_or_else(|e| exc::raise(e)) }
fn cmp_bool(a: &Py, b: &Py, op: &str) -> Py {
    match py_cmp(a, b, op) {
        Ok(std::cmp::Ordering::Less) => Py::Bool(matches!(op, "<" | "<=" | "!=")),
        Ok(std::cmp::Ordering::Equal) => Py::Bool(matches!(op, "<=" | ">=" | "==")),
        Ok(std::cmp::Ordering::Greater) => Py::Bool(matches!(op, ">" | ">=" | "!=")),
        Err(e) => exc::raise(e),
    }
}
pub fn op_lt(a: &Py, b: &Py) -> Py { cmp_bool(a, b, "<") }
pub fn op_le(a: &Py, b: &Py) -> Py { cmp_bool(a, b, "<=") }
pub fn op_gt(a: &Py, b: &Py) -> Py { cmp_bool(a, b, ">") }
pub fn op_ge(a: &Py, b: &Py) -> Py { cmp_bool(a, b, ">") }
