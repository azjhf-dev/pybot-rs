//! Python-semantics dynamic value for transpiled PyBot code.
use indexmap::{IndexMap, IndexSet};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::exc::{PyExc, py_err};
use crate::pyre::{PyMatch, PyPattern};
use crate::pydatetime::{PyDate, PyDateTime, PyTimeDelta};

pub type PyResult = Result<Py, PyExc>;

#[derive(Clone)]
pub struct PyFunc {
    pub name: String,
    pub f: Rc<dyn Fn(&[Py]) -> Py>,
}
impl PyFunc {
    pub fn call(&self, args: &[Py]) -> Py { (self.f)(args) }
}

#[derive(Clone)]
pub struct PyBoundMethod {
    pub recv: Box<Py>,
    pub name: Rc<String>,
}

#[derive(Clone)]
pub struct PyInstance {
    pub class_name: String,
    pub attrs: Rc<RefCell<IndexMap<String, Py>>>,
}

#[derive(Clone)]
pub struct PyModule {
    pub name: String,
    pub get: Rc<dyn Fn(&str) -> Option<Py>>,
    pub set: Option<Rc<dyn Fn(&str, Py)>>,
}

#[derive(Clone)]
pub struct PyRange {
    pub start: i64,
    pub stop: i64,
    pub step: i64,
}
impl PyRange {
    pub fn values(&self) -> Vec<Py> {
        let mut out = Vec::new();
        let mut i = self.start;
        if self.step > 0 {
            while i < self.stop { out.push(Py::Int(i)); i += self.step; }
        } else if self.step < 0 {
            while i > self.stop { out.push(Py::Int(i)); i += self.step; }
        }
        out
    }
    pub fn len(&self) -> i64 {
        if self.step > 0 {
            if self.start >= self.stop { 0 } else { (self.stop - self.start + self.step - 1) / self.step }
        } else if self.step < 0 {
            if self.stop >= self.start { 0 } else { (self.start - self.stop + (-self.step) - 1) / (-self.step) }
        } else { 0 }
    }
}

#[derive(Clone)]
pub enum Py {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(Rc<String>),
    List(Rc<RefCell<Vec<Py>>>),
    Tuple(Rc<Vec<Py>>),
    Dict(Rc<RefCell<IndexMap<PyKey, Py>>>),
    Set(Rc<RefCell<IndexSet<PyKey>>>),
    FrozenSet(Rc<IndexSet<PyKey>>),
    Range(Rc<PyRange>),
    Pattern(Rc<PyPattern>),
    Match(Rc<PyMatch>),
    Func(Rc<PyFunc>),
    BoundMethod(Rc<PyBoundMethod>),
    Instance(Rc<PyInstance>),
    Module(Rc<PyModule>),
    Date(Rc<PyDate>),
    DateTime(Rc<PyDateTime>),
    TimeDelta(Rc<PyTimeDelta>),
    Kwargs(Rc<Vec<(String, Py)>>),
    Exc(Rc<PyExc>),
    Bytes(Rc<Vec<u8>>),
    Ellipsis,
    NotImplementedType,
}

// ---------------------------------------------------------------- errors
pub fn type_err(msg: &str) -> PyExc { py_err("TypeError", msg) }
pub fn val_err(msg: &str) -> PyExc { py_err("ValueError", msg) }
pub fn key_err(key: &Py) -> PyExc { PyExc::new("KeyError", &py_repr(key)) }
pub fn value_err(msg: &str) -> PyExc { py_err("ValueError", msg) }
pub fn index_err() -> PyExc { py_err("IndexError", "list index out of range") }
pub fn attr_err(name: &str) -> PyExc { PyExc::new("AttributeError", &format!("object has no attribute '{}'", name)) }
pub fn name_err(name: &str) -> PyExc { PyExc::new("NameError", &format!("name '{}' is not defined", name)) }
pub fn zero_div() -> PyExc { py_err("ZeroDivisionError", "division by zero") }

// ---------------------------------------------------------------- helpers
pub fn py_str(s: &str) -> Py { Py::Str(Rc::new(s.to_string())) }
pub fn py_string(s: String) -> Py { Py::Str(Rc::new(s)) }
pub fn py_list(v: Vec<Py>) -> Py { Py::List(Rc::new(RefCell::new(v))) }
pub fn py_tuple(v: Vec<Py>) -> Py { Py::Tuple(Rc::new(v)) }
pub fn py_dict(v: Vec<(Py, Py)>) -> Py {
    let mut m = IndexMap::new();
    for (k, val) in v { m.insert(PyKey(k), val); }
    Py::Dict(Rc::new(RefCell::new(m)))
}
pub fn py_set(v: Vec<Py>) -> Py {
    let mut s = IndexSet::new();
    for x in v { s.insert(PyKey(x)); }
    Py::Set(Rc::new(RefCell::new(s)))
}
pub fn py_frozenset(v: Vec<Py>) -> Py {
    let mut s = IndexSet::new();
    for x in v { s.insert(PyKey(x)); }
    Py::FrozenSet(Rc::new(s))
}
pub fn py_func<F: Fn(&[Py]) -> Py + 'static>(name: &str, f: F) -> Py {
    Py::Func(Rc::new(PyFunc { name: name.to_string(), f: Rc::new(f) }))
}

impl Py {
    pub fn as_str(&self) -> Option<&str> {
        if let Py::Str(s) = self { Some(s.as_str()) } else { None }
    }
    pub fn to_string_py(&self) -> String { py_str_of(self) }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Py::Int(i) => Some(*i),
            Py::Bool(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Py::Int(i) => Some(*i as f64),
            Py::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
            Py::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn is_none(&self) -> bool { matches!(self, Py::None) }
    pub fn is_truthy(&self) -> bool { py_truthy(self) }
    pub fn type_name(&self) -> &'static str {
        match self {
            Py::None => "NoneType",
            Py::Bool(_) => "bool",
            Py::Int(_) => "int",
            Py::Float(_) => "float",
            Py::Str(_) => "str",
            Py::List(_) => "list",
            Py::Tuple(_) => "tuple",
            Py::Dict(_) => "dict",
            Py::Set(_) => "set",
            Py::FrozenSet(_) => "frozenset",
            Py::Range(_) => "range",
            Py::Pattern(_) => "re.Pattern",
            Py::Match(_) => "re.Match",
            Py::Func(_) => "function",
            Py::BoundMethod(_) => "builtin_function_or_method",
            Py::Instance(_) => "object",
            Py::Module(_) => "module",
            Py::Date(_) => "date",
            Py::DateTime(_) => "datetime",
            Py::TimeDelta(_) => "timedelta",
            Py::Kwargs(_) => "dict",
            Py::Exc(_) => "Exception",
            Py::Bytes(_) => "bytes",
            Py::Ellipsis => "ellipsis",
            Py::NotImplementedType => "NotImplementedType",
        }
    }
}

// ---------------------------------------------------------------- PyKey
#[derive(Clone)]
pub struct PyKey(pub Py);

impl PartialEq for PyKey {
    fn eq(&self, other: &Self) -> bool { py_eq(&self.0, &other.0) }
}
impl Eq for PyKey {}

impl Hash for PyKey {
    fn hash<H: Hasher>(&self, state: &mut H) { py_hash_into(&self.0, state) }
}

fn py_hash_into<H: Hasher>(v: &Py, state: &mut H) {
    match v {
        Py::None => { 0u8.hash(state); }
        Py::Bool(b) => { 1u8.hash(state); (*b as i64).hash(state); }
        Py::Int(i) => { 2u8.hash(state); i.hash(state); }
        Py::Float(f) => {
            3u8.hash(state);
            if f.fract() == 0.0 && f.is_finite() && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                (*f as i64).hash(state);
            } else if f.is_nan() {
                0u32.hash(state);
            } else {
                f.to_bits().hash(state);
            }
        }
        Py::Str(s) => { 4u8.hash(state); s.hash(state); }
        Py::Tuple(t) => { 5u8.hash(state); for x in t.iter() { py_hash_into(x, state); } }
        Py::FrozenSet(s) => {
            6u8.hash(state);
            let mut hs: Vec<u64> = s.iter().map(|k| { let mut h = DefaultHasher::new(); k.hash(&mut h); h.finish() }).collect();
            hs.sort();
            for h in hs { h.hash(state); }
        }
        Py::Bytes(b) => { 7u8.hash(state); b.hash(state); }
        _ => { 255u8.hash(state); (v as *const Py as usize).hash(state); }
    }
}
pub fn py_hash(v: &Py) -> i64 {
    let mut h = DefaultHasher::new();
    py_hash_into(v, &mut h);
    h.finish() as i64
}

// ---------------------------------------------------------------- truthiness
pub fn py_truthy(v: &Py) -> bool {
    match v {
        Py::None => false,
        Py::Bool(b) => *b,
        Py::Int(i) => *i != 0,
        Py::Float(f) => *f != 0.0,
        Py::Str(s) => !s.is_empty(),
        Py::List(l) => !l.borrow().is_empty(),
        Py::Tuple(t) => !t.is_empty(),
        Py::Dict(d) => !d.borrow().is_empty(),
        Py::Set(s) => !s.borrow().is_empty(),
        Py::FrozenSet(s) => !s.is_empty(),
        Py::Range(r) => r.len() != 0,
        Py::Bytes(b) => !b.is_empty(),
        Py::Exc(_) => true,
        Py::Instance(inst) => {
            // __bool__ support: instances may carry an explicit __bool__ attr result
            if let Some(b) = inst.attrs.borrow().get("__bool_value__") {
                return b.is_truthy();
            }
            true
        }
        Py::NotImplementedType => true,
        _ => true,
    }
}

// ---------------------------------------------------------------- equality / ordering
pub fn py_eq(a: &Py, b: &Py) -> bool {
    match (a, b) {
        (Py::None, Py::None) => true,
        (Py::Bool(x), Py::Bool(y)) => x == y,
        (Py::Bool(x), Py::Int(y)) | (Py::Int(y), Py::Bool(x)) => (*x as i64) == *y,
        (Py::Bool(x), Py::Float(y)) | (Py::Float(y), Py::Bool(x)) => (*x as i64 as f64) == *y,
        (Py::Int(x), Py::Int(y)) => x == y,
        (Py::Int(x), Py::Float(y)) | (Py::Float(y), Py::Int(x)) => (*x as f64) == *y,
        (Py::Float(x), Py::Float(y)) => x == y,
        (Py::Str(x), Py::Str(y)) => x == y,
        (Py::List(x), Py::List(y)) => {
            let (x, y) = (x.borrow(), y.borrow());
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| py_eq(p, q))
        }
        (Py::Tuple(x), Py::Tuple(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(p, q)| py_eq(p, q))
        }
        (Py::Dict(x), Py::Dict(y)) => {
            let (x, y) = (x.borrow(), y.borrow());
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).map_or(false, |w| py_eq(v, w)))
        }
        (Py::Set(x), Py::Set(y)) => {
            let (x, y) = (x.borrow(), y.borrow());
            x.len() == y.len() && x.iter().all(|k| y.contains(k))
        }
        (Py::FrozenSet(x), Py::FrozenSet(y)) => {
            x.len() == y.len() && x.iter().all(|k| y.contains(k))
        }
        (Py::Set(x), Py::FrozenSet(y)) | (Py::FrozenSet(y), Py::Set(x)) => {
            let x = x.borrow();
            x.len() == y.len() && x.iter().all(|k| y.contains(k))
        }
        (Py::Range(x), Py::Range(y)) => x.start == y.start && x.stop == y.stop && x.step == y.step,
        (Py::Bytes(x), Py::Bytes(y)) => x == y,
        (Py::Func(x), Py::Func(y)) => Rc::ptr_eq(x, y),
        (Py::Instance(x), Py::Instance(y)) => Rc::ptr_eq(x, y),
        (Py::Date(x), Py::Date(y)) => x == y,
        (Py::DateTime(x), Py::DateTime(y)) => x == y,
        (Py::TimeDelta(x), Py::TimeDelta(y)) => x == y,
        (Py::Pattern(x), Py::Pattern(y)) => Rc::ptr_eq(x, y),
        _ => false,
    }
}

fn type_rank(v: &Py) -> u8 {
    match v {
        Py::None => 0, Py::Bool(_) | Py::Int(_) | Py::Float(_) => 1, Py::Str(_) => 2,
        Py::List(_) => 3, Py::Tuple(_) => 4, Py::Dict(_) => 5, Py::Set(_) | Py::FrozenSet(_) => 6,
        Py::Date(_) => 7, Py::DateTime(_) => 8, Py::TimeDelta(_) => 9, _ => 100,
    }
}

/// Total ordering used for sort; matches Python for homogeneous comparable values.
pub fn py_cmp_total(a: &Py, b: &Py) -> Ordering {
    match (a, b) {
        (Py::None, Py::None) => Ordering::Equal,
        (Py::Bool(_), _) | (Py::Int(_), _) | (Py::Float(_), _)
            if matches!(b, Py::Bool(_) | Py::Int(_) | Py::Float(_)) => {
            let (x, y) = (a.as_f64().unwrap(), b.as_f64().unwrap());
            // exact int comparison to avoid float rounding
            match (a.as_i64(), b.as_i64()) {
                (Some(i), Some(j)) => i.cmp(&j),
                _ => x.partial_cmp(&y).unwrap_or(Ordering::Equal),
            }
        }
        (Py::Str(x), Py::Str(y)) => x.cmp(y),
        (Py::List(x), Py::List(y)) => {
            let (x, y) = (x.borrow().clone(), b_list(b));
            cmp_seq(&x, &y)
        }
        (Py::Tuple(x), Py::Tuple(y)) => cmp_seq(x, y),
        (Py::Date(x), Py::Date(y)) => x.cmp_ord(y),
        (Py::DateTime(x), Py::DateTime(y)) => x.cmp_ord(y),
        (Py::TimeDelta(x), Py::TimeDelta(y)) => x.cmp_ord(y),
        _ => {
            let (ra, rb) = (type_rank(a), type_rank(b));
            if ra != rb { ra.cmp(&rb) } else { py_repr(a).cmp(&py_repr(b)) }
        }
    }
}
fn b_list(b: &Py) -> Vec<Py> {
    if let Py::List(l) = b { l.borrow().clone() } else { vec![] }
}
fn cmp_seq(x: &[Py], y: &[Py]) -> Ordering {
    for (p, q) in x.iter().zip(y.iter()) {
        let c = py_cmp_total(p, q);
        if c != Ordering::Equal { return c; }
    }
    x.len().cmp(&y.len())
}

/// Python comparison; errors like CPython for incomparable types.
pub fn py_cmp(a: &Py, b: &Py, op: &str) -> Result<Ordering, PyExc> {
    let both_num = matches!(a, Py::Bool(_) | Py::Int(_) | Py::Float(_)) && matches!(b, Py::Bool(_) | Py::Int(_) | Py::Float(_));
    let ok = both_num
        || matches!((a, b), (Py::Str(_), Py::Str(_)) | (Py::List(_), Py::List(_)) | (Py::Tuple(_), Py::Tuple(_))
            | (Py::Date(_), Py::Date(_)) | (Py::DateTime(_), Py::DateTime(_)) | (Py::TimeDelta(_), Py::TimeDelta(_))
            | (Py::Set(_), Py::Set(_)) | (Py::FrozenSet(_), Py::FrozenSet(_)) | (Py::Set(_), Py::FrozenSet(_)) | (Py::FrozenSet(_), Py::Set(_)));
    if !ok && !matches!(op, "==" | "!=") {
        return Err(type_err(&format!("'{}' not supported between instances of '{}' and '{}'", op, a.type_name(), b.type_name())));
    }
    Ok(py_cmp_total(a, b))
}

// ---------------------------------------------------------------- str / repr
pub fn py_str_of(v: &Py) -> String {
    match v {
        Py::None => "None".to_string(),
        Py::Bool(b) => if *b { "True".into() } else { "False".into() },
        Py::Int(i) => i.to_string(),
        Py::Float(f) => py_float_repr(*f),
        Py::Str(s) => s.to_string(),
        Py::Bytes(b) => format!("b{}", repr_bytes(b)),
        other => py_repr(other),
    }
}

pub fn py_float_repr(f: f64) -> String {
    if f.is_nan() { return "nan".into(); }
    if f.is_infinite() { return if f > 0.0 { "inf".into() } else { "-inf".into() }; }
    let mut s = format!("{}", f);
    if !s.contains('.') && !s.contains('e') && !s.contains("inf") && !s.contains("nan") {
        s.push_str(".0");
    }
    // Python uses repr-style shortest; Rust {} is shortest round-trip too.
    s
}

fn repr_bytes(b: &[u8]) -> String {
    let mut s = String::from("'");
    for &c in b {
        match c {
            b'\'' => s.push_str("\\'"),
            b'\\' => s.push_str("\\\\"),
            0x20..=0x7e => s.push(c as char),
            _ => s.push_str(&format!("\\x{:02x}", c)),
        }
    }
    s.push('\'');
    s
}

pub fn py_repr(v: &Py) -> String {
    match v {
        Py::Str(s) => repr_str(s),
        Py::List(l) => {
            let parts: Vec<String> = l.borrow().iter().map(py_repr).collect();
            format!("[{}]", parts.join(", "))
        }
        Py::Tuple(t) => {
            let parts: Vec<String> = t.iter().map(py_repr).collect();
            if parts.len() == 1 { format!("({},)", parts[0]) } else { format!("({})", parts.join(", ")) }
        }
        Py::Dict(d) => {
            let parts: Vec<String> = d.borrow().iter().map(|(k, v)| format!("{}: {}", py_repr(&k.0), py_repr(v))).collect();
            format!("{{{}}}", parts.join(", "))
        }
        Py::Set(s) => {
            let b = s.borrow();
            if b.is_empty() { return "set()".into(); }
            let parts: Vec<String> = b.iter().map(|k| py_repr(&k.0)).collect();
            format!("{{{}}}", parts.join(", "))
        }
        Py::FrozenSet(s) => {
            let parts: Vec<String> = s.iter().map(|k| py_repr(&k.0)).collect();
            format!("frozenset({{{}}})", parts.join(", "))
        }
        Py::Range(r) => {
            if r.step == 1 { format!("range({}, {})", r.start, r.stop) } else { format!("range({}, {}, {})", r.start, r.stop, r.step) }
        }
        Py::Func(f) => format!("<function {}>", f.name),
        Py::BoundMethod(_) => "<bound method>".into(),
        Py::Pattern(p) => format!("re.compile('{}')", p.source.replace('\'', "\\'")),
        Py::Match(_) => "<re.Match object>".into(),
        Py::Instance(i) => format!("<{} object>", i.class_name),
        Py::Module(m) => format!("<module '{}'>", m.name),
        Py::Date(d) => d.repr(),
        Py::DateTime(d) => d.repr(),
        Py::TimeDelta(d) => d.repr(),
        Py::Exc(e) => format!("{}('{}')", e.class_name, e.message),
        Py::Kwargs(k) => {
            let parts: Vec<String> = k.iter().map(|(n, v)| format!("'{}': {}", n, py_repr(v))).collect();
            format!("{{{}}}", parts.join(", "))
        }
        Py::Ellipsis => "Ellipsis".into(),
        Py::NotImplementedType => "NotImplemented".into(),
        other => py_str_of(other),
    }
}

pub fn repr_str(s: &str) -> String {
    let has_sq = s.contains('\'');
    let has_dq = s.contains('"');
    let (quote, escape_quote) = if has_sq && !has_dq { ('"', false) } else { ('\'', has_sq) };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for c in s.chars() {
        match c {
            '\'' if escape_quote => out.push_str("\\'"),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push(quote);
    out
}

// ---------------------------------------------------------------- arithmetic
pub fn py_add(a: &Py, b: &Py) -> PyResult {
    match (a, b) {
        (Py::Int(x), Py::Int(y)) => Ok(Py::Int(x.wrapping_add(*y))),
        (Py::Bool(_), _) | (_, Py::Bool(_)) if a.as_i64().is_some() && b.as_i64().is_some() && !matches!(a, Py::Str(_)) && !matches!(b, Py::Str(_)) => {
            Ok(Py::Int(a.as_i64().unwrap() + b.as_i64().unwrap()))
        }
        (Py::Float(_), _) | (_, Py::Float(_)) if a.as_f64().is_some() && b.as_f64().is_some() => {
            Ok(Py::Float(a.as_f64().unwrap() + b.as_f64().unwrap()))
        }
        (Py::Str(x), Py::Str(y)) => Ok(py_string(format!("{}{}", x, y))),
        (Py::List(x), Py::List(y)) => {
            let mut v = x.borrow().clone();
            v.extend(y.borrow().iter().cloned());
            Ok(py_list(v))
        }
        (Py::Tuple(x), Py::Tuple(y)) => {
            let mut v = (**x).clone();
            v.extend(y.iter().cloned());
            Ok(py_tuple(v))
        }
        (Py::Bytes(x), Py::Bytes(y)) => {
            let mut v = (**x).clone();
            v.extend(y.iter());
            Ok(Py::Bytes(Rc::new(v)))
        }
        (Py::Date(d), Py::TimeDelta(t)) => Ok(Py::Date(Rc::new(d.add_days(t.days)))),
        (Py::TimeDelta(t), Py::Date(d)) => Ok(Py::Date(Rc::new(d.add_days(t.days)))),
        (Py::DateTime(d), Py::TimeDelta(t)) | (Py::TimeDelta(t), Py::DateTime(d)) => Ok(Py::DateTime(Rc::new(d.add(t)))),
        (Py::TimeDelta(x), Py::TimeDelta(y)) => Ok(Py::TimeDelta(Rc::new(x.add(y)))),
        _ => Err(type_err(&format!("unsupported operand type(s) for +: '{}' and '{}'", a.type_name(), b.type_name()))),
    }
}

pub fn py_sub(a: &Py, b: &Py) -> PyResult {
    match (a, b) {
        _ if a.as_i64().is_some() && b.as_i64().is_some() => Ok(Py::Int(a.as_i64().unwrap() - b.as_i64().unwrap())),
        _ if a.as_f64().is_some() && b.as_f64().is_some() => Ok(Py::Float(a.as_f64().unwrap() - b.as_f64().unwrap())),
        (Py::Date(d), Py::TimeDelta(t)) => Ok(Py::Date(Rc::new(d.add_days(-t.days)))),
        (Py::Date(x), Py::Date(y)) => Ok(Py::TimeDelta(Rc::new(PyTimeDelta { days: x.days() - y.days(), seconds: 0 }))),
        (Py::DateTime(d), Py::TimeDelta(t)) => Ok(Py::DateTime(Rc::new(d.add(&PyTimeDelta { days: -t.days, seconds: -t.seconds })))),
        (Py::DateTime(x), Py::DateTime(y)) => Ok(Py::TimeDelta(Rc::new(x.sub(y)))),
        (Py::TimeDelta(x), Py::TimeDelta(y)) => Ok(Py::TimeDelta(Rc::new(x.add(&PyTimeDelta { days: -y.days, seconds: -y.seconds })))),
        (Py::Set(_), _) | (Py::FrozenSet(_), _) if matches!(b, Py::Set(_) | Py::FrozenSet(_)) => {
            let mut s = IndexSet::new();
            let xs: Vec<PyKey> = match a { Py::Set(v) => v.borrow().iter().cloned().collect(), Py::FrozenSet(v) => v.iter().cloned().collect(), _ => vec![] };
            for k in xs {
                let present = match b { Py::Set(v) => v.borrow().contains(&k), Py::FrozenSet(v) => v.contains(&k), _ => false };
                if !present { s.insert(k); }
            }
            Ok(Py::Set(Rc::new(RefCell::new(s))))
        }
        _ => Err(type_err(&format!("unsupported operand type(s) for -: '{}' and '{}'", a.type_name(), b.type_name()))),
    }
}

pub fn py_mul(a: &Py, b: &Py) -> PyResult {
    match (a, b) {
        _ if a.as_i64().is_some() && b.as_i64().is_some() => Ok(Py::Int(a.as_i64().unwrap() * b.as_i64().unwrap())),
        _ if a.as_f64().is_some() && b.as_f64().is_some() => Ok(Py::Float(a.as_f64().unwrap() * b.as_f64().unwrap())),
        (Py::Str(s), _) if b.as_i64().is_some() => Ok(py_string(s.repeat(b.as_i64().unwrap().max(0) as usize))),
        (_, Py::Str(s)) if a.as_i64().is_some() => Ok(py_string(s.repeat(a.as_i64().unwrap().max(0) as usize))),
        (Py::List(l), _) if b.as_i64().is_some() => {
            let n = b.as_i64().unwrap().max(0) as usize;
            let mut v = Vec::with_capacity(l.borrow().len() * n);
            for _ in 0..n { v.extend(l.borrow().iter().cloned()); }
            Ok(py_list(v))
        }
        (_, Py::List(l)) if a.as_i64().is_some() => {
            let n = a.as_i64().unwrap().max(0) as usize;
            let mut v = Vec::with_capacity(l.borrow().len() * n);
            for _ in 0..n { v.extend(l.borrow().iter().cloned()); }
            Ok(py_list(v))
        }
        (Py::Tuple(t), _) if b.as_i64().is_some() => {
            let n = b.as_i64().unwrap().max(0) as usize;
            let mut v = Vec::with_capacity(t.len() * n);
            for _ in 0..n { v.extend(t.iter().cloned()); }
            Ok(py_tuple(v))
        }
        (_, Py::Tuple(t)) if a.as_i64().is_some() => {
            let n = a.as_i64().unwrap().max(0) as usize;
            let mut v = Vec::with_capacity(t.len() * n);
            for _ in 0..n { v.extend(t.iter().cloned()); }
            Ok(py_tuple(v))
        }
        _ => Err(type_err(&format!("unsupported operand type(s) for *: '{}' and '{}'", a.type_name(), b.type_name()))),
    }
}

pub fn py_truediv(a: &Py, b: &Py) -> PyResult {
    let (x, y) = (a.as_f64(), b.as_f64());
    match (x, y) {
        (Some(x), Some(y)) => {
            if y == 0.0 { Err(zero_div()) } else { Ok(Py::Float(x / y)) }
        }
        _ => Err(type_err(&format!("unsupported operand type(s) for /: '{}' and '{}'", a.type_name(), b.type_name()))),
    }
}

pub fn py_floordiv(a: &Py, b: &Py) -> PyResult {
    match (a.as_i64(), b.as_i64()) {
        (Some(x), Some(y)) => {
            if y == 0 { return Err(zero_div()); }
            let q = x / y;
            let r = x % y;
            Ok(Py::Int(if r != 0 && ((r < 0) != (y < 0)) { q - 1 } else { q }))
        }
        _ => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => {
                if y == 0.0 { return Err(zero_div()); }
                Ok(Py::Float((x / y).floor()))
            }
            _ => Err(type_err(&format!("unsupported operand type(s) for //: '{}' and '{}'", a.type_name(), b.type_name()))),
        },
    }
}

pub fn py_mod(a: &Py, b: &Py) -> PyResult {
    if let Py::Str(s) = a {
        return crate::pystr::percent_format(s, b);
    }
    match (a.as_i64(), b.as_i64()) {
        (Some(x), Some(y)) => {
            if y == 0 { return Err(zero_div()); }
            let r = x % y;
            Ok(Py::Int(if r != 0 && ((r < 0) != (y < 0)) { r + y } else { r }))
        }
        _ => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => {
                if y == 0.0 { return Err(zero_div()); }
                let r = x % y;
                Ok(Py::Float(if r != 0.0 && ((r < 0.0) != (y < 0.0)) { r + y } else { r }))
            }
            _ => Err(type_err(&format!("unsupported operand type(s) for %: '{}' and '{}'", a.type_name(), b.type_name()))),
        },
    }
}

pub fn py_pow(a: &Py, b: &Py) -> PyResult {
    match (a.as_i64(), b.as_i64()) {
        (Some(x), Some(y)) if y >= 0 => {
            match x.checked_pow(y as u32) {
                Some(v) => Ok(Py::Int(v)),
                None => Ok(Py::Float((x as f64).powf(y as f64))),
            }
        }
        _ => match (a.as_f64(), b.as_f64()) {
            (Some(x), Some(y)) => Ok(Py::Float(x.powf(y))),
            _ => Err(type_err("unsupported operand type(s) for **")),
        },
    }
}

pub fn py_neg(a: &Py) -> PyResult {
    match a {
        Py::Int(i) => Ok(Py::Int(-i)),
        Py::Bool(b) => Ok(Py::Int(if *b { -1 } else { 0 })),
        Py::Float(f) => Ok(Py::Float(-f)),
        Py::TimeDelta(t) => Ok(Py::TimeDelta(Rc::new(PyTimeDelta { days: -t.days, seconds: -t.seconds }))),
        _ => Err(type_err("bad operand type for unary -")),
    }
}

// bitwise ops (Python ints)
pub fn py_bitand(a: &Py, b: &Py) -> PyResult {
    match (a, b) {
        (Py::Set(_), _) | (Py::FrozenSet(_), _) if matches!(b, Py::Set(_) | Py::FrozenSet(_)) => {
            let mut s = IndexSet::new();
            let xs: Vec<PyKey> = match a { Py::Set(v) => v.borrow().iter().cloned().collect(), Py::FrozenSet(v) => v.iter().cloned().collect(), _ => vec![] };
            let has = |k: &PyKey| match b { Py::Set(v) => v.borrow().contains(k), Py::FrozenSet(v) => v.contains(k), _ => false };
            for k in xs { if has(&k) { s.insert(k); } }
            Ok(Py::Set(Rc::new(RefCell::new(s))))
        }
        _ if a.as_i64().is_some() && b.as_i64().is_some() => Ok(Py::Int(a.as_i64().unwrap() & b.as_i64().unwrap())),
        _ => Err(type_err("unsupported operand type(s) for &")),
    }
}
pub fn py_bitor(a: &Py, b: &Py) -> PyResult {
    match (a, b) {
        (Py::Set(_), _) | (Py::FrozenSet(_), _) if matches!(b, Py::Set(_) | Py::FrozenSet(_)) => {
            let mut s = IndexSet::new();
            let mut add = |v: &Py| match v {
                Py::Set(x) => { for k in x.borrow().iter() { s.insert(k.clone()); } }
                Py::FrozenSet(x) => { for k in x.iter() { s.insert(k.clone()); } }
                _ => {}
            };
            add(a); add(b);
            Ok(Py::Set(Rc::new(RefCell::new(s))))
        }
        (Py::Dict(x), Py::Dict(y)) => {
            let mut m = x.borrow().clone();
            for (k, v) in y.borrow().iter() { m.insert(k.clone(), v.clone()); }
            Ok(Py::Dict(Rc::new(RefCell::new(m))))
        }
        _ if a.as_i64().is_some() && b.as_i64().is_some() => Ok(Py::Int(a.as_i64().unwrap() | b.as_i64().unwrap())),
        _ => Err(type_err("unsupported operand type(s) for |")),
    }
}
pub fn py_bitxor(a: &Py, b: &Py) -> PyResult {
    if a.as_i64().is_some() && b.as_i64().is_some() { Ok(Py::Int(a.as_i64().unwrap() ^ b.as_i64().unwrap())) } else { Err(type_err("unsupported operand type(s) for ^")) }
}
pub fn py_invert(a: &Py) -> PyResult {
    if let Some(i) = a.as_i64() { Ok(Py::Int(!i)) } else { Err(type_err("bad operand type for unary ~")) }
}
pub fn py_lshift(a: &Py, b: &Py) -> PyResult {
    match (a.as_i64(), b.as_i64()) { (Some(x), Some(y)) => Ok(Py::Int(x.wrapping_shl(y as u32))), _ => Err(type_err("unsupported operand type(s) for <<")) }
}
pub fn py_rshift(a: &Py, b: &Py) -> PyResult {
    match (a.as_i64(), b.as_i64()) { (Some(x), Some(y)) => Ok(Py::Int(x.wrapping_shr(y as u32))), _ => Err(type_err("unsupported operand type(s) for >>")) }
}

// ---------------------------------------------------------------- membership / iteration
pub fn py_contains(container: &Py, item: &Py) -> bool {
    match container {
        Py::Str(s) => item.as_str().map_or(false, |sub| s.contains(sub)),
        Py::List(l) => l.borrow().iter().any(|x| py_eq(x, item)),
        Py::Tuple(t) => t.iter().any(|x| py_eq(x, item)),
        Py::Dict(d) => d.borrow().contains_key(&PyKey(item.clone())),
        Py::Set(s) => s.borrow().contains(&PyKey(item.clone())),
        Py::FrozenSet(s) => s.contains(&PyKey(item.clone())),
        Py::Range(r) => item.as_i64().map_or(false, |i| {
            if r.step > 0 { i >= r.start && i < r.stop && (i - r.start) % r.step == 0 }
            else if r.step < 0 { i <= r.start && i > r.stop && (r.start - i) % (-r.step) == 0 }
            else { false }
        }),
        Py::Bytes(b) => match item {
            Py::Int(i) => b.contains(&(*i as u8)),
            Py::Bytes(sub) => sub.is_empty() || b.windows(sub.len()).any(|w| w == sub.as_slice()),
            _ => false,
        },
        _ => false,
    }
}

pub fn py_iter(v: &Py) -> Result<Vec<Py>, PyExc> {
    match v {
        Py::Str(s) => Ok(s.chars().map(|c| py_string(c.to_string())).collect()),
        Py::List(l) => Ok(l.borrow().clone()),
        Py::Tuple(t) => Ok((**t).clone()),
        Py::Dict(d) => Ok(d.borrow().keys().map(|k| k.0.clone()).collect()),
        Py::Set(s) => Ok(s.borrow().iter().map(|k| k.0.clone()).collect()),
        Py::FrozenSet(s) => Ok(s.iter().map(|k| k.0.clone()).collect()),
        Py::Range(r) => Ok(r.values()),
        Py::Bytes(b) => Ok(b.iter().map(|&x| Py::Int(x as i64)).collect()),
        Py::Kwargs(k) => Ok(k.iter().map(|(n, _)| py_str(n)).collect()),
        _ => Err(type_err(&format!("'{}' object is not iterable", v.type_name()))),
    }
}

pub fn py_len(v: &Py) -> Result<i64, PyExc> {
    match v {
        Py::Str(s) => Ok(s.chars().count() as i64),
        Py::List(l) => Ok(l.borrow().len() as i64),
        Py::Tuple(t) => Ok(t.len() as i64),
        Py::Dict(d) => Ok(d.borrow().len() as i64),
        Py::Set(s) => Ok(s.borrow().len() as i64),
        Py::FrozenSet(s) => Ok(s.len() as i64),
        Py::Range(r) => Ok(r.len()),
        Py::Bytes(b) => Ok(b.len() as i64),
        _ => Err(type_err(&format!("object of type '{}' has no len()", v.type_name()))),
    }
}

// ---------------------------------------------------------------- indexing
fn norm_index(i: i64, len: i64) -> Option<usize> {
    let idx = if i < 0 { len + i } else { i };
    if idx >= 0 && idx < len { Some(idx as usize) } else { None }
}

/// Slice indices in Python semantics. Returns (start, stop, step) as char/elem positions.
pub fn slice_indices(len: i64, start: Option<i64>, stop: Option<i64>, step: i64) -> Vec<i64> {
    let mut out = Vec::new();
    if step == 0 { return out; }
    let (mut lo, mut hi) = if step > 0 { (0i64, len) } else { (-1i64, -len - 1) };
    let clamp = |v: i64| -> i64 {
        if step > 0 { v.max(0).min(len) } else { v.max(-1).min(len - 1) }
    };
    let s = match start { None => if step > 0 { 0 } else { len - 1 }, Some(v) => clamp(if v < 0 { v + len } else { v }) };
    let e = match stop { None => if step > 0 { len } else { -1 }, Some(v) => clamp(if v < 0 { v + len } else { v }) };
    let _ = (&mut lo, &mut hi);
    let mut i = s;
    if step > 0 {
        while i < e { out.push(i); i += step; }
    } else {
        while i > e { out.push(i); i += step; }
    }
    out
}

pub fn py_getitem(v: &Py, key: &Py) -> PyResult {
    match v {
        Py::Str(s) => {
            let chars: Vec<char> = s.chars().collect();
            match key {
                Py::Int(_) | Py::Bool(_) => {
                    let i = key.as_i64().unwrap();
                    match norm_index(i, chars.len() as i64) {
                        Some(idx) => Ok(py_string(chars[idx].to_string())),
                        None => Err(py_err("IndexError", "string index out of range")),
                    }
                }
                Py::Tuple(_) => Err(type_err("string indices must be integers")),
                _ => Err(type_err("string indices must be integers")),
            }
        }
        Py::List(l) => {
            let b = l.borrow();
            let i = key.as_i64().ok_or_else(|| type_err("list indices must be integers"))?;
            match norm_index(i, b.len() as i64) {
                Some(idx) => Ok(b[idx].clone()),
                None => Err(index_err()),
            }
        }
        Py::Tuple(t) => {
            let i = key.as_i64().ok_or_else(|| type_err("tuple indices must be integers"))?;
            match norm_index(i, t.len() as i64) {
                Some(idx) => Ok(t[idx].clone()),
                None => Err(py_err("IndexError", "tuple index out of range")),
            }
        }
        Py::Dict(d) => {
            let b = d.borrow();
            match b.get(&PyKey(key.clone())) {
                Some(v) => Ok(v.clone()),
                None => Err(key_err(key)),
            }
        }
        Py::Range(r) => {
            let i = key.as_i64().ok_or_else(|| type_err("range indices must be integers"))?;
            match norm_index(i, r.len()) {
                Some(idx) => Ok(Py::Int(r.start + idx as i64 * r.step)),
                None => Err(py_err("IndexError", "range index out of range")),
            }
        }
        Py::Bytes(bts) => {
            let i = key.as_i64().ok_or_else(|| type_err("bytes indices must be integers"))?;
            match norm_index(i, bts.len() as i64) {
                Some(idx) => Ok(Py::Int(bts[idx] as i64)),
                None => Err(py_err("IndexError", "bytes index out of range")),
            }
        }
        Py::Match(m) => {
            let i = key.as_i64().ok_or_else(|| type_err("match indices must be integers"))?;
            Ok(m.group(i))
        }
        _ => Err(type_err(&format!("'{}' object is not subscriptable", v.type_name()))),
    }
}

pub fn py_slice(v: &Py, start: &Py, stop: &Py, step: &Py) -> PyResult {
    let get_i = |p: &Py| -> Option<i64> {
        match p { Py::None => None, Py::Int(i) => Some(*i), Py::Bool(b) => Some(*b as i64), _ => None }
    };
    let st = get_i(step).unwrap_or(1);
    let (s, e) = (get_i(start), get_i(stop));
    match v {
        Py::Str(x) => {
            let chars: Vec<char> = x.chars().collect();
            let idx = slice_indices(chars.len() as i64, s, e, st);
            Ok(py_string(idx.iter().map(|&i| chars[i as usize]).collect()))
        }
        Py::List(l) => {
            let b = l.borrow();
            let idx = slice_indices(b.len() as i64, s, e, st);
            Ok(py_list(idx.iter().map(|&i| b[i as usize].clone()).collect()))
        }
        Py::Tuple(t) => {
            let idx = slice_indices(t.len() as i64, s, e, st);
            Ok(py_tuple(idx.iter().map(|&i| t[i as usize].clone()).collect()))
        }
        Py::Range(r) => {
            let vals = r.values();
            let idx = slice_indices(vals.len() as i64, s, e, st);
            Ok(py_list(idx.iter().map(|&i| vals[i as usize].clone()).collect()))
        }
        Py::Bytes(bts) => {
            let idx = slice_indices(bts.len() as i64, s, e, st);
            Ok(Py::Bytes(Rc::new(idx.iter().map(|&i| bts[i as usize]).collect())))
        }
        _ => Err(type_err(&format!("'{}' object is not subscriptable", v.type_name()))),
    }
}

pub fn py_setitem(v: &Py, key: &Py, val: Py) -> Result<(), PyExc> {
    match v {
        Py::List(l) => {
            let mut b = l.borrow_mut();
            let i = key.as_i64().ok_or_else(|| type_err("list indices must be integers"))?;
            match norm_index(i, b.len() as i64) {
                Some(idx) => { b[idx] = val; Ok(()) }
                None => Err(index_err()),
            }
        }
        Py::Dict(d) => {
            d.borrow_mut().insert(PyKey(key.clone()), val);
            Ok(())
        }
        Py::Instance(i) => {
            if let Some(name) = key.as_str() {
                i.attrs.borrow_mut().insert(name.to_string(), val);
                Ok(())
            } else { Err(type_err("instance keys must be strings")) }
        }
        _ => Err(type_err(&format!("'{}' object does not support item assignment", v.type_name()))),
    }
}

pub fn py_delitem(v: &Py, key: &Py) -> Result<(), PyExc> {
    match v {
        Py::Dict(d) => {
            let mut b = d.borrow_mut();
            if b.shift_remove(&PyKey(key.clone())).is_some() { Ok(()) } else { Err(key_err(key)) }
        }
        Py::List(l) => {
            let mut b = l.borrow_mut();
            let i = key.as_i64().ok_or_else(|| type_err("list indices must be integers"))?;
            match norm_index(i, b.len() as i64) {
                Some(idx) => { b.remove(idx); Ok(()) }
                None => Err(index_err()),
            }
        }
        _ => Err(type_err("does not support item deletion")),
    }
}

#[cfg(test)]
mod slice_tests {
    use super::*;
    #[test] fn reverse_slices_stay_in_range() {
        assert_eq!(slice_indices(1,None,None,-1),vec![0]);
        assert_eq!(slice_indices(3,None,None,-1),vec![2,1,0]);
        assert_eq!(slice_indices(3,Some(-8),None,-1),Vec::<i64>::new());
        assert_eq!(slice_indices(0,None,None,-1),Vec::<i64>::new());
    }
}

/// Python list slice assignment, including in-place alias preservation.
pub fn py_setslice(v:&Py,start:&Py,stop:&Py,step:&Py,replacement:&Py)->Result<(),PyExc>{
 let items=py_iter(replacement)?;
 let list=match v { Py::List(l)=>l, _=>return Err(type_err("slice assignment requires list")) };
 let mut b=list.borrow_mut(); let len=b.len() as i64;
 let st=step.as_i64().unwrap_or(1); if st==0{return Err(value_err("slice step cannot be zero"));}
 if st==1 {
  let bound=|x:&Py,default:i64| {let n=x.as_i64().unwrap_or(default); (if n<0 {n+len}else{n}).clamp(0,len) as usize};
  let a=bound(start,0);let z=bound(stop,len).max(a);b.splice(a..z,items);
 }else{
  let indices=slice_indices(len,start.as_i64(),stop.as_i64(),st);
  if indices.len()!=items.len(){return Err(value_err("extended slice assignment size mismatch"));}
  for (i,x) in indices.into_iter().zip(items){b[i as usize]=x;}
 } Ok(())
}
#[cfg(test)] mod slice_assignment_tests {
 use super::*;
 #[test] fn full_slice_replaces_not_nests(){ let a=py_list(vec![Py::Int(1)]);let alias=a.clone();py_setslice(&a,&Py::None,&Py::None,&Py::None,&py_list(vec![Py::Int(2),Py::Int(3)])).unwrap();assert!(py_eq(&alias,&py_list(vec![Py::Int(2),Py::Int(3)]))); }
}

/// Identity follows reference storage across argument clones.
pub fn py_identity(v:&Py)->i64 { match v {
 Py::Str(x)=>Rc::as_ptr(x) as usize as i64,
 Py::List(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Tuple(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Dict(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Set(x)=>Rc::as_ptr(x) as usize as i64,
 Py::FrozenSet(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Range(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Pattern(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Match(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Func(x)=>Rc::as_ptr(x) as usize as i64,
 Py::BoundMethod(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Instance(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Module(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Date(x)=>Rc::as_ptr(x) as usize as i64,
 Py::DateTime(x)=>Rc::as_ptr(x) as usize as i64,
 Py::TimeDelta(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Kwargs(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Exc(x)=>Rc::as_ptr(x) as usize as i64,
 Py::Bytes(x)=>Rc::as_ptr(x) as usize as i64,
 Py::None=>1,Py::Bool(false)=>2,Py::Bool(true)=>3,Py::Int(n)=>n.wrapping_mul(16).wrapping_add(4),Py::Float(n)=>n.to_bits() as i64,_=>5,}}

pub fn py_delslice(v:&Py,start:&Py,stop:&Py,step:&Py)->Result<(),PyExc>{
 let list=match v{Py::List(x)=>x,_=>return Err(type_err("slice deletion requires list"))};
 let st=step.as_i64().unwrap_or(1);if st==0{return Err(value_err("slice step cannot be zero"));}
 let mut b=list.borrow_mut();let mut indices=slice_indices(b.len() as i64,start.as_i64(),stop.as_i64(),st);
 indices.sort_unstable();for i in indices.into_iter().rev(){b.remove(i as usize);}Ok(())
}
